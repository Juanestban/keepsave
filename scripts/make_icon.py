"""Draws the KeepSave icon and writes it to crates/keepsave-app/assets/icon/.

Uses only the Python standard library, so it runs anywhere:

    python scripts/make_icon.py

Outputs:
- keepsave.ico: 16, 24, 32, 48, 64 and 128 px as 32-bit bitmaps, 256 px as PNG.
- keepsave.png: 256 px, for docs.
- keepsave-128.rgba: raw 128x128 RGBA, the window icon (no decoder needed).
"""

import math
import struct
import zlib
from pathlib import Path

OUT = Path(__file__).resolve().parent.parent / "crates" / "keepsave-app" / "assets" / "icon"
SIZES = [16, 24, 32, 48, 64, 128, 256]
SUPERSAMPLE = 4

# Theme colors (see theme.rs): accent to violet.
TOP_LEFT = (132, 160, 255)
BOTTOM_RIGHT = (150, 70, 225)
KEYHOLE = (12, 16, 36)
FACE = (246, 248, 255)


def rounded_box(x, y, half, radius):
    """Signed distance to a rounded square centered at 0, side 2*half."""
    qx = abs(x) - half + radius
    qy = abs(y) - half + radius
    outside = math.hypot(max(qx, 0.0), max(qy, 0.0))
    return outside + min(max(qx, qy), 0.0) - radius


def keyhole(x, y):
    """Signed distance to a keyhole: a circle over a tapered stem."""
    circle = math.hypot(x, y + 0.12) - 0.15
    # Stem: a trapezoid from y=-0.02 to y=0.30, wider at the bottom.
    t = min(max((y + 0.02) / 0.32, 0.0), 1.0)
    half_width = 0.055 + 0.04 * t
    # Negative inside; only the sign matters for the coverage test.
    stem = max(abs(x) - half_width, -0.02 - y, y - 0.30)
    return min(circle, stem)


def shade(x, y):
    """Color of the point (x, y) in [-1, 1]^2, premultiplied RGBA floats."""
    body = rounded_box(x, y, 0.92, 0.42)
    if body > 0:
        return (0.0, 0.0, 0.0, 0.0)
    t = (x + y + 2.0) / 4.0
    color = [TOP_LEFT[i] + (BOTTOM_RIGHT[i] - TOP_LEFT[i]) * t for i in range(3)]
    # Glass: a soft light band near the top.
    glow = max(0.0, 1.0 - (y + 0.92) / 0.9) ** 2 * 0.22
    color = [c + (255 - c) * glow for c in color]
    # A thin bright rim.
    if body > -0.035:
        color = [c + (255 - c) * 0.25 for c in color]
    # The disc behind the keyhole.
    if math.hypot(x, y) < 0.52:
        color = list(FACE)
        if keyhole(x, y) < 0:
            color = list(KEYHOLE)
    return (color[0] / 255, color[1] / 255, color[2] / 255, 1.0)


def render(size):
    """Size x size straight-alpha RGBA bytes."""
    pixels = bytearray()
    n = SUPERSAMPLE
    for py in range(size):
        for px in range(size):
            acc = [0.0, 0.0, 0.0, 0.0]
            for sy in range(n):
                for sx in range(n):
                    x = ((px + (sx + 0.5) / n) / size) * 2 - 1
                    y = ((py + (sy + 0.5) / n) / size) * 2 - 1
                    r, g, b, a = shade(x, y)
                    acc[0] += r * a
                    acc[1] += g * a
                    acc[2] += b * a
                    acc[3] += a
            alpha = acc[3] / (n * n)
            if alpha > 0:
                rgb = [round(255 * c / acc[3]) for c in acc[:3]]
            else:
                rgb = [0, 0, 0]
            pixels += bytes(rgb + [round(255 * alpha)])
    return bytes(pixels)


def png(size, rgba):
    rows = b"".join(b"\x00" + rgba[y * size * 4 : (y + 1) * size * 4] for y in range(size))

    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data))

    header = struct.pack(">IIBBBBB", size, size, 8, 6, 0, 0, 0)
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", header) + chunk(b"IDAT", zlib.compress(rows, 9)) + chunk(b"IEND", b"")


def bmp(size, rgba):
    """An ICO bitmap entry: BITMAPINFOHEADER, bottom-up BGRA, AND mask."""
    header = struct.pack("<IiiHHIIiiII", 40, size, size * 2, 1, 32, 0, 0, 0, 0, 0, 0)
    pixels = bytearray()
    for y in reversed(range(size)):
        for x in range(size):
            r, g, b, a = rgba[(y * size + x) * 4 : (y * size + x) * 4 + 4]
            pixels += bytes([b, g, r, a])
    mask_row = ((size + 31) // 32) * 4
    return header + bytes(pixels) + bytes(mask_row * size)


def ico(images):
    entries = b""
    data = b""
    offset = 6 + 16 * len(images)
    for size, blob in images:
        dim = 0 if size >= 256 else size
        entries += struct.pack("<BBBBHHII", dim, dim, 0, 0, 1, 32, len(blob), offset)
        data += blob
        offset += len(blob)
    return struct.pack("<HHH", 0, 1, len(images)) + entries + data


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    rendered = {size: render(size) for size in SIZES}
    images = [(size, png(size, rendered[size]) if size >= 256 else bmp(size, rendered[size])) for size in SIZES]
    (OUT / "keepsave.ico").write_bytes(ico(images))
    (OUT / "keepsave.png").write_bytes(png(256, rendered[256]))
    (OUT / "keepsave-128.rgba").write_bytes(rendered[128])
    print(f"wrote {OUT}")


if __name__ == "__main__":
    main()
