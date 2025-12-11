# ⚡ Port Your Unity Game to Lunaris in 10 Minutes

A fast, practical guide to get your Unity project running in Lunaris.

---

## ⏱️ Timeline

| Minute | Task |
|--------|------|
| 0-2 | Export assets from Unity |
| 2-4 | Create Lunaris project |
| 4-6 | Import assets |
| 6-8 | Convert main scripts |
| 8-10 | Test and run |

---

## Minute 0-2: Export from Unity

### 1. Export 3D Models
```
Unity: Assets → Right-click folder → Export Package
Or: File → Export → FBX
```

Save as `.fbx` or `.gltf` (preferred).

### 2. Copy Textures
```bash
# Copy your textures folder directly
cp -r Assets/Textures ./exports/
```

Keep `.png`, `.jpg`, `.hdr` files.

### 3. Export Audio
```bash
cp -r Assets/Audio ./exports/
```

Keep `.wav`, `.ogg`, `.mp3` files.

### 4. Note Your Scene Hierarchy
Take a screenshot of your Hierarchy panel - you'll recreate this.

---

## Minute 2-4: Create Lunaris Project

```bash
# Install Lunaris (if not already)
cargo install lunaris-cli

# Create new project
lunaris new my_ported_game
cd my_ported_game
```

Your project structure:
```
my_ported_game/
├── assets/          ← Put your exports here
├── src/
│   └── lib.rs       ← Your game code
├── Cargo.toml
└── lunaris.toml
```

---

## Minute 4-6: Import Assets

### 1. Copy Assets
```bash
# Copy all exports to assets folder
cp -r ../exports/* assets/
```

### 2. Auto-Import
```bash
# Lunaris auto-processes everything
lunaris import assets/
```

This automatically:
- ✅ Converts FBX → optimized meshes
- ✅ Compresses textures (BC7/ASTC)
- ✅ Generates LODs for Nanite
- ✅ Creates asset metadata

---

## Minute 6-8: Convert Scripts

### Your Unity Script

```csharp
// PlayerController.cs
using UnityEngine;

public class PlayerController : MonoBehaviour
{
    public float speed = 5f;
    public float jumpForce = 10f;
    private Rigidbody rb;
    private bool isGrounded;

    void Start()
    {
        rb = GetComponent<Rigidbody>();
    }

    void Update()
    {
        // Movement
        float h = Input.GetAxis("Horizontal");
        float v = Input.GetAxis("Vertical");
        Vector3 move = new Vector3(h, 0, v) * speed;
        rb.velocity = new Vector3(move.x, rb.velocity.y, move.z);

        // Jump
        if (Input.GetButtonDown("Jump") && isGrounded)
        {
            rb.AddForce(Vector3.up * jumpForce, ForceMode.Impulse);
        }
    }

    void OnCollisionEnter(Collision col)
    {
        if (col.gameObject.CompareTag("Ground"))
            isGrounded = true;
    }

    void OnCollisionExit(Collision col)
    {
        if (col.gameObject.CompareTag("Ground"))
            isGrounded = false;
    }
}
```

### Converted to Lunaris

```rust
// src/player.rs
use lunaris::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
}

#[derive(Component, Default)]
pub struct Grounded(pub bool);

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 5.0,
            jump_force: 10.0,
        }
    }
}

// Movement system (replaces Update)
pub fn player_movement(
    input: Res<Input>,
    mut query: Query<(&Player, &mut RigidBody)>,
) {
    for (player, mut rb) in query.iter_mut() {
        let h = input.axis("Horizontal");
        let v = input.axis("Vertical");
        
        let move_vec = Vec3::new(h, 0.0, v) * player.speed;
        rb.velocity.x = move_vec.x;
        rb.velocity.z = move_vec.z;
    }
}

// Jump system
pub fn player_jump(
    input: Res<Input>,
    mut query: Query<(&Player, &mut RigidBody, &Grounded)>,
) {
    for (player, mut rb, grounded) in query.iter_mut() {
        if input.just_pressed("Jump") && grounded.0 {
            rb.apply_impulse(Vec3::Y * player.jump_force);
        }
    }
}

// Ground detection (replaces OnCollision)
pub fn ground_check(
    mut events: EventReader<CollisionEvent>,
    mut query: Query<&mut Grounded>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b) => {
                if let Ok(mut grounded) = query.get_mut(*a) {
                    grounded.0 = true;
                }
            }
            CollisionEvent::Ended(a, b) => {
                if let Ok(mut grounded) = query.get_mut(*a) {
                    grounded.0 = false;
                }
            }
        }
    }
}

// Register in lib.rs
pub fn register(app: &mut App) {
    app.add_system(player_movement)
       .add_system(player_jump)
       .add_system(ground_check);
}
```

