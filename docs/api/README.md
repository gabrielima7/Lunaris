# Lunaris Engine API Documentation

Complete API reference for the Lunaris Game Engine.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Core Systems](#core-systems)
3. [Rendering](#rendering)
4. [Physics](#physics)
5. [Audio](#audio)
6. [Animation](#animation)
7. [AI](#ai)
8. [Editor](#editor)
9. [Networking](#networking)
10. [Platform](#platform)

---

## Getting Started

### Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone Lunaris
git clone https://github.com/gabrielima7/Lunaris.git
cd Lunaris

# Build
cargo build --release

# Run editor
cargo run --bin lunaris-editor
```

### Quick Start

```rust
use lunaris::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera3dBundle::default());
    
    // Spawn cube
    commands.spawn(MeshBundle {
        mesh: Mesh::cube(1.0),
        material: Material::default(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
    });
}

fn update(time: Res<Time>, mut query: Query<&mut Transform>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(time.delta_seconds());
    }
}
```

---

## Core Systems

### Entity Component System (ECS)

```rust
// Component
#[derive(Component)]
struct Health(f32);

// Resource
#[derive(Resource)]
struct Score(u32);

// System
fn damage_system(mut query: Query<&mut Health>) {
    for mut health in query.iter_mut() {
        health.0 -= 1.0;
    }
}

// Events
struct DamageEvent { entity: Entity, amount: f32 }

fn send_damage(mut events: EventWriter<DamageEvent>) {
    events.send(DamageEvent { entity, amount: 10.0 });
}
```

### Asset Management

```rust
// Load assets
let mesh: Handle<Mesh> = assets.load("models/player.glb");
let texture: Handle<Texture> = assets.load("textures/diffuse.png");
let sound: Handle<AudioClip> = assets.load("sounds/explosion.wav");

// Hot reloading (automatic in editor)
assets.watch_for_changes();
```

### Input System

```rust
// Check input
if input.just_pressed(KeyCode::Space) {
    player.jump();
}

// Get axis
let movement = input.get_axis_2d("Move"); // WASD/Stick

// Gamepad
let trigger = input.gamepad(0).right_trigger();
```

---

## Rendering

### Materials

```rust
let material = Material {
    base_color: Color::rgb(1.0, 0.5, 0.2),
    metallic: 0.0,
    roughness: 0.5,
    emission: Color::BLACK,
    normal_map: Some(normal_texture),
    ..Default::default()
};
```

### Lighting

```rust
// Directional light
commands.spawn(DirectionalLightBundle {
    light: DirectionalLight {
        color: Color::WHITE,
        intensity: 100000.0,
        shadows: true,
    },
    transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
});

// Point light
commands.spawn(PointLightBundle {
    light: PointLight {
        color: Color::rgb(1.0, 0.8, 0.6),
        intensity: 1000.0,
        radius: 10.0,
    },
    transform: Transform::from_xyz(0.0, 5.0, 0.0),
});
```

### Post Processing

```rust
let post_processing = PostProcessing {
    bloom: Bloom { intensity: 0.3, threshold: 1.0 },
    color_grading: ColorGrading { saturation: 1.1, contrast: 1.05 },
    motion_blur: MotionBlur { intensity: 0.5 },
    dof: DepthOfField { focus_distance: 10.0, aperture: 2.8 },
    ..Default::default()
};
```

### Ray Tracing

```rust
let rt_settings = RayTracingSettings {
    enabled: true,
    shadows: true,
    reflections: true,
    global_illumination: true,
    samples_per_pixel: 1,
    max_bounces: 4,
};
```

---

## Physics

### Rigid Bodies

```rust
commands.spawn((
    RigidBody::Dynamic,
    Collider::Sphere { radius: 0.5 },
    Mass(1.0),
    Velocity::linear(Vec3::new(0.0, 10.0, 0.0)),
));
```

### Collision Detection

```rust
fn collision_handler(
    mut events: EventReader<CollisionEvent>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b) => {
                println!("Collision between {:?} and {:?}", a, b);
            }
            CollisionEvent::Ended(a, b) => {}
        }
    }
}
```

### Character Controller

```rust
let controller = CharacterController {
    height: 1.8,
    radius: 0.3,
    step_height: 0.3,
    slope_limit: 45.0,
    ground_check: true,
};
```

---

## Audio

### Playing Sounds

```rust
// 2D sound
audio.play("sounds/ui_click.wav");

// 3D spatial sound
audio.play_spatial("sounds/explosion.wav", position);

// Music with crossfade
audio.play_music("music/ambient.ogg", 2.0);
```

### Audio Settings

```rust
let audio_source = AudioSource {
    clip: audio_clip,
    volume: 0.8,
    pitch: 1.0,
    spatial: true,
    min_distance: 1.0,
    max_distance: 50.0,
    rolloff: Rolloff::Logarithmic,
};
```

---

## Animation

### Playing Animations

```rust
animator.play("walk", 0.2); // With blend time
animator.crossfade("run", 0.3);
animator.set_speed(1.5);
```

### Animation State Machine

```rust
let state_machine = AnimationStateMachine::new()
    .add_state("idle", idle_clip)
    .add_state("walk", walk_clip)
    .add_state("run", run_clip)
    .add_transition("idle", "walk", |params| params.speed > 0.1)
    .add_transition("walk", "run", |params| params.speed > 5.0);
```

### Inverse Kinematics

```rust
let foot_ik = FootIK {
    left_foot_bone: "LeftFoot",
    right_foot_bone: "RightFoot",
    ground_offset: 0.05,
    raycast_distance: 0.5,
};
```

---

## AI

### Navigation

```rust
// Generate NavMesh
let navmesh = NavMesh::generate(&scene, NavMeshSettings::default());

// Find path
let path = navmesh.find_path(start, end);

// AI Agent
let agent = NavAgent {
    speed: 5.0,
    acceleration: 10.0,
    stopping_distance: 0.5,
    avoidance_radius: 0.5,
};
```

### Behavior Trees

```rust
let tree = BehaviorTree::new()
    .selector(vec![
        sequence(vec![
            condition("enemy_visible"),
            action("attack"),
        ]),
        sequence(vec![
            condition("patrol_point_reached"),
            action("next_patrol_point"),
        ]),
        action("move_to_patrol_point"),
    ]);
```

---

## Editor

### Custom Panels

```rust
#[derive(EditorPanel)]
struct MyPanel {
    value: f32,
}

impl Panel for MyPanel {
    fn ui(&mut self, ui: &mut Ui) {
        ui.slider("Value", &mut self.value, 0.0..=1.0);
    }
}
```

### Gizmos

```rust
fn draw_gizmos(mut gizmos: Gizmos) {
    gizmos.line(start, end, Color::RED);
    gizmos.sphere(center, radius, Color::GREEN);
    gizmos.box(center, size, Color::BLUE);
}
```

---

## Networking

### Server/Client

```rust
// Start server
let server = NetworkServer::bind("0.0.0.0:7777");

// Connect client
let client = NetworkClient::connect("127.0.0.1:7777");

// Send message
client.send(GameMessage::PlayerMove { position, rotation });

// Receive
for message in server.receive() {
    match message {
        GameMessage::PlayerMove { position, rotation } => { }
    }
}
```

### Replication

```rust
#[derive(Replicated)]
struct PlayerPosition(Vec3);

#[derive(Replicated)]
struct PlayerHealth(f32);
```

---

## Platform

### Build Configuration

```toml
# lunaris.toml
[build]
target = ["windows", "linux", "macos", "android", "ios", "webgl"]

[build.windows]
icon = "assets/icon.ico"
product_name = "My Game"

[build.android]
package = "com.example.mygame"
min_sdk = 26
```

### VR Support

```rust
if let Some(vr) = vr_system.get_headset() {
    let left_hand = vr.controller(Hand::Left);
    let right_hand = vr.controller(Hand::Right);
    
    if left_hand.button_pressed(VRButton::Trigger) {
        // Fire
    }
}
```

---

## Best Practices

1. **Use ECS properly** - Keep components small and focused
2. **Profile regularly** - Use the built-in profiler
3. **Batch draw calls** - Use instancing for many similar objects
4. **Stream assets** - Use async loading for large assets
5. **Pool objects** - Reuse entities instead of spawning/despawning

---

## Support

- 📖 [Full Documentation](https://docs.lunaris.dev)
- 💬 [Discord Community](https://discord.gg/lunaris)
- 🐛 [GitHub Issues](https://github.com/gabrielima7/Lunaris/issues)
