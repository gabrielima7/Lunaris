# Contributing to Lunaris Engine

Thank you for your interest in contributing to Lunaris! 🎮

## 🚀 Quick Start

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/YOUR_USERNAME/Lunaris.git`
3. **Create a branch**: `git checkout -b feature/my-feature`
4. **Make changes** and commit: `git commit -m "feat: add my feature"`
5. **Push**: `git push origin feature/my-feature`
6. **Open a Pull Request**

## 📋 Development Setup

### Prerequisites

- **Rust 1.75+**: Install via [rustup](https://rustup.rs)
- **Git**: For version control
- **GPU**: Vulkan, Metal, or DX12 compatible

### Build

```bash
# Clone
git clone https://github.com/gabrielima7/Lunaris.git
cd Lunaris

# Build
cargo build --workspace

# Test
cargo test --workspace

# Run lints
cargo clippy --workspace -- -D warnings

# Format
cargo fmt --all
```

## 📝 Commit Convention

We use **Conventional Commits**:

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation |
| `style` | Formatting (no code change) |
| `refactor` | Code refactoring |
| `test` | Add/fix tests |
| `chore` | Build/tooling |
| `perf` | Performance |

**Examples:**
```
feat: add hardware ray tracing support
fix: resolve memory leak in asset loader
docs: update README with examples
```

## 🏗️ Project Structure

```
Lunaris/
├── crates/
│   ├── lunaris-core       # Core utilities
│   ├── lunaris-ecs        # Entity Component System
│   ├── lunaris-renderer   # GPU rendering
│   ├── lunaris-physics    # Physics simulation
│   ├── lunaris-audio      # Audio system
│   ├── lunaris-scripting  # Lua + Blueprints
│   ├── lunaris-assets     # Asset management
│   ├── lunaris-editor     # Visual editor
│   └── lunaris-runtime    # Game runtime
├── examples/              # Example games
├── docs/                  # Documentation
└── tests/                 # Integration tests
```

## 🎯 Areas to Contribute

### 🟢 Good First Issues
- Documentation improvements
- Code comments
- Example projects
- Typo fixes

### 🟡 Intermediate
- New asset importers
- UI widgets
- Audio effects
- Shader improvements

### 🔴 Advanced
- Platform backends (consoles)
- Rendering features
- Physics improvements
- Networking systems

## 📐 Code Guidelines

### Rust Style

```rust
// ✅ Good
pub fn calculate_damage(base: f32, multiplier: f32) -> f32 {
    base * multiplier
}

// ❌ Bad
pub fn calc(b:f32,m:f32)->f32{b*m}
```

### Documentation

```rust
/// Calculate damage with multiplier
///
/// # Arguments
///
/// * `base` - Base damage value
/// * `multiplier` - Damage multiplier
///
/// # Returns
///
/// Final damage value
///
/// # Example
///
/// ```
/// let damage = calculate_damage(10.0, 1.5);
/// assert_eq!(damage, 15.0);
/// ```
pub fn calculate_damage(base: f32, multiplier: f32) -> f32 {
    base * multiplier
}
```

### Error Handling

```rust
// ✅ Use Result for fallible operations
pub fn load_asset(path: &Path) -> Result<Asset, Error> {
    // ...
}

// ❌ Don't panic in library code
pub fn load_asset(path: &Path) -> Asset {
    panic!("Failed!"); // Never do this
}
```

## 🧪 Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_calculation() {
        let damage = calculate_damage(10.0, 2.0);
        assert_eq!(damage, 20.0);
    }
}
```

### Run Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p lunaris-core

# With output
cargo test -- --nocapture
```

## 🔍 Pull Request Checklist

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] Documentation added/updated
- [ ] Commit messages follow convention
- [ ] PR description explains changes

## 📜 License

By contributing, you agree that your contributions will be licensed under MIT.

## 🤝 Code of Conduct

- Be respectful and inclusive
- Constructive feedback only
- Help others learn
- No harassment or discrimination

## 💬 Getting Help

- **Discord**: [Coming soon]
- **GitHub Issues**: For bugs and features
- **Discussions**: For questions

---

Thank you for contributing to Lunaris! 🌙
