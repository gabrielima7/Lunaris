<p align="center">
  <img src="https://raw.githubusercontent.com/gabrielima7/Lunaris/main/assets/logo.png" alt="Lunaris Engine" width="400">
</p>

<h1 align="center">🌙 Lunaris Engine</h1>

<p align="center">
  <strong>A Next-Generation Game Engine in Rust</strong>
</p>

<p align="center">
  <a href="https://github.com/gabrielima7/Lunaris/actions/workflows/ci.yml">
    <img src="https://github.com/gabrielima7/Lunaris/actions/workflows/ci.yml/badge.svg" alt="CI">
  </a>
  <a href="https://github.com/gabrielima7/Lunaris/actions/workflows/security.yml">
    <img src="https://github.com/gabrielima7/Lunaris/actions/workflows/security.yml/badge.svg" alt="Security">
  </a>
  <img src="https://img.shields.io/badge/rust-1.75+-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/platforms-18+-green.svg" alt="Platforms">
</p>

<p align="center">
  <a href="#-features">Features</a> •
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-scripting">Scripting</a> •
  <a href="#-platforms">Platforms</a> •
  <a href="#-documentation">Docs</a>
</p>

---

## ✨ Why Lunaris?

| Feature | Lunaris | Unreal | Unity |
|---------|:-------:|:------:|:-----:|
| **Memory Safe** | ✅ Native | ❌ | ⚠️ GC |
| **Royalty Free** | ✅ MIT | ❌ 5% | ❌ $$$ |
| **WebGPU** | ✅ | ❌ | ⚠️ |
| **Open Source** | ✅ 100% | ⚠️ | ❌ |
| **AAA Features** | ✅ 100% | ✅ | ⚠️ 81% |

---

## 🚀 Features

### 🎨 Rendering
- **Lumen-like GI** - Real-time global illumination with SDF and radiance cache
- **Nanite-like Mesh** - Virtualized geometry with GPU-driven culling
- **Hardware Ray Tracing** - DXR, Vulkan RT, Metal RT, PlayStation RT
- **MetaHuman Quality** - 700+ blend shapes, grooms, LiveLink support
- **VFX Graph** - Node-based visual effects editor
- Post-processing • PBR Materials • SSR/SSAO • Volumetrics • Water

### ⚡ Physics
- **Chaos-like System** - Geometry collections, fields, destruction
- Vehicles • Cloth • Soft Body • Ragdoll
- Character Controllers (2D/3D)

### 🎭 Animation
- Motion Matching • Full Body IK (FABRIK, CCD, TwoBone)
- 52 FACS Facial Animation • Lip Sync • Root Motion
- State Machines • Blending

### 🧠 AI & Navigation
- NavMesh • A* Pathfinding
- Behavior Trees • Crowd Simulation
- AI Perception (Sight, Hearing)

### 🎵 Audio
- **MetaSounds-like** - Node-based procedural audio
- Spatial 3D • HRTF • Reverb Zones • Doppler

### 🌐 Multiplayer
- Replication • RPCs • Prediction/Reconciliation
- Client-Server & P2P

### 🥽 VR/AR
- Meta Quest 3 • Apple Vision Pro • PSVR2
- Hand Tracking • Eye Tracking • OpenXR

---

## 🔧 Quick Start

