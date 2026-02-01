# Project Configuration
BINARY_PATH = rs/target/release/img2webp
OUTPUT_FILES = animated.webp animated_perfect.webp animated_lossy.webp animated_slowest.webp animated_small.webp
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

# Run Rust tests
.PHONY: test
test:
	@echo "Running tests..."
	@cd rs && cargo test

# Quality Control: Linting and Formatting
.PHONY: lint
lint:
	@echo "Running clippy..."
	@cd rs && cargo clippy -- -D warnings
	@echo "Checking formatting..."
	@cd rs && cargo fmt --all -- --check

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

# Create a GPG-signed release on GitHub
# Usage: make release VERSION=v0.1.0
.PHONY: release
release:
	@if [ -z "$(VERSION)" ]; then echo "Error: VERSION is not set. Usage: make release VERSION=v1.2.3"; exit 1; fi
	@echo "Preparing release $(VERSION)..."
	@make check
	@make build
	@echo "Signing tag $(VERSION)..."
	@git tag -s $(VERSION) -m "Release $(VERSION)"
	@git push origin $(VERSION)
	@echo "Signing binary..."
	@rm -f $(BINARY_PATH).asc
	@gpg --armor --detach-sign $(BINARY_PATH)
	@echo "Creating GitHub release..."
	@gh release create $(VERSION) $(BINARY_PATH) $(BINARY_PATH).asc \
		--title "$(VERSION)" \
		--notes "Release $(VERSION) generated and signed via Makefile."
	@echo "Release $(VERSION) published successfully."

# Help target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make build           - Build the Rust tool"
	@echo "  make generate        - Create all WebP animation variants"
	@echo "  make test            - Run Rust tests"
	@echo "  make lint            - Run clippy and format check"
	@echo "  make check           - Run lint and tests (CI simulation)"
	@echo "  make deploy          - Run check, generate assets, and push to GitHub Pages"
	@echo "  make release VERSION=vX.Y.Z - Tag, sign, and publish a new release"
	@echo "  make clean           - Remove build artifacts and generated WebP files"
	@echo "  make all             - Build and generate (default)"
