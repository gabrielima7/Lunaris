# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

Please report security vulnerabilities to **security@lunaris.dev**.

### What to Include

1. Description of the vulnerability
2. Steps to reproduce
3. Potential impact
4. Suggested fix (if any)

### Response Timeline

- **24 hours**: Initial acknowledgment
- **72 hours**: Preliminary assessment
- **7 days**: Fix development (for critical issues)
- **30 days**: Full resolution and disclosure

### Bug Bounty

We appreciate responsible disclosure. Significant security contributions will be acknowledged in our [CHANGELOG.md](CHANGELOG.md) and [CONTRIBUTORS](CONTRIBUTORS).

## Security Measures

### Memory Safety

Lunaris is written in **Rust**, which provides:
- No null pointer dereferences
- No buffer overflows
- No use-after-free
- No data races

### Sandboxed Scripting

Lua scripts run in a **secure sandbox**:
- Memory limits (default: 64MB)
- CPU time limits
- No file system access (unless explicitly granted)
- No network access (unless explicitly granted)
- Capability-based permissions

### Dependency Auditing

We use automated security scanning:

```bash
# Run locally
cargo audit

# Check licenses
cargo deny check
```

### Secure Defaults

- All network communication uses TLS
- Sensitive data is encrypted at rest
- Secure random number generation
- Input validation on all user data