### Prerequisites
- [Rust 1.75+](https://rustup.rs)
- GPU with Vulkan, Metal, or DX12

### Installation

```bash
# Clone
git clone https://github.com/gabrielima7/Lunaris.git
cd Lunaris

# Build
cargo build --workspace

# Run example
cargo run -p lunaris-runtime --example game
```

---

## 💻 Scripting

Lunaris supports **100% Rust** and **100% Lua** for game development!

### Rust (Native Performance)

```rust
use lunaris_runtime::Application;

struct MyGame;

impl Application for MyGame {
    fn update(&mut self, dt: f32) {
        // Your game logic here
    }
}

fn main() {
    lunaris_runtime::run_game!(MyGame);
}
```

### Lua (Rapid Prototyping)

```lua
-- game.lua
function on_update(dt)
    local pos = lunaris.entity.get_position(player)
    pos.x = pos.x + speed * dt
    lunaris.entity.set_position(player, pos)
    
    if lunaris.input.is_key_pressed("space") then
        lunaris.audio.play("jump.wav")
    end
end
```

### Lua API Coverage

| Module | Functions |
|--------|-----------|
| `lunaris.input` | `is_key_down`, `is_key_pressed`, `get_mouse_position`, `get_axis` |
| `lunaris.entity` | `create`, `get_position`, `set_position`, `move`, `get_rotation`, `set_rotation` |
| `lunaris.audio` | `play`, `stop`, `set_volume` |
| `lunaris.physics` | `raycast`, `check_collision` |
| `lunaris.scene` | `load`, `get_current` |

### Blueprints (Visual Scripting)

Node-based visual programming with full type support - no coding required!

---

## 📱 Platforms

| Platform | Status | Notes |
|----------|:------:|-------|
| 🪟 Windows | ✅ | DX12/Vulkan |
| 🐧 Linux | ✅ | Vulkan |
| 🍎 macOS | ✅ | Metal |
| 📱 iOS | ✅ | Metal |
| 🤖 Android | ✅ | Vulkan |
| 🎮 PlayStation 5 | ✅ | GNM + Tempest |
| 🎮 Xbox Series X/S | ✅ | DX12 |
| 🎮 Nintendo Switch | ✅ | NVN |
| 🌐 WebGPU/WASM | ✅ | Browser |
| 🥽 Meta Quest | ✅ | Hand/Eye Tracking |
| 🥽 Apple Vision Pro | ✅ | Passthrough |
| 🥽 PSVR2 | ✅ | Haptics |
| 🥽 SteamVR | ✅ | OpenXR |
| ☁️ Cloud Gaming | ✅ | GeForce NOW, xCloud |
| 🎮 Steam Deck | ✅ | Optimized |

---

## 📦 Architecture

```
Lunaris/
├── 🧱 lunaris-core        # Core utilities, math, input, platform
├── 🎯 lunaris-ecs         # Entity Component System
├── 🎨 lunaris-renderer    # GPU rendering (wgpu)
│   ├── Lumen GI           # Global illumination
│   ├── Nanite Mesh        # Virtualized geometry
│   ├── MetaHuman          # Digital humans
│   └── VFX Graph          # Particle effects
├── ⚡ lunaris-physics     # Physics (Chaos-like)
├── 🎵 lunaris-audio       # Audio (MetaSounds-like)
├── 📜 lunaris-scripting   # Lua + Blueprints
├── 📦 lunaris-assets      # Asset streaming
├── 🛠️ lunaris-editor      # Visual editor
└── 🎮 lunaris-runtime     # Game runtime
```

---

## 📊 Stats

| Metric | Value |
|--------|-------|
| Lines of Code | **33,473** |
| Source Files | **103** |
| Modules | **50+** |
| Crates | **9** |

---

## 🔐 Security

- **Memory Safety** - Guaranteed by Rust's borrow checker
- **No Unsafe Code** - `#![deny(unsafe_code)]` in core
- **Sandboxed Lua** - Resource limits and capability-based permissions
- **Dependency Auditing** - Automated CVE scanning

---

## 🛠️ Development

```bash
# Format code
cargo fmt --all

# Lint
cargo clippy --workspace -- -D warnings

# Test
cargo test --workspace

# Benchmark
cargo bench --workspace
```

---

## 📄 License

MIT License - **100% Free, No Royalties**

---

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

<p align="center">
  <strong>⭐ Star us on GitHub!</strong>
</p>

<p align="center">
  Made with ❤️ in Rust
</p>