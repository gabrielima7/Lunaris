# Lunaris Engine Documentation

Welcome to the Lunaris Engine documentation! 🎮

## 📚 Table of Contents

- [Getting Started](#-getting-started)
- [Core Concepts](#-core-concepts)
- [Tutorials](#-tutorials)
- [API Reference](#-api-reference)
- [Examples](#-examples)

---

## 🚀 Getting Started

### Installation

```bash
# Add to Cargo.toml
[dependencies]
lunaris-runtime = "0.1"
lunaris-renderer = "0.1"
lunaris-physics = "0.1"
```

### Your First Game

```rust
use lunaris_runtime::{Application, WindowConfig, run_game};

struct MyGame {
    score: u32,
}

impl Application for MyGame {
    fn init(&mut self) {
        println!("Game initialized!");
    }

    fn update(&mut self, dt: f32) {
        // Game logic here
    }

    fn render(&mut self) {
        // Rendering here
    }
}

fn main() {
    let game = MyGame { score: 0 };
    run_game!(game);
}
```

---

## 🧠 Core Concepts

### Entity Component System (ECS)

Lunaris uses a data-oriented ECS architecture:

```rust
use lunaris_ecs::{World, Entity};

// Create world
let mut world = World::new();

// Spawn entity
let player = world.spawn();

// Add components
world.add_component(player, Position { x: 0.0, y: 0.0 });
world.add_component(player, Velocity { x: 1.0, y: 0.0 });

// Query entities
for (entity, pos, vel) in world.query::<(Position, Velocity)>() {
    // Update positions
}
```

### Rendering Pipeline

```rust
use lunaris_renderer::{Camera3D, Render3D};

// Create camera
let camera = Camera3D::perspective(45.0, 16.0/9.0, 0.1, 1000.0);

// Create renderer
let mut renderer = Render3D::new();

// Render scene
renderer.begin_frame(&camera);
renderer.draw_mesh(&mesh, &transform, &material);
renderer.end_frame();
```

### Physics

```rust
use lunaris_physics::{PhysicsWorld, RigidbodyType};

// Create physics world
let mut physics = PhysicsWorld::new(Vec3::new(0.0, -9.81, 0.0));

// Add rigidbody
let body = physics.create_rigidbody(
    position,
    RigidbodyType::Dynamic,
    collider,
);

// Step simulation
physics.step(delta_time);
```

### Scripting (Lua)

```lua
-- player.lua
function on_init()
    print("Player initialized!")
end

function on_update(dt)
    local pos = lunaris.entity.get_position(self)
    
    if lunaris.input.is_key_down("W") then
        pos.y = pos.y + speed * dt
    end
    
    lunaris.entity.set_position(self, pos)
end

function on_collision(other)
    if other.tag == "enemy" then
        lunaris.audio.play("hit.wav")
    end
end
```

---

## 📖 Tutorials

### Tutorial 1: 2D Platformer

Learn to create a simple 2D platformer with:
- Player movement
- Jumping and gravity
- Collectibles
- Enemies

[Read Tutorial →](tutorials/platformer.md)

### Tutorial 2: 3D First Person

Create a first-person game with:
- Camera controls
- Physics-based movement
- Shooting mechanics
- AI enemies

[Read Tutorial →](tutorials/fps.md)

### Tutorial 3: RPG Systems

Implement RPG mechanics:
- Inventory system
- Quest system
- Dialogue trees
- Save/Load

[Read Tutorial →](tutorials/rpg.md)

---

## 📋 API Reference

### Core Modules

| Module | Description |
|--------|-------------|
| `lunaris_core` | Core utilities, math, input |
| `lunaris_ecs` | Entity Component System |
| `lunaris_renderer` | GPU rendering |
| `lunaris_physics` | Physics simulation |
| `lunaris_audio` | Audio playback |
| `lunaris_scripting` | Lua + Blueprints |
| `lunaris_assets` | Asset management |
| `lunaris_editor` | Visual editor |
| `lunaris_runtime` | Game runtime |

### Key Types

```rust
// Transform
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

// Camera
pub struct Camera3D {
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

// Material
pub struct Material {
    pub albedo: Color,
    pub metallic: f32,
    pub roughness: f32,
}
```

---

## 💡 Examples

### Hello World

```rust
use lunaris_runtime::Application;

struct HelloWorld;

impl Application for HelloWorld {
    fn update(&mut self, _dt: f32) {
        println!("Hello, Lunaris!");
    }
}
```

### Sprite Animation

```rust
use lunaris_renderer::{Sprite, SpriteAnimation};

let mut anim = SpriteAnimation::new("player_walk", 0.1);
anim.add_frames(&[0, 1, 2, 3, 2, 1]);
anim.play();
```

### Audio

```rust
use lunaris_audio::AudioSystem;

let mut audio = AudioSystem::new();
audio.play_sound("music.ogg", 0.8, true); // looping
audio.play_sound("jump.wav", 1.0, false); // one-shot
```

### Networking

```rust
use lunaris_runtime::network::{NetworkServer, NetworkClient};

// Server
let mut server = NetworkServer::new(7777);
server.on_connect(|client| {
    println!("Client connected: {:?}", client);
});

// Client
let mut client = NetworkClient::new();
client.connect("127.0.0.1:7777");
```

---

## 🔧 Configuration

### Window Config

```rust
let config = WindowConfig {
    title: "My Game".to_string(),
    width: 1920,
    height: 1080,
    fullscreen: false,
    vsync: true,
    resizable: true,
};
```

### Render Config

```rust
let config = RenderConfig {
    msaa: 4,
    shadow_quality: ShadowQuality::High,
    gi_enabled: true,
    ray_tracing: false,
};
```

---

## 🆘 Troubleshooting

### Common Issues

**Q: Game won't start**
- Check GPU drivers are up to date
- Ensure Vulkan/Metal/DX12 is available

**Q: Low FPS**
- Enable frustum culling
- Use LOD groups
- Check draw call count

**Q: Audio not playing**
- Check audio file format (WAV, OGG, MP3)
- Verify volume settings

---

## 📞 Support

- **GitHub Issues**: Bug reports
- **Discord**: Community help
- **Email**: support@lunaris.dev

---

Made with ❤️ by the Lunaris Team
