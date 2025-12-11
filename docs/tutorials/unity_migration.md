# 🎮 Migrating from Unity to Lunaris

Welcome, Unity developer! This guide will help you transition smoothly to Lunaris Engine. You'll find familiar concepts with modern, Rust-powered improvements.

---

## 🌟 Why Switch?

| Unity | Lunaris |
|-------|---------|
| C# GC pauses | Rust zero-cost abstractions |
| 5% royalty (Pro) | **100% free, forever** |
| Closed source | Open source |
| Legacy ECS | Modern, built-in ECS |
| Separate HDRP | Lumen GI built-in |

---

## 📖 Concept Mapping

### Core Concepts

| Unity | Lunaris | Notes |
|-------|---------|-------|
| `GameObject` | `Entity` | Same concept, lighter weight |
| `MonoBehaviour` | `Component` | Data-only, no inheritance |
| `Update()` | `System` | Query-based, parallelizable |
| `ScriptableObject` | `Resource` | Type-safe assets |
| `Prefab` | `Prefab` | Same! Works identically |
| `Scene` | `Scene` | Same! With hot-reload |

### Rendering

| Unity | Lunaris |
|-------|---------|
| URP/HDRP | Unified renderer |
| Shader Graph | Material Graph |
| Post-processing Stack | Built-in effects |
| Lightmapping | Lumen (real-time GI) |

### Physics

| Unity | Lunaris |
|-------|---------|
| PhysX | Chaos Physics |
| Rigidbody | `RigidBody` component |
| Collider | `Collider` component |
| CharacterController | `CharacterController` |

### Audio

| Unity | Lunaris |
|-------|---------|
| AudioSource | `AudioEmitter` |
| AudioMixer | `AudioMixer` |
| - | MetaSounds (procedural) |

---

## 🔄 Code Translation

### Before: Unity C#

```csharp
using UnityEngine;

public class PlayerController : MonoBehaviour
{
    public float speed = 5f;
    private Rigidbody rb;

    void Start()
    {
        rb = GetComponent<Rigidbody>();
    }

    void Update()
    {
        float h = Input.GetAxis("Horizontal");
        float v = Input.GetAxis("Vertical");
        
        Vector3 movement = new Vector3(h, 0, v) * speed;
        rb.velocity = movement;
    }
}
```

### After: Lunaris Rust

```rust
use lunaris::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32,
}

fn player_movement(
    input: Res<Input>,
    mut query: Query<(&Player, &mut RigidBody)>,
) {
    for (player, mut rb) in query.iter_mut() {
        let h = input.axis("Horizontal");
        let v = input.axis("Vertical");
        
        let movement = Vec3::new(h, 0.0, v) * player.speed;
        rb.velocity = movement;
    }
}

// Register the system
app.add_system(player_movement);
```

### Key Differences:
- ✅ No inheritance, just data
- ✅ Systems are pure functions
- ✅ Automatic parallelization
- ✅ Compile-time safety

---

## 🎯 Common Patterns

### Spawning Objects

**Unity:**
```csharp
Instantiate(prefab, position, rotation);
```

**Lunaris:**
```rust
commands.spawn((
    Transform::from_position(position).with_rotation(rotation),
    prefab.clone(),
));
```

### Finding Objects

**Unity:**
```csharp
GameObject.Find("Player");
GameObject.FindWithTag("Enemy");
```

**Lunaris:**
```rust
// By component (recommended)
query.iter().find(|(name, _)| name.0 == "Player");

// By tag
query.filter(|e| e.has::<EnemyTag>());
```

### Destroying Objects

**Unity:**
```csharp
Destroy(gameObject);
Destroy(gameObject, 2f); // delayed
```

**Lunaris:**
```rust
commands.despawn(entity);
commands.despawn_delayed(entity, Duration::from_secs(2));
```

### Coroutines → Async

**Unity:**
```csharp
IEnumerator FadeOut()
{
    for (float t = 1; t > 0; t -= Time.deltaTime)
    {
        color.a = t;
        yield return null;
    }
}
StartCoroutine(FadeOut());
```

**Lunaris:**
```rust
async fn fade_out(mut color: Mut<Color>) {
    let mut t = 1.0;
    while t > 0.0 {
        color.a = t;
        t -= time.delta();
        yield_frame().await;
    }
}
spawn_task(fade_out(color));
```

---

## 🖼️ Editor Comparison

### Hierarchy Panel
- Unity: Left side, tree view → **Lunaris: Same!**

### Inspector
- Unity: Right side, component list → **Lunaris: Same!**

### Scene View
- Unity: Center, 3D view → **Lunaris: Same, with better gizmos**

### Console
- Unity: Bottom, logs → **Lunaris: Same, with filtering**

### Project Window
- Unity: Asset browser → **Lunaris: Asset Browser**

**💡 You'll feel right at home!**

---

## 📦 Asset Migration

### Supported Formats

| Type | Formats |
|------|---------|
| 3D Models | `.gltf`, `.glb`, `.fbx`, `.obj` |
| Textures | `.png`, `.jpg`, `.hdr`, `.exr` |
| Audio | `.wav`, `.ogg`, `.mp3`, `.flac` |
| Fonts | `.ttf`, `.otf` |

### Migration Steps

1. **Export from Unity:**
   - Models: Export as FBX or GLTF
   - Textures: Keep original PNGs
   - Audio: Keep originals

2. **Import to Lunaris:**
   ```bash
   lunaris import ./unity_export/
   ```

3. **Auto-conversion:**
   - Textures → Compressed (BC7/ASTC)
   - Models → Optimized meshes
   - Materials → PBR conversion

---

## 🎮 Input System

**Unity (New Input System):**
```csharp
var gamepad = Gamepad.current;
var move = gamepad.leftStick.ReadValue();
```

**Lunaris:**
```rust
let move_input = input.gamepad(0).left_stick();
// Or action-based:
let move_input = input.action::<Vec2>("Move");
```

---

## 🔧 Project Structure

```
unity-project/          →  lunaris-project/
├── Assets/             →  ├── assets/
│   ├── Scripts/        →  │   ├── src/
│   ├── Prefabs/        →  │   ├── prefabs/
│   └── Scenes/         →  │   └── scenes/
├── Packages/           →  ├── Cargo.toml (dependencies)
└── ProjectSettings/    →  └── lunaris.toml
```

---

## 🚀 Quick Start Checklist

- [ ] Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [ ] Install Lunaris: `cargo install lunaris-cli`
- [ ] Create project: `lunaris new my_game`
- [ ] Import assets: Drop into `assets/` folder
- [ ] Open editor: `lunaris editor`
- [ ] Build: `lunaris build --release`

---

## 💬 FAQ

**Q: Do I need to learn Rust?**
A: Basic Rust helps, but Lua scripting works too! Use Lua for prototyping, Rust for performance.

**Q: Can I use my Unity assets?**
A: Yes! Most formats import directly. Materials may need recreation.

**Q: What about multiplayer?**
A: Built-in! Replication, RPCs, prediction - all included.

**Q: Mobile support?**
A: iOS, Android, and all consoles supported.

**Q: VR/AR?**
A: Quest, PSVR2, Vision Pro, and more. Built-in.

---

## 🆘 Getting Help

- 📚 [Documentation](https://docs.lunaris.dev)
- 💬 [Discord](https://discord.gg/lunaris)
- 🐛 [GitHub Issues](https://github.com/lunaris/engine)
- 🎥 [YouTube Tutorials](https://youtube.com/@lunaris)

---

**Welcome to Lunaris! 🌙**

You're going to love the performance, the safety, and the freedom.
