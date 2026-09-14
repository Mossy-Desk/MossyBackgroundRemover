# Mossy Background Remover

A Tauri desktop app that removes an image's background entirely on-device:
no cloud API, no upload, nothing leaves your machine. Part of the
[MossyDesk](https://github.com/Mossy-Desk) app suite.

Background removal is powered by **[U²-Net](https://github.com/xuebinqin/U-2-Net)**
(specifically its lightweight **u2netp** variant), a neural network this app
does not train or own.
See [Credits](#credits) below for full attribution.

# Credits

This app embeds a third-party, open-source neural network to do the actual
background removal:

- **Model**: [U²-Net](https://github.com/xuebinqin/U-2-Net) (u2netp variant)
  — Xuebin Qin, Zichen Zhang, Chenyang Huang, Masood Dehghan, Osmar Zaiane,
  Martin Jagersand. *"U²-Net: Going Deeper with Nested U-Structure for
  Salient Object Detection"*, Pattern Recognition, 2020. Licensed under the
  [Apache License 2.0](https://github.com/xuebinqin/U-2-Net/blob/master/LICENSE).
  ```bibtex
  @InProceedings{Qin_2020_PR,
  title = {U2-Net: Going Deeper with Nested U-Structure for Salient Object Detection},
  author = {Qin, Xuebin and Zhang, Zichen and Huang, Chenyang and Dehghan, Masood and Zaiane, Osmar and Jagersand, Martin},
  journal = {Pattern Recognition},
  volume = {106},
  pages = {107404},
  year = {2020}
  }
  ```
- **Model file**: the `.onnx` build embedded in this app
  (`mossbgr/src-tauri/models/u2netp.onnx`) is sourced from
  [danielgatis/rembg](https://github.com/danielgatis/rembg)'s GitHub
  Releases, not converted by this project. rembg is licensed under the
  [MIT License](https://github.com/danielgatis/rembg/blob/main/LICENSE.txt).

No part of the model's weights or architecture was created by this project: all credit for the background-removal capability itself belongs to the
U²-Net authors above.

# Features
- **Local background removal**: runs a u2netp ONNX segmentation model on
  your own machine via the `ort` crate; images are never sent anywhere.
- **No download, ready instantly**: the ~4.4MB lightweight model is
  embedded in the app itself, not fetched on first run.
- **Before/after preview**: see the original and the result side by side.
- **Export as PNG** — save the result with a transparent background.

v1 handles one PNG/JPEG image at a time; batch/folder processing is a future addition, not part of this release.

# Installation

## Prerequisites
- **Node.js** 18+ and npm (for frontend)
- **Rust** 1.70+ (for backend)
- **Tauri CLI** (auto-installed by npm during setup)

## Setup

Clone the repository and install dependencies:

```bash
git clone <repository-url>
cd MossyBackgroundRemover
cd mossbgr
npm install
```

# Usage

## Quick Start

Launch the app in dev mode (watch mode — Rust changes rebuild and restart
the app, frontend changes hot-reload):

```bash
./scripts/testapp.sh
```

## Available Scripts

From the repository root, the following helper scripts are available:

### `./scripts/testapp.sh`
Runs the app in dev/watch mode (`npm run tauri:dev`). Edit Rust or frontend
code with the app open and it picks up the change automatically.
```bash
./scripts/testapp.sh
```

### `./scripts/quickie.sh`
Runs format check and clippy on both frontend and backend — useful during
local iteration.
```bash
./scripts/quickie.sh
```

### `./scripts/pre-push-check.sh`
Runs the full CI gate (frontend check/build, backend fmt/clippy/test/build).
Run this before pushing to verify your changes pass CI.
```bash
./scripts/pre-push-check.sh
```

### `./scripts/security-check.sh`
Runs npm audit, cargo audit, and cargo deny to check for known
vulnerabilities.
```bash
./scripts/security-check.sh
```

## Manual Commands

### Frontend (from `mossbgr/`)
```bash
npm run dev              # SvelteKit dev server (used by tauri:dev)
npm run tauri:dev        # Launch Tauri app in dev mode
npm run check            # TypeScript type-check
npm run format            # Format with Prettier and Svelte-check
npm run build            # Production build
```

### Backend (from `mossbgr/src-tauri/`)
```bash
cargo test --all-features                    # Run all tests
cargo test <test_name>                       # Run a specific test
cargo fmt --all -- --check                   # Format check
cargo clippy --all-targets --all-features -- -D warnings  # Lint
cargo build --release                        # Production build
```

## Using the app

1. **Choose an image** — pick a PNG or JPEG from disk.
2. **Remove background** — runs the model locally and shows a before/after
   preview.
3. **Export** — save the result as a PNG with a transparent background.

# Contributing
Feel free to open a pull request or an issue — bug fixes, features, and
feedback are all welcome.
