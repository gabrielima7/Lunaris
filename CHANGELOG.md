# Changelog

All notable changes to Lunaris Engine will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-12-11

### 🎉 Initial Release

The first stable release of Lunaris Engine - a production-ready game engine written in Rust.

### Added

#### Core Engine
- Complete Entity Component System (ECS)
- Asset management with hot reloading
- Resource management
- Event system
- Input system with gamepad support

#### Rendering (25+ systems)
- Lumen-style Global Illumination
- Nanite-style virtualized geometry
- Ray tracing (shadows, reflections, GI)
- Path tracing for offline rendering
- Subsurface Scattering (skin, wax, etc.)
- Strand-based hair rendering
- Mesh shaders with Variable Rate Shading
- Virtual texturing
- RT shadows with area lights
- GPU particles (millions)
- NeRF (Neural Radiance Fields)
- Gaussian Splatting
- SDF raymarching with CSG
- FFT ocean simulation
- Volumetric fog and clouds
- Screen space effects (SSAO, SSR, SSGI)
- Depth of Field with bokeh
- Motion blur
- Post-processing pipeline
- Terrain system with LOD

#### Physics (10+ systems)
- Rigid body dynamics
- Collision detection
- Character controller
- Destruction system
- Vehicle physics (cars, boats, aircraft)
- Cloth simulation
- Soft body physics
- Fluid simulation (SPH)
- Inverse dynamics

#### Animation (10+ systems)
- Skeletal animation
- Animation blending
- Motion matching
- IK (FABRIK, two-bone, look-at)
- Ragdoll blending
- Spring bones
- Procedural animation
- Sequencer

#### Audio (8+ systems)
- 3D spatial audio
- MetaSounds-style system
- Audio occlusion
- Reverb zones
- DSP effects (distortion, chorus, etc.)

#### AI (10+ systems)
- NavMesh pathfinding
- Behavior trees
- Blackboard system
- Crowd simulation
- Machine learning integration

#### Editor (20+ modules)
- Visual graph editor
- Curve editor
- Timeline
- Dock system
- Terrain editor
- Collaboration

#### Networking
- Client-server architecture
- State replication
- Prediction/reconciliation
- Cloud services

#### Platform Support
- Windows
- Linux
- macOS
- WebGL
- Android
- iOS
- VR (OpenXR)
- Console (PlayStation, Xbox, Nintendo)

### Documentation
- Getting started guide
- API reference
- Migration guides (Unity, Unreal)
- Example projects

### Examples
- Space shooter demo
- Survival horror demo
- AAA visual showcase

---

## [Unreleased]

### Planned
- WebGPU backend
- More examples
- Plugin marketplace

---

## Statistics

- **Files**: 181 Rust files
- **Lines**: 61,000+ lines of code
- **Systems**: 130+ complete systems
- **Commits**: 31

---

## License

MIT License - see [LICENSE](LICENSE) for details.
