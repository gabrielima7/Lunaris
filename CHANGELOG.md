# Changelog

All notable changes to Lunaris Engine will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Documentation and tutorials
- CONTRIBUTING.md guidelines

---

## [0.1.0] - 2025-12-11

### 🎮 Core Engine
- Complete Entity Component System (`lunaris-ecs`)
- Scene management with prefabs
- Input handling (keyboard, mouse, gamepad)
- Time management and delta time
- Profiler for performance monitoring

### 🎨 Rendering (`lunaris-renderer`)
- **Lumen-like GI** - Real-time global illumination with SDF
- **Nanite-like Mesh** - Virtualized geometry with GPU culling
- **Hardware Ray Tracing** - DXR, Vulkan RT, Metal RT
- **MetaHuman** - 700+ blend shapes, grooms, LiveLink
- PBR materials and shaders
- VFX Graph (node-based particles)
- Post-processing pipeline
- 2D and 3D rendering pipelines
- Camera system (perspective, orthographic)
- Sprite batching and animation

### ⚡ Physics (`lunaris-physics`)
- **Chaos-like Physics** - Destruction, fields
- Rigidbody dynamics (2D/3D)
- Character controllers
- Vehicle system
- Cloth simulation
- Ragdoll physics
- Collision detection and response

### 🎵 Audio (`lunaris-audio`)
- **MetaSounds-like** - Node-based procedural audio
- Spatial 3D audio with HRTF
- Audio mixer with effects
- Streaming audio support
- Reverb zones

### 📜 Scripting (`lunaris-scripting`)
- **Blueprints** - Full visual scripting
- **AI Copilot** - LLM-powered assistance
- Sandboxed Lua scripting
- Hot reload support
- Capability-based permissions

### 🌐 Runtime (`lunaris-runtime`)
- **Cognitive NPCs** - Dynamic dialogue with LLM
- **Marketplace/Plugins** - Extension system
- Multiplayer networking (replication, RPCs)
- VR/AR support (Quest, Vision Pro, PSVR2)
- Console APIs (PS5, Xbox, Switch)
- Hot reload system
- Save/Load system

### 📦 Assets (`lunaris-assets`)
- **Asset Pipeline** - Auto-import, LOD, Nanite opt
- Streaming with priority queue
- Reference counting
- Multiple format support

### 🛠️ Editor (`lunaris-editor`)
- **World Builder** - Biome painting, vegetation
- Visual scene editor
- Hierarchy and inspector panels
- Gizmos (translate, rotate, scale)
- Timeline/Sequencer
- Console panel

### 🌍 Platforms
- Windows (DX12/Vulkan)
- Linux (Vulkan)
- macOS (Metal)
- iOS (Metal)
- Android (Vulkan)
- PlayStation 5
- Xbox Series X/S
- Nintendo Switch
- WebGPU/WASM
- Meta Quest
- Apple Vision Pro
- Steam Deck
- Cloud Gaming

### 📚 Documentation
- API reference
- Getting started guide
- Platformer tutorial
- FPS tutorial

---

## [0.0.1] - 2025-12-01

### Added
- Initial project setup
- Basic workspace structure
- Core crate skeleton

---

## Legend

- ✨ New feature
- 🐛 Bug fix
- 📝 Documentation
- ⚡ Performance
- 🔒 Security
- 💥 Breaking change
- 🗑️ Deprecated
