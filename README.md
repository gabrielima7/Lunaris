# Lunaris Engine

[![CI](https://github.com/gabrielima7/Lunaris/actions/workflows/ci.yml/badge.svg)](https://github.com/gabrielima7/Lunaris/actions/workflows/ci.yml)
[![Security](https://github.com/gabrielima7/Lunaris/actions/workflows/security.yml/badge.svg)](https://github.com/gabrielima7/Lunaris/actions/workflows/security.yml)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

A high-performance, industrial-grade game engine written in **Rust** with **Lua** scripting.

## 🎯 Vision

Lunaris aims to be a modern game engine that combines:
- **Memory Safety**: Built entirely in Rust for memory safety without sacrificing performance
- **Performance**: Optimized for modern hardware with multi-threaded architecture
- **Security**: Sandboxed scripting with capability-based permissions
- **Cross-Platform**: Windows, Linux, macOS, and console support

## 📦 Workspace Structure

```
Lunaris/
├── crates/
│   ├── lunaris-core       # Core utilities, error types, ID system
│   ├── lunaris-ecs        # Entity Component System (bevy_ecs)
│   ├── lunaris-renderer   # GPU rendering (wgpu)
│   ├── lunaris-physics    # Physics simulation
│   ├── lunaris-audio      # Audio playback
│   ├── lunaris-scripting  # Sandboxed Lua scripting
│   ├── lunaris-assets     # Asset loading and management
│   ├── lunaris-editor     # Visual editor
│   └── lunaris-runtime    # Game runtime
├── deny.toml              # Security policies
├── clippy.toml            # Lint configuration
└── rustfmt.toml           # Code formatting
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs))
- For GPU features: Vulkan, Metal, or DX12 compatible graphics

### Build

```bash
# Clone the repository
git clone https://github.com/gabrielima7/Lunaris.git
cd Lunaris

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run benchmarks
cargo bench --workspace
```

## 🔐 Security

Lunaris follows a security-first approach:

- **Dependency Auditing**: Automated CVE scanning with `cargo-audit`
- **License Compliance**: Enforced via `cargo-deny`
- **Sandboxed Scripting**: Lua scripts run in isolated environments with resource limits
- **Capability-Based Permissions**: Scripts only access APIs they're granted

### Running Security Checks

```bash
# Install security tools
cargo install cargo-audit cargo-deny

# Run security audit
cargo audit

# Check dependency policies
cargo deny check
```

## 🎮 Scripting

Lua scripts run in a secure sandbox with configurable trust levels:

```lua
-- Example game script (runs in sandbox)
function on_update(dt)
    local pos = lunaris.entity.get_position(self)
    pos.x = pos.x + 10 * dt
    lunaris.entity.set_position(self, pos)
end
```

| Trust Level | Capabilities |
|-------------|--------------|
| **Untrusted** | Basic gameplay APIs, math, logging |
| **Verified** | + Entity modification, config read |
| **Trusted** | + File system (game dir), debug |

## 📊 Performance

Benchmarks run automatically on each commit:

```bash
# Run benchmarks locally
cargo bench -p lunaris-core
cargo bench -p lunaris-scripting
```

## 🛠️ Development

### Code Quality

```bash
# Format code
cargo fmt --all

# Run lints
cargo clippy --workspace --all-targets -- -D warnings

# Check for unsafe code
cargo install cargo-geiger
cargo geiger
```

### Cross-Platform Builds

Supported targets:
- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu` (ARM64/Steam Deck)
- `x86_64-pc-windows-msvc`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin` (Apple Silicon)

## 📝 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🤝 Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.