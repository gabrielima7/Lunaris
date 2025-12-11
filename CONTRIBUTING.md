# Contributing to Lunaris Engine

Thank you for your interest in contributing to Lunaris! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)

---

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment. We do not tolerate harassment, discrimination, or disrespectful behavior.

---

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Git
- A code editor (VS Code with rust-analyzer recommended)

### Fork and Clone

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/Lunaris.git
cd Lunaris
git remote add upstream https://github.com/gabrielima7/Lunaris.git
```

---

## Development Setup

### Build

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build specific crate
cargo build -p lunaris-renderer
```

### Run Tests

```bash
# All tests
cargo test

# Specific crate
cargo test -p lunaris-core

# With output
cargo test -- --nocapture
```

### Run Lints

```bash
# Format check
cargo fmt --all -- --check

# Clippy
cargo clippy --all-targets --all-features -- -D warnings

# Both
cargo fmt && cargo clippy
```

---

## Making Changes

### Branch Naming

```
feature/description    # New features
fix/description        # Bug fixes
docs/description       # Documentation
refactor/description   # Code refactoring
perf/description       # Performance improvements
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new rendering feature
fix: resolve memory leak in physics
docs: update API documentation
refactor: simplify ECS internals
perf: optimize mesh processing
test: add unit tests for audio
```

### Example Workflow

```bash
# Update main
git checkout main
git pull upstream main

# Create feature branch
git checkout -b feature/amazing-feature

# Make changes
# ... edit files ...

# Stage and commit
git add .
git commit -m "feat: add amazing feature"

# Push
git push origin feature/amazing-feature

# Create PR on GitHub
```

---

## Pull Request Process

### Before Submitting

1. ✅ Tests pass: `cargo test`
2. ✅ Lints pass: `cargo clippy`
3. ✅ Formatted: `cargo fmt`
4. ✅ Documentation updated
5. ✅ CHANGELOG.md updated (for significant changes)

### PR Template

```markdown
## Description

Brief description of changes.

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing

Describe how you tested the changes.

## Checklist

- [ ] Tests pass
- [ ] Code is formatted
- [ ] Documentation updated
```

### Review Process

1. Automated CI checks run
2. Maintainers review code
3. Address feedback
4. Approval and merge

---

## Coding Standards

### Rust Style

```rust
// Use descriptive names
pub struct PlayerController { ... }

// Document public items
/// A component that handles player movement.
/// 
/// # Example
/// ```
/// let controller = PlayerController::new(speed);
/// ```
pub struct PlayerController { ... }

// Use Result for fallible operations
pub fn load_asset(path: &str) -> Result<Asset, LoadError> { ... }

// Prefer explicit error types
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("file not found: {0}")]
    NotFound(String),
    #[error("invalid format")]
    InvalidFormat,
}
```

### File Organization

```rust
// Imports
use std::collections::HashMap;
use glam::Vec3;

// Constants
const MAX_ENTITIES: usize = 10000;

// Types
pub struct MySystem { ... }

// Implementations
impl MySystem {
    pub fn new() -> Self { ... }
    pub fn update(&mut self) { ... }
}

// Private functions
fn helper_function() { ... }

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_system() { ... }
}
```

---

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_with_fixtures() {
        let system = TestSystem::default();
        system.run();
        assert!(system.completed());
    }
}
```

### Integration Tests

Place in `tests/` directory:

```rust
// tests/integration_test.rs
use lunaris::prelude::*;

#[test]
fn test_full_system() {
    let app = App::new();
    // Test full system
}
```

### Benchmarks

```rust
#[bench]
fn bench_update(b: &mut Bencher) {
    let mut system = System::new();
    b.iter(|| {
        system.update(0.016);
    });
}
```

---

## Documentation

### Code Documentation

```rust
/// Short description of the item.
///
/// Longer description with more details about behavior,
/// use cases, and important notes.
///
/// # Arguments
///
/// * `param1` - Description of first parameter
/// * `param2` - Description of second parameter
///
/// # Returns
///
/// Description of return value.
///
/// # Errors
///
/// * `ErrorType` - When this error occurs
///
/// # Examples
///
/// ```
/// use lunaris::MyStruct;
///
/// let instance = MyStruct::new();
/// instance.do_something();
/// ```
///
/// # Panics
///
/// Panics if invalid input is provided.
pub fn my_function(param1: i32, param2: &str) -> Result<(), Error> {
    // ...
}
```

### Building Docs

```bash
# Build documentation
cargo doc --no-deps --all-features

# Open in browser
cargo doc --open
```

---

## Getting Help

- 📖 [Documentation](https://docs.lunaris.dev)
- 💬 [Discord](https://discord.gg/lunaris)
- 🐛 [GitHub Issues](https://github.com/gabrielima7/Lunaris/issues)

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing! 🌙
