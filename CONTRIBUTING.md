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
make check  # Run full lint check + test suite
```

### 2. Verification
```bash
make generate # Re-generate all benchmark animations in assets/ to verify no visual regressions
```

## Release & Deployment (Maintainers only)

The project relies on a **Native Cross-Compilation Workflow**. GitHub Actions is disabled to ensure all releases are built and signed in a controlled, local environment.

### Updating the Live Demo
To update the GitHub Pages site with new assets:
```bash
make deploy
```

### Publishing a New Version
To perform a full, GPG-signed, multi-platform release:
```bash
make release VERSION=v1.0.0
```
This command:
1.  **Validates**: Runs all quality checks (`clippy`, `rustfmt`, `tests`).
2.  **Cross-Builds**: Compiles native binaries for:
    - **Linux (x64)** (Native)
    - **Windows (x64)** (via MinGW)
    - **macOS (x64)** (via Zig)
3.  **Signs**: Creates a **GPG-signed git tag** and generates GPG detached signatures (`.asc`) for **all** binaries.
4.  **Publishes**: Creates a GitHub release and uploads all binaries and their corresponding signatures.

## Development Setup
Ensure you have the following installed for full release capability:
- **Rust** (Stable) + `x86_64-pc-windows-gnu` and `x86_64-apple-darwin` targets.
- **Zig** + **cargo-zigbuild** (for macOS builds).
- **MinGW-w64** (for Windows builds).
- **libwebp** development headers.
- **gh** CLI (GitHub CLI).
- **gpg** (For signed releases).
