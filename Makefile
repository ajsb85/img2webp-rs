# Project Configuration
BINARY_NAME = img2webp
BINARY_PATH = rs/target/release/$(BINARY_NAME)
ASSETS_DIR = assets
OUTPUT_FILES = $(ASSETS_DIR)/animated.webp $(ASSETS_DIR)/animated_perfect.webp $(ASSETS_DIR)/animated_lossy.webp \
               $(ASSETS_DIR)/animated_slowest.webp $(ASSETS_DIR)/animated_small.webp $(ASSETS_DIR)/animated_near_lossless60.webp \
               $(ASSETS_DIR)/animated_lossy_low.webp $(ASSETS_DIR)/animated_backward.webp $(ASSETS_DIR)/animated_pingpong.webp
FRAMES = frames/frame_*.webp

# Multi-platform targets
LINUX_TARGET = x86_64-unknown-linux-gnu
WINDOWS_TARGET = x86_64-pc-windows-gnu
MACOS_TARGET = x86_64-apple-darwin

LINUX_BINARY = $(BINARY_NAME)-linux-x64
WINDOWS_BINARY = $(BINARY_NAME)-windows-x64.exe
MACOS_BINARY = $(BINARY_NAME)-macos-x64

# Default target
.PHONY: all
all: build generate

# Build the Rust tool natively
.PHONY: build
build:
	@echo "Building img2webp-rs natively..."
	@cd rs && cargo build --release

# Cross-compile Windows binary
.PHONY: build-windows
build-windows:
	@echo "Cross-compiling for Windows (x64)..."
	@cd rs && cargo build --release --target $(WINDOWS_TARGET)

# Cross-compile macOS binary using zigbuild
.PHONY: build-macos
build-macos:
	@echo "Cross-compiling for macOS (x64) using zigbuild..."
	@cd rs && cargo zigbuild --release --target $(MACOS_TARGET)

# Generate all animation variants
.PHONY: generate
generate: build
	@echo "Generating animations into $(ASSETS_DIR)/..."
	@mkdir -p $(ASSETS_DIR)
	@$(BINARY_PATH) -o $(ASSETS_DIR)/animated.webp -loop 0 -d 100 $(FRAMES)
	@$(BINARY_PATH) -o $(ASSETS_DIR)/animated_perfect.webp -loop 0 -d 100 -lossless -exact $(FRAMES)
	@$(BINARY_PATH) -o $(ASSETS_DIR)/animated_lossy.webp -loop 0 -d 100 -lossy -q 75 -sharp_yuv $(FRAMES)
	@$(BINARY_PATH) -o $(ASSETS_DIR)/animated_small.webp -loop 0 -d 100 -min_size -mixed -alpha_q 50 $(FRAMES)
	@$(BINARY_PATH) -o $(ASSETS_DIR)/animated_slowest.webp -loop 0 -d 100 -m 6 -alpha_filter 2 -exact $(FRAMES)
	@$(BINARY_PATH) -o $(ASSETS_DIR)/animated_near_lossless60.webp -loop 0 -d 100 -near_lossless 60 $(FRAMES)
	@$(BINARY_PATH) -o $(ASSETS_DIR)/animated_lossy_low.webp -loop 0 -d 100 -lossy -q 30 -alpha_q 30 $(FRAMES)
	@$(BINARY_PATH) -o $(ASSETS_DIR)/animated_backward.webp -loop 0 -d 100 -reverse $(FRAMES)
	@$(BINARY_PATH) -o $(ASSETS_DIR)/animated_pingpong.webp -loop 0 -d 100 -pingpong $(FRAMES)

# Quality Control
.PHONY: check
check:
	@cd rs && cargo clippy --fix --allow-dirty --allow-staged -- -D warnings
	@cd rs && cargo fmt
	@cd rs && cargo test

# Clean
.PHONY: clean
clean:
	@cd rs && cargo clean
	@rm -rf $(ASSETS_DIR)
	@rm -f $(LINUX_BINARY)* $(WINDOWS_BINARY)* $(MACOS_BINARY)*

# Deploy GitHub Pages
.PHONY: deploy
deploy: check generate
	@git add .gitignore index.html .nojekyll $(ASSETS_DIR)
	@git commit -m "deploy: update assets and fix paths" || echo "No changes"
	@git push origin main

# FULL MULTI-PLATFORM RELEASE (Linux, Windows, macOS)
# Usage: make release VERSION=v1.0.0
.PHONY: release
release:
	@if [ -z "$(VERSION)" ]; then echo "Error: VERSION is not set."; exit 1; fi
	@echo "Performing pre-release checks..."
	@make check
	@echo "Building all platform binaries..."
	@make build
	@make build-windows
	@make build-macos
	@cp rs/target/release/$(BINARY_NAME) $(LINUX_BINARY)
	@cp rs/target/$(WINDOWS_TARGET)/release/$(BINARY_NAME).exe $(WINDOWS_BINARY)
	@cp rs/target/$(MACOS_TARGET)/release/$(BINARY_NAME) $(MACOS_BINARY)
	@echo "Signing tag $(VERSION)..."
	@git tag -s $(VERSION) -m "Release $(VERSION)" || (git tag -d $(VERSION) && git tag -s $(VERSION) -m "Release $(VERSION)")
	@git push origin $(VERSION) --force
	@echo "Signing binaries..."
	@gpg --armor --detach-sign $(LINUX_BINARY)
	@gpg --armor --detach-sign $(WINDOWS_BINARY)
	@gpg --armor --detach-sign $(MACOS_BINARY)
	@echo "Publishing release to GitHub with multi-platform artifacts..."
	@gh release create $(VERSION) \
		$(LINUX_BINARY) $(LINUX_BINARY).asc \
		$(WINDOWS_BINARY) $(WINDOWS_BINARY).asc \
		$(MACOS_BINARY) $(MACOS_BINARY).asc \
		--title "$(VERSION)" \
		--notes "img2webp-rs stable release $(VERSION). Native binaries for Linux (x64), Windows (x64), and macOS (x64). GPG Signed."
	@echo "Multi-platform release $(VERSION) complete."

# Help target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make build           - Build the Rust tool (native)"
	@echo "  make build-windows   - Cross-compile for Windows"
	@echo "  make build-macos     - Cross-compile for macOS"
	@echo "  make deploy          - Push site and assets to GH Pages"
	@echo "  make release VERSION=v1.0.0 - Tag, Build (All), Sign, and Publish"
