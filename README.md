<div align="center">

# 🌙 Lunaris Engine

**A next-generation game engine written in Rust**

[![CI](https://github.com/gabrielima7/Lunaris/actions/workflows/ci.yml/badge.svg)](https://github.com/gabrielima7/Lunaris/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Lines of Code](https://img.shields.io/badge/lines-61K+-blue.svg)](https://github.com/gabrielima7/Lunaris)
[![Discord](https://img.shields.io/badge/Discord-Join-7289da.svg)](https://discord.gg/lunaris)

[Documentation](https://docs.lunaris.dev) • [Getting Started](#-quick-start) • [Examples](#-examples) • [Contributing](#-contributing)

</div>

---

## ✨ Features

<table>
<tr>
<td width="50%">

### 🎨 Rendering
- **Lumen-style** Global Illumination
- **Nanite-style** Virtualized Geometry
- **Ray Tracing** (Shadows, Reflections, GI)
- **Path Tracing** for offline rendering
- **NeRF** & **Gaussian Splatting**
- **Subsurface Scattering** (Skin, Wax)
- **Strand-based Hair** Rendering
- **Mesh Shaders** with VRS
- **Virtual Texturing**
- **FFT Ocean** Simulation

</td>
<td width="50%">

### ⚡ Physics
- **Rigid Body** Dynamics
- **Destruction** System
- **Vehicle Physics** (Cars, Boats, Aircraft)
- **Cloth Simulation**
- **Soft Body** Physics
- **Fluid Simulation** (SPH)
- **Inverse Dynamics**
- **Character Controller**

</td>
</tr>
<tr>
<td>

### 🎭 Animation
- **Motion Matching**
- **IK Systems** (FABRIK, Look-At, Foot)
- **Ragdoll Blending**
- **Spring Bones**
- **Procedural Animation**
- **Sequencer**

</td>
<td>

### 🧠 AI
- **NavMesh** Pathfinding
- **Behavior Trees**
- **Crowd Simulation**
- **Machine Learning** Integration
- **Reinforcement Learning**

</td>
</tr>
<tr>
<td>

### 🔊 Audio
- **3D Spatial Audio**
- **Audio Occlusion**
- **Reverb Zones**
- **DSP Effects**
- **MetaSounds-style** System

</td>
<td>

### 🌐 Networking
- **Client-Server** Architecture
- **State Replication**
- **Prediction/Reconciliation**
- **Cloud Services**

</td>
</tr>
</table>

---

## 📊 Comparison

| Feature | Lunaris | Unreal | Unity | Godot |
|---------|:-------:|:------:|:-----:|:-----:|
| Lines of Code | **61K** | 3M+ | 2M+ | 1M+ |
| Language | **Rust** | C++ | C# | GDScript |
| Memory Safe | ✅ | ❌ | ❌ | ❌ |
| Royalties | **0%** | 5% | Runtime | 0% |
| Open Source | **100%** | Partial | ❌ | ✅ |
| Ray Tracing | ✅ | ✅ | ✅ | 🔜 |
| Path Tracing | ✅ | ✅ | ❌ | ❌ |
| NeRF | ✅ | ❌ | ❌ | ❌ |
| Gaussian Splatting | ✅ | ❌ | ❌ | ❌ |
| Motion Matching | ✅ | ✅ | Plugin | ❌ |

---

## 🚀 Quick Start

### Installation

#### Linux / macOS
```bash
curl -fsSL https://raw.githubusercontent.com/gabrielima7/Lunaris/main/scripts/install.sh | bash
```

#### Windows
```powershell
iwr -useb https://raw.githubusercontent.com/gabrielima7/Lunaris/main/scripts/install.ps1 | iex
```

#### From Source
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/gabrielima7/Lunaris.git
cd Lunaris
cargo build --release
```

### Hello World

```rust
use lunaris::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera3dBundle::default());
    
    // Cube
    commands.spawn(MeshBundle {
        mesh: Mesh::cube(1.0),
        material: Material::default(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
    });
    
    // Light
    commands.spawn(DirectionalLightBundle {
        light: DirectionalLight::default(),
        ..default()
    });
}
```

---

## 📚 Examples

| Example | Description |
|---------|-------------|
| [Space Shooter](examples/space_shooter) | Classic arcade game |
| [Survival Horror](examples/survival_horror) | First-person horror |
| [AAA Showcase](examples/aaa_showcase) | Visual feature demo |

```bash
# Run examples
cargo run --example space_shooter
cargo run --example survival_horror
```

---

## 📖 Documentation

- [Getting Started](docs/getting_started.md)
- [API Reference](docs/api/README.md)
- [Unity Migration Guide](docs/tutorials/unity_migration.md)
- [Unreal Migration Guide](docs/tutorials/unreal_migration.md)

---

## 🏗️ Architecture

```
lunaris/
├── crates/
│   ├── lunaris-core/       # Core systems (ECS, Assets, Math)
│   ├── lunaris-runtime/    # Runtime (Physics, Audio, AI, Input)
│   ├── lunaris-renderer/   # Rendering (GI, RT, Post-processing)
│   └── lunaris-editor/     # Editor (UI, Tools, Panels)
├── examples/               # Example projects
├── docs/                   # Documentation
└── scripts/                # Build & install scripts
```

---

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/Lunaris.git

# Create branch
git checkout -b feature/amazing-feature

# Make changes and test
cargo test
cargo clippy

# Submit PR
```

---

## 📜 License

MIT License - see [LICENSE](LICENSE) for details.

**No royalties. No runtime fees. 100% free.**

---

## 🌟 Star History

If you like Lunaris, please give us a ⭐!

---

<div align="center">

Made with ❤️ and Rust

[⬆ Back to top](#-lunaris-engine)

</div>