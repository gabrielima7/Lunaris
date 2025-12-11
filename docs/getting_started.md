# 🚀 Getting Started with Lunaris

Welcome to the Lunaris Game Engine! This guide will have you building your first game in under 10 minutes.

---

## 📦 Installation

### Prerequisites
- **Rust**: Install from [rustup.rs](https://rustup.rs)
- **GPU**: Vulkan 1.2+ or Metal compatible

### Install Lunaris CLI

```bash
cargo install lunaris-cli
```

Verify installation:
```bash
lunaris --version
# Lunaris Engine 1.0.0
```

---

## 🎮 Your First Project

### Create a New Project

```bash
lunaris new my_awesome_game
cd my_awesome_game
```

This creates:
```
my_awesome_game/
├── assets/           # Your game assets
├── src/
│   └── lib.rs        # Game code
├── Cargo.toml        # Dependencies
└── lunaris.toml      # Project settings
```

### Run the Editor

```bash
lunaris editor
```

### Or Run the Game

```bash
lunaris run
```

---

## 📝 Writing Your First Code

Open `src/lib.rs`:

```rust
use lunaris::prelude::*;

// Define a component
#[derive(Component)]
struct Player {
    speed: f32,
}

// Define a system
fn player_movement(
    input: Res<Input>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        let movement = input.get_axis_2d("Move");
        transform.position += Vec3::new(movement.x, 0.0, movement.y) * player.speed;
    }
}

// Register with the app
pub fn build(app: &mut App) {
    app.add_system(player_movement);
    app.add_startup_system(spawn_player);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player { speed: 5.0 },
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
}
```

---

## 🎨 Key Concepts

### 1. Entity Component System (ECS)

| Concept | Description |
|---------|-------------|
| **Entity** | An ID for a game object |
| **Component** | Data attached to an entity |
| **System** | Logic that operates on components |

```rust
// Component = Data
#[derive(Component)]
struct Health(f32);

// System = Logic
fn damage_system(mut query: Query<&mut Health>) {
    for mut health in query.iter_mut() {
        health.0 -= 1.0;
    }
}
```

### 2. Resources

Global data accessible by all systems:

```rust
#[derive(Resource)]
struct Score(u32);

fn add_score(mut score: ResMut<Score>) {
    score.0 += 10;
}
```

### 3. Events

Communication between systems:

```rust
struct DamageEvent { entity: Entity, amount: f32 }

fn send_damage(mut events: EventWriter<DamageEvent>) {
    events.send(DamageEvent { entity, amount: 10.0 });
}

fn receive_damage(mut events: EventReader<DamageEvent>) {
    for event in events.iter() {
        // Handle damage
    }
}
```

---

## 🎯 Common Tasks

### Spawn an Object

```rust
commands.spawn((
    Transform::from_xyz(0.0, 0.0, 0.0),
    Mesh::cube(1.0),
    Material::default(),
));
```

### Load an Asset

```rust
let mesh: Handle<Mesh> = assets.load("models/player.glb");
let texture: Handle<Texture> = assets.load("textures/player.png");
```

### Handle Input

```rust
if input.just_pressed("Jump") {
    // Jump!
}

let mouse_delta = input.mouse_delta();
let gamepad_axis = input.gamepad(0).left_stick();
```

### Play Audio

```rust
audio.play("sounds/explosion.wav");
audio.play_spatial("sounds/footstep.wav", position);
```

---

## 🔧 Project Configuration

Edit `lunaris.toml`:

```toml
[project]
name = "My Game"
version = "1.0.0"

[window]
title = "My Awesome Game"
width = 1920
height = 1080
fullscreen = false

[rendering]
vsync = true
msaa = 4
renderer = "deferred"

[input]
[input.actions]
Move = ["WASD", "LeftStick"]
Jump = ["Space", "ButtonSouth"]
Fire = ["MouseLeft", "RightTrigger"]
```

---

## 📚 Next Steps

| Guide | Description |
|-------|-------------|
| [3D Basics](./3d_basics.md) | Meshes, materials, lighting |
| [Physics](./physics.md) | Rigid bodies, colliders |
| [UI](./ui.md) | Menus, HUD |
| [Audio](./audio.md) | Sound effects, music |
| [Networking](./networking.md) | Multiplayer |
| [VR/AR](./vr_ar.md) | Virtual reality |

---

## 🆘 Getting Help

- 📖 [Full Documentation](https://docs.lunaris.dev)
- 💬 [Discord Community](https://discord.gg/lunaris)
- 🐛 [GitHub Issues](https://github.com/lunaris/engine)
- 🎥 [YouTube Tutorials](https://youtube.com/@lunaris)

---

**Happy game making! 🎮**
