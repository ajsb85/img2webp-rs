# img2webp-rs 🦀

[![GitHub Pages](https://img.shields.io/badge/Live-Demo-blue?style=for-the-badge&logo=github)](https://ajsb85.github.io/img2webp-rs/) [![Release](https://img.shields.io/github/v/release/ajsb85/img2webp-rs?style=for-the-badge)](https://github.com/ajsb85/img2webp-rs/releases)

An **Enterprise Grade Rust** migration of the Google `img2webp` utility. This project provides a robust, safe, and highly optimized library and CLI for creating transparent WebP animations with granular control over compression, alpha-blending, and pixel integrity.

## 🌟 Live Preview & Benchmarks

Explore the interactive comparison of different optimization strategies:
👉 **[https://ajsb85.github.io/img2webp-rs/](https://ajsb85.github.io/img2webp-rs/)**

## 🖼️ Showcase: Optimized Small

Below is a live example of a transparent animation generated using the **Optimized Small** strategy. It leverages `-mixed` mode and `-alpha_q 50` to achieve a ~80% reduction in file size while maintaining excellent visual quality for transparent edges.

![Optimized Small Animation](animated_small.webp)

**Command used to generate this file:**
```bash
img2webp -o animated_small.webp -loop 0 -d 100 -min_size -mixed -alpha_q 50 frames/*.webp
```

---

## 🚀 Features

- **Memory Safe Migration**: Replaces the original C codebase with a modular Rust workspace.
- **Advanced Transparency Handling**: Deep integration with WebP's `alpha_filter`, `alpha_quality`, and `exact` RGB preservation.
- **Pixel-Perfect Integrity**: Use the `-exact` flag to prevent color bleeding on transparent edges—essential for professional UI and game assets.
- **Enhanced Format Support**: Unlike the original C tool, this version supports **PNG, JPEG, TIFF, GIF, BMP, WebP, and more** out of the box via the Rust `image` crate.
- **Production Ready**: Full GPG signing for releases and tags, automated with a comprehensive `Makefile`.

---

## 🛠 Installation

### From Source

```bash
git clone https://github.com/ajsb85/img2webp-rs.git
cd img2webp-rs/rs
cargo build --release
```

The binary will be available at `./rs/target/release/img2webp`.

---

## 📖 CLI Usage & Options

`img2webp` uses a two-pass argument parser. **Global options** apply to the entire animation, while **Frame options** apply to the *subsequent* input files.

### Basic Example
```bash
img2webp -o animation.webp -loop 0 -d 100 frame01.png frame02.png frame03.png
```

### Advanced usage with Mixed Options
```bash
img2webp -o optimized.webp -mixed -near_lossless 90 \
  -d 200 frame_start.png \
  -lossy -q 50 -d 50 frame_fast_action_*.png \
  -lossless frame_end.png
```

### All CLI Options

| Option | Type | Description | 
| :--- | :--- | :--- | 
| **Global Options** | | | 
| `-o <path>` | String | Path to the output WebP file. | 
| `-min_size` | Flag | Minimize output size (intensive search). | 
| `-mixed` | Flag | Automatically choose between lossy/lossless for each frame. | 
| `-loop <int>` | Integer | Number of times to loop (0 = infinite). | 
| `-near_lossless <int>` | 0-100 | Near-lossless preprocessing (100 = off). | 
| `-alpha_q <int>` | 0-100 | Alpha channel quality. | 
| `-alpha_method <int>` | 0-1 | Alpha compression method (0 = none, 1 = lossless). | 
| `-alpha_filter <int>` | 0-2 | Alpha predictive filter (0 = none, 1 = fast, 2 = best). | 
| **Frame Options** | | | 
| `-d <int>` | ms | Duration of the *next* frames in milliseconds. | 
| `-lossless` | Flag | Switch to lossless encoding for subsequent frames (Default). | 
| `-lossy` | Flag | Switch to lossy encoding for subsequent frames. | 
| `-q <float>` | 0-100 | Quality factor for lossy frames. | 
| `-m <int>` | 0-6 | Compression method (0 = fast, 6 = slowest/best). | 
| `-exact` | Flag | Preserve RGB values in transparent areas (prevents "dark edges"). | 
| `-noexact` | Flag | Discard RGB info in transparent areas for better compression. | 

---

## 🧪 Benchmark Comparison Logic

The [Live Demo](https://ajsb85.github.io/img2webp-rs/) compares five distinct encoding strategies:

1.  **Standard**: The default lossless behavior. Good balance of size and speed.
2.  **Pixel Perfect**: Uses `-lossless -exact`. This is the gold standard for transparent animations, ensuring that even the "invisible" pixels maintain their color data to prevent artifacts during browser rendering.
3.  **Extreme Slow (M6)**: Uses compression method `6` and the "Best" alpha filter. It puts maximum CPU effort into finding the smallest possible bitstream.
4.  **Optimized Small**: Uses `-mixed` mode and reduced alpha quality (`-alpha_q 50`). This demonstrates how much space can be saved (~80% reduction) while maintaining acceptable visual quality.
5.  **Lossy Sharp**: Uses `-lossy` with `-sharp_yuv`. This targets high-contrast edges in lossy mode to reduce the "blurring" typically seen in animated WebPs.

### Interactive Controls in Demo:
- **Background Toggles**: Switch to **Solid Black** or **Checkered** to see how antialiasing behaves against high-contrast backgrounds.
- **Rendering Toggle**: Switch to **Pixel Perfect** (CSS `image-rendering: pixelated`) to inspect individual pixel boundaries without browser smoothing.

---

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

### Makefile Shortcuts
- `make build`: Build the release binary.
- `make check`: Run lints (`clippy`) and tests.
- `make deploy`: Update the GitHub Pages site.
- `make release VERSION=vX.Y.Z`: Create a signed GPG release.

---

## 📄 License

This project is licensed under the same terms as the original `libwebp` (BSD-style). See the original project for details.