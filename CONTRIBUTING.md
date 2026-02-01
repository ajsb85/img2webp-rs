# Contributing to img2webp-rs

We welcome contributions! Please follow these guidelines:

1.  **Code Style**: Adhere to standard Rust formatting (`cargo fmt`).
2.  **Commits**: Use descriptive, single-line subjects for commits. Provide more context in the body if necessary.
3.  **Safety**: Since this project wraps a C library via FFI, ensure all `unsafe` blocks are well-documented and minimal.
4.  **Tests**: Add tests for new features in the `webp_anim` crate.

## Automation with Makefile

The project includes a `Makefile` to streamline development and deployment. 

### Available Targets:

- `make build`: Compiles the Rust workspace in release mode. Essential before running the tool or generating assets.
- `make generate`: Re-generates all the WebP animation variants used in the benchmarks. It requires the `frames/` directory to be present.
- `make test`: Runs the full Rust test suite.
- `make deploy`: This is the standard procedure for updating the GitHub Pages site. It builds the tool, generates new animations, and pushes the results to the `main` branch.
- `make release VERSION=vX.Y.Z`: Performs a full, secure release:
    1. Builds the latest binary.
    2. Creates a GPG-signed git tag.
    3. Generates a GPG detached signature (`.asc`) for the binary.
    4. Creates a GitHub release with both the binary and the signature.
- `make clean`: Removes all build artifacts and generated WebP files to ensure a fresh state.

## Process

1. Fork the repo.
2. Create a feature branch.
3. Submit a Pull Request.