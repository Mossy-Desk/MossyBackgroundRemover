#!/usr/bin/env bash
# Builds distributable installer bundles (.deb/.rpm on Linux, .msi/.nsis on
# Windows, .dmg on macOS — whichever bundlers are available on this machine)
# after running the same checks CI runs. No AppImage: bundling one triples
# build time and ships a full copy of WebKitGTK for a ~90MB portable binary,
# which isn't worth it over a standard installer.
#
# Usage:
#   ./scripts/build-release.sh              # full CI gate + release build
#   ./scripts/build-release.sh --skip-checks  # skip fmt/clippy/test, just build
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR/mossbgr"

SKIP_CHECKS=false
if [[ "${1:-}" == "--skip-checks" ]]; then
  SKIP_CHECKS=true
fi

echo "== Checking version consistency =="
PKG_VERSION=$(node -p "require('./package.json').version")
TAURI_VERSION=$(node -p "require('./src-tauri/tauri.conf.json').version")
CARGO_VERSION=$(grep -m1 '^version' src-tauri/Cargo.toml | sed -E 's/version = "(.*)"/\1/')

echo "  package.json:     $PKG_VERSION"
echo "  tauri.conf.json:  $TAURI_VERSION"
echo "  Cargo.toml:       $CARGO_VERSION"

if [[ "$PKG_VERSION" != "$TAURI_VERSION" || "$PKG_VERSION" != "$CARGO_VERSION" ]]; then
  echo "ERROR: version mismatch across package.json / tauri.conf.json / Cargo.toml" >&2
  echo "Bump all three to the same version before releasing." >&2
  exit 1
fi
VERSION="$PKG_VERSION"
echo "Building release v$VERSION"

npm ci

if [[ "$SKIP_CHECKS" == false ]]; then
  echo "== [1/3] Frontend typecheck =="
  npm run check

  echo "== [2/3] Rust format/lint/test =="
  (cd src-tauri && cargo fmt --all -- --check)
  (cd src-tauri && cargo clippy --all-targets --all-features -- -D warnings)
  (cd src-tauri && cargo test --all-features)
else
  echo "== Skipping checks (--skip-checks) =="
fi

BUNDLE_DIR="src-tauri/target/release/bundle"
# Wipe stale bundles from a previous run/config (e.g. a leftover .AppImage
# from before a targets change) so dist/ never picks up something the
# current build didn't actually produce.
rm -rf "$BUNDLE_DIR"

echo "== [3/3] Tauri release build =="
npm run tauri:build

DIST_DIR="$ROOT_DIR/dist/v$VERSION"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

while IFS= read -r -d '' f; do
  cp "$f" "$DIST_DIR/"
done < <(find "$BUNDLE_DIR" -maxdepth 2 -type f \
  \( -name "*.deb" -o -name "*.rpm" \
     -o -name "*.dmg" -o -name "*.msi" -o -name "*.exe" \) -print0 2>/dev/null)

echo
echo "== Release artifacts =="
ls -lh "$DIST_DIR"
echo
echo "Copied to: $DIST_DIR"
echo
echo "Next steps for the v$VERSION release:"
echo "  git tag -a v$VERSION -m \"v$VERSION\""
echo "  git push origin v$VERSION"
echo "  # then create a GitHub Release and attach the files in $DIST_DIR"
