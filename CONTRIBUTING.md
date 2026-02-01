# Contributing to img2webp-rs

We welcome contributions to this Enterprise Grade Rust migration! 

## Technical Philosophy
- **Safety First**: Leverage Rust's ownership model to eliminate C-style buffer overflows and null pointer dereferences.
- **Performance**: Use efficient FFI bindings and parallel processing where possible.
- **Documentation**: All public APIs in `webp_anim` should be documented.

## Quality Control Workflow

We use a `Makefile` to enforce standards across the project. Please use these commands before submitting a Pull Request:

### 1. Code Quality
```bash
make lint   # Automatically fix common lints and format code
make check  # Run full lint check + test suite (Simulates CI)
```

### 2. Verification
```bash
make generate # Re-generate all benchmark animations to verify no visual regressions
```

## Release & Deployment (Maintainers only)

### Updating the Live Demo
To update the GitHub Pages site with new code or frames:
```bash
make deploy
```

### Publishing a New Version
To perform a full, GPG-signed, multi-platform release:
```bash
make release VERSION=v1.0.0
```
This command:
1.  Runs all quality checks.
2.  Creates a **GPG-signed git tag**.
3.  Pushes the tag to GitHub, triggering **GitHub Actions**.
4.  GitHub Actions automatically builds and signs binaries for **Linux**, **Windows**, and **macOS** (x64).

## Development Setup
Ensure you have the following installed:
- Rust (Stable)
- `libwebp` development headers
- `gh` CLI (GitHub CLI)
- `gpg` (For signed releases)
