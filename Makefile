# Project Configuration
BINARY_NAME = img2webp
BINARY_PATH = rs/target/release/$(BINARY_NAME)
OUTPUT_FILES = animated.webp animated_perfect.webp animated_lossy.webp animated_slowest.webp animated_small.webp animated_near_lossless60.webp animated_lossy_low.webp animated_backward.webp animated_pingpong.webp
FRAMES = frames/frame_*.webp

# Default target
.PHONY: all
all: build generate

# Build the Rust tool
.PHONY: build
build:
	@echo "Building img2webp-rs in release mode..."
	@cd rs && cargo build --release

# Generate all animation variants
.PHONY: generate
generate: build
	@echo "Generating standard animation..."
	@$(BINARY_PATH) -o animated.webp -loop 0 -d 100 $(FRAMES)
	@echo "Generating perfect animation (-lossless -exact)..."
	@$(BINARY_PATH) -o animated_perfect.webp -loop 0 -d 100 -lossless -exact $(FRAMES)
	@echo "Generating lossy sharp animation..."
	@$(BINARY_PATH) -o animated_lossy.webp -loop 0 -d 100 -lossy -q 75 -sharp_yuv $(FRAMES)
	@echo "Generating smallest animation (-min_size -mixed)..."
	@$(BINARY_PATH) -o animated_small.webp -loop 0 -d 100 -min_size -mixed -alpha_q 50 $(FRAMES)
	@echo "Generating slowest/best compression animation (-m 6)..."
	@$(BINARY_PATH) -o animated_slowest.webp -loop 0 -d 100 -m 6 -alpha_filter 2 -exact $(FRAMES)
	@echo "Generating near-lossless 60 animation..."
	@$(BINARY_PATH) -o animated_near_lossless60.webp -loop 0 -d 100 -near_lossless 60 $(FRAMES)
	@echo "Generating lossy low quality animation..."
	@$(BINARY_PATH) -o animated_lossy_low.webp -loop 0 -d 100 -lossy -q 30 -alpha_q 30 $(FRAMES)
	@echo "Generating backward animation (-reverse)..."
	@$(BINARY_PATH) -o animated_backward.webp -loop 0 -d 100 -reverse $(FRAMES)
	@echo "Generating ping-pong loop (-pingpong)..."
	@$(BINARY_PATH) -o animated_pingpong.webp -loop 0 -d 100 -pingpong $(FRAMES)

# Run Rust tests
.PHONY: test
test:
	@echo "Running tests..."
	@cd rs && cargo test

# Quality Control: Linting and Formatting
.PHONY: lint
lint:
	@echo "Running clippy..."
	@cd rs && cargo clippy --fix --allow-dirty --allow-staged -- -D warnings
	@echo "Standardizing formatting..."
	@cd rs && cargo fmt

# Comprehensive quality check
.PHONY: check
check: lint test

# Clean build artifacts and generated images
.PHONY: clean
clean:
	@echo "Cleaning up..."
	@cd rs && cargo clean
	@rm -f $(OUTPUT_FILES)

# Deploy to GitHub Pages
.PHONY: deploy
deploy: check generate
	@echo "Deploying to GitHub Pages..."
	@git add .gitignore index.html .nojekyll $(OUTPUT_FILES)
	@git commit -m "deploy: automated update via Makefile" || echo "No changes to commit"
	@git push origin main
	@echo "Deployment complete. Visit: https://ajsb85.github.io/img2webp-rs/"

# Create a GPG-signed release on GitHub with multi-platform support
# Usage: make release VERSION=v1.0.0
.PHONY: release
release:
	@if [ -z "$(VERSION)" ]; then echo "Error: VERSION is not set. Usage: make release VERSION=v1.0.0"; exit 1; fi
	@echo "Performing pre-release checks..."
	@make check
	@echo "Tagging version $(VERSION)..."
	@git tag -s $(VERSION) -m "Release $(VERSION)"
	@git push origin $(VERSION)
	@echo "Triggering multi-platform build via GitHub Actions..."
	@echo "Release $(VERSION) is being processed. Binaries for Linux, Windows, and macOS will appear shortly."
	@echo "Visit: https://github.com/ajsb85/img2webp-rs/releases/tag/$(VERSION)"

# Local signing of specific artifacts (for manual override)
.PHONY: sign
sign: build
	@echo "Signing Linux binary..."
	@gpg --armor --detach-sign $(BINARY_PATH)

# Help target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make build           - Build the Rust tool (native)"
	@echo "  make generate        - Create all WebP animation variants"
	@echo "  make test            - Run Rust unit tests"
	@echo "  make lint            - Auto-fix lints and format code"
	@echo "  make check           - Comprehensive Lint + Test"
	@echo "  make deploy          - Check, Generate Assets, and Push to GH Pages"
	@echo "  make release VERSION=vX.Y.Z - Tag and trigger Multi-Platform release"
	@echo "  make clean           - Remove all build and asset artifacts"
	@echo "  make all             - Build and generate (default)"
