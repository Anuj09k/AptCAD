#!/usr/bin/env bash
# Regenerate AptCAD's logo files from assets/brand/aptcad-logo-source.png (Aptimizer's logo):
# the README logo (assets/brand/aptcad-logo.png) and the app icons, which use the logo's emblem
# (the A with the pencil and rulers) on a rounded tile, because the wordmark can't be read at
# icon sizes: aptcad-1024/256/64.png, the hicolor PNGs Linux desktops read, aptcad.ico for
# Windows, and aptcad-macos-512.png and aptcad.icns for macOS. Needs Python with Pillow, on any
# OS. The outputs are committed, so builds and packaging never need these tools.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
python3 - "$ROOT" <<'PY'
import os, sys
from PIL import Image, ImageDraw
root = sys.argv[1]
icons, brand, app_id = f"{root}/assets/app-icon", f"{root}/assets/brand", "ai.storyteller.aptcad"
im = Image.open(f"{brand}/aptcad-logo-source.png").convert("RGB")
# The source is on textured paper: flatten anything close to the paper colour to that colour,
# so small sizes stay crisp.
BG = (250, 241, 231)
px = im.load()
for y in range(im.height):
    for x in range(im.width):
        c = px[x, y]
        if max(abs(c[0] - BG[0]), abs(c[1] - BG[1]), abs(c[2] - BG[2])) <= 22:
            px[x, y] = BG
# Boxes in the 2048 x 1536 source: the whole logo, and the emblem above the wordmark.
FULL, EMBLEM = (390, 170, 1660, 1430), (636, 226, 1440, 978)
full = im.crop(FULL)
full.resize((full.width * 3 // 5, full.height * 3 // 5), Image.LANCZOS).save(f"{brand}/aptcad-logo.png", optimize=True)
S = 1024
inset, radius = round(S * 8 / 256), round(S * 52 / 256)
rect = (inset * 4, inset * 4, (S - inset) * 4 - 1, (S - inset) * 4 - 1)
mask = Image.new("L", (S * 4, S * 4), 0)
ImageDraw.Draw(mask).rounded_rectangle(rect, radius * 4, fill=255)
edge = Image.new("L", (S * 4, S * 4), 0)
ImageDraw.Draw(edge).rounded_rectangle(rect, radius * 4, outline=255, width=16)
tile = Image.new("RGBA", (S, S), (0, 0, 0, 0))
tile.paste(Image.new("RGBA", (S, S), BG + (255,)), (0, 0), mask.resize((S, S), Image.LANCZOS))
tile.paste(Image.new("RGBA", (S, S), (0x1F, 0x35, 0x5E, 255)), (0, 0), edge.resize((S, S), Image.LANCZOS).point(lambda v: v * 70 // 255))
emblem = im.crop(EMBLEM)
k = (S - 2 * inset - round(S * 0.16)) / max(emblem.size)
emblem = emblem.resize((round(emblem.width * k), round(emblem.height * k)), Image.LANCZOS)
tile.paste(emblem, ((S - emblem.width) // 2, (S - emblem.height) // 2))
tile.save(f"{icons}/aptcad-1024.png", optimize=True)
size = lambda n: tile.resize((n, n), Image.LANCZOS)
size(256).save(f"{icons}/aptcad-256.png", optimize=True)
size(64).save(f"{icons}/aptcad-64.png", optimize=True)
for n in (16, 24, 32, 48, 64, 128, 256, 512):
    os.makedirs(f"{icons}/hicolor/{n}x{n}/apps", exist_ok=True)
    size(n).save(f"{icons}/hicolor/{n}x{n}/apps/{app_id}.png", optimize=True)
# macOS draws app icons on Apple's grid, an 824 px body on a 1024 px canvas, where the tile fills
# 960: shrunk to 824 and centred, it sits as large as the Dock's other icons.
body = round(1024 * 824 / 960)
mac = Image.new("RGBA", (1024, 1024), (0, 0, 0, 0))
mac.paste(size(body), ((1024 - body) // 2, (1024 - body) // 2))
mac.resize((512, 512), Image.LANCZOS).save(f"{icons}/aptcad-macos-512.png", optimize=True)
mac.save(f"{icons}/aptcad.icns", format="ICNS")
tile.save(f"{icons}/aptcad.ico", format="ICO", sizes=[(n, n) for n in (16, 20, 24, 32, 40, 48, 64, 128, 256)])
PY
echo "logo and icons written"
