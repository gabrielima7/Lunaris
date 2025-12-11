# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial workspace structure with 9 core crates
- `lunaris-core`: Error types, ID system, time utilities
- `lunaris-scripting`: Sandboxed Lua 5.4 with mlua
  - Capability-based security system
  - Resource limits (instructions, memory, stack depth)
  - Trust levels (Untrusted, Verified, Trusted)
- `lunaris-ecs`: bevy_ecs wrapper
- `lunaris-renderer`: wgpu-based rendering (stub)
- `lunaris-physics`: Physics simulation (stub)
- `lunaris-audio`: Audio playback (stub)
- `lunaris-assets`: Asset loading (stub)
- `lunaris-editor`: Visual editor (stub)
- `lunaris-runtime`: Game runtime executable

### Security
- `deny.toml` configuration for dependency auditing
- GitHub Actions workflows for CI/CD and security scanning
- SBOM generation in CycloneDX format
- `cargo-geiger` unsafe code reporting

### Infrastructure
- Multi-platform CI (Linux, Windows, macOS, ARM64)
- Automated benchmarking with criterion
- Code coverage with cargo-llvm-cov
- Release workflow with checksums

## [0.1.0] - TBD

Initial release (planned).