### Quick Conversion Cheatsheet

| Unity | Lunaris |
|-------|---------|
| `public float x` | `pub x: f32` |
| `GetComponent<T>()` | Query component |
| `Input.GetAxis()` | `input.axis()` |
| `Input.GetButtonDown()` | `input.just_pressed()` |
| `rb.AddForce()` | `rb.apply_impulse()` |
| `OnCollisionEnter` | `CollisionEvent::Started` |
| `CompareTag()` | Check for component |

---

## Minute 8-10: Test and Run

### 1. Update lib.rs

```rust
// src/lib.rs
mod player;

use lunaris::prelude::*;

pub fn build(app: &mut App) {
    // Register your systems
    player::register(app);
    
    // Spawn player
    app.add_startup_system(spawn_player);
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Player::default(),
        Grounded::default(),
        RigidBody::dynamic(),
        Collider::capsule(0.5, 1.8),
        Transform::from_xyz(0.0, 2.0, 0.0),
        assets.load::<Mesh>("player.glb"),
    ));
}
```

### 2. Run!

```bash
lunaris run
```

### 3. Open Editor (Optional)

```bash
lunaris editor
```

Drag your scene objects from the Asset Browser to recreate your hierarchy.

---

## 🎯 Common Conversions

### Prefabs

**Unity:**
```csharp
Instantiate(enemyPrefab, position, rotation);
```

**Lunaris:**
```rust
commands.spawn((
    EnemyBundle::default(),
    Transform::from_translation(position).with_rotation(rotation),
));
```

### Find Objects

**Unity:**
```csharp
var enemies = GameObject.FindGameObjectsWithTag("Enemy");
```

**Lunaris:**
```rust
for entity in query.iter::<&Enemy>() {
    // Each enemy
}
```

### Destroy

**Unity:**
```csharp
Destroy(gameObject);
```

**Lunaris:**
```rust
commands.despawn(entity);
```

### Singleton/Manager

**Unity:**
```csharp
GameManager.Instance.Score += 10;
```

**Lunaris:**
```rust
fn add_score(mut score: ResMut<Score>) {
    score.0 += 10;
}
```

---

## ✅ Checklist

- [ ] Exported models (FBX/GLTF)
- [ ] Exported textures (PNG/JPG)
- [ ] Exported audio (WAV/OGG)
- [ ] Created Lunaris project
- [ ] Imported assets
- [ ] Converted main player script
- [ ] Registered systems
- [ ] Spawned player entity
- [ ] Tested with `lunaris run`

---

## 🆘 Stuck?

| Problem | Solution |
|---------|----------|
| Assets not loading | Check paths in `assets/` folder |
| Script errors | Use `cargo check` for hints |
| Physics issues | Ensure `RigidBody` + `Collider` added |
| No input | Check input mapping in `lunaris.toml` |

---

## 🚀 Next Steps

1. **Convert remaining scripts** - Use the cheatsheet above
2. **Setup scenes** - Use the editor to build levels
3. **Add AI** - Explore Lunaris's cognitive NPCs
4. **Optimize** - Enable Nanite for high-poly models

---

**Congratulations! 🎉** You've ported your Unity game to Lunaris.

Now enjoy:
- ⚡ Better performance (Rust)
- 💰 No royalties (ever)
- 🔧 Full source access
- 🎮 All platforms supported
