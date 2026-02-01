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

The project relies on a **Pure Makefile Workflow**. GitHub Actions is disabled to ensure all releases are built and signed in a controlled, local environment.

### Updating the Live Demo
To update the GitHub Pages site with new code or frames:
```bash
make deploy
```

### Publishing a New Version
To perform a full, GPG-signed release with binary artifacts:
```bash
make release VERSION=v1.0.0
```
This command:
1.  Runs all quality checks (`clippy`, `rustfmt`, `tests`).
2.  Builds the production-ready release binary.
3.  Creates a **GPG-signed git tag**.
4.  Generates a GPG detached signature (`.asc`) for the binary.
5.  Creates a GitHub release and uploads both the **binary** and the **signature**.

## Development Setup
Ensure you have the following installed:
- Rust (Stable)
- `libwebp` development headers
- `gh` CLI (GitHub CLI)
- `gpg` (For signed releases)