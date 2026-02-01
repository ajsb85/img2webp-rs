# img2webp-rs

An Enterprise Grade Rust migration of the `img2webp` utility from `libwebp`. This project provides a safe, high-performance library and CLI for creating transparent WebP animations with advanced optimization controls.

## Features

- **Safe Rust Wrapper**: High-level API for `libwebp` animation encoding.
- **Advanced Optimization**: Support for alpha quality, predictive filtering, and various compression methods.
- **Pixel Perfect**: Support for the `-exact` flag to preserve RGB values in transparent areas.
- **Interactive Demo**: Built-in HTML benchmark to compare different optimization strategies.

## Getting Started

### Prerequisites

- Rust (latest stable)
- libwebp development headers

### Build

```bash
cd rs
cargo build --release
```

### Usage

```bash
./rs/target/release/img2webp -o animation.webp -loop 0 -d 100 frames/*.webp
```

## Benchmarks

See `demo.html` for a side-by-side comparison of different optimization flags and their impact on file size and transparency quality.
