#!/usr/bin/env bash
# Regenerate the app icons from AptCad.png (macOS: sips, iconutil).
# The outputs are committed, so builds and packaging never need these tools.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
SRC="$DIR/aptcad-1024.png"
ID="ai.storyteller.aptcad"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
command -v sips >/dev/null || { echo "error: sips not found (macOS only)" >&2; exit 1; }

resize() {
  local src="$1" size="$2" out="$3"
  sips -s format png -z "$size" "$size" "$src" --out "$out" >/dev/null
}

# AptCad.png is 4:3. The centered 1:1 crop keeps the entire mark and wordmark; the full image
# remains the README/brand logo, while square variants are used by native app icon slots.
sips -s format png -c 1536 1536 "$ROOT/AptCad.png" --out "$TMP/logo-square.png" >/dev/null
resize "$TMP/logo-square.png" 1024 "$SRC"
resize "$SRC" 512 "$DIR/aptcad-macos-512.png"

for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/apps"
  resize "$SRC" "$s" "$DIR/hicolor/${s}x${s}/apps/$ID.png"
done
resize "$SRC" 64 "$DIR/aptcad-64.png"
resize "$SRC" 256 "$DIR/aptcad-256.png"

ICO_PNGS=()
for s in 16 20 24 32 40 48 64 128 256; do
  resize "$SRC" "$s" "$TMP/ico-$s.png"
  ICO_PNGS+=("$TMP/ico-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- ico "$DIR/aptcad.ico" "${ICO_PNGS[@]}")

# The macOS iconset is built from the same square master; macOS applies its rounded presentation.
SET="$TMP/aptcad.iconset"
mkdir -p "$SET"
for s in 16 32 128 256; do
  resize "$DIR/aptcad-macos-512.png" "$s" "$SET/icon_${s}x${s}.png"
  resize "$DIR/aptcad-macos-512.png" $((s * 2)) "$SET/icon_${s}x${s}@2x.png"
done
resize "$DIR/aptcad-macos-512.png" 512 "$SET/icon_512x512.png"
iconutil -c icns -o "$DIR/aptcad.icns" "$SET"
echo "icons written to $DIR"
