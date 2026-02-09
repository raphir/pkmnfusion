"""
Generates a placeholder tileset for PkmnFusion.

Each tile is 16x16 px.  Output is a single horizontal strip: 96x16 (6 tiles).

Tile indices (must stay in sync with main.rs constants):
    0 — WATER      purple-blue
    1 — GRASS      bright green
    2 — PATH       pale cream-green
    3 — SAND       golden brown
    4 — FENCE      mid gray
    5 — RAILROAD   dark slate gray

Run:
    tools/venv/bin/python tools/generate_tileset.py
"""

from pathlib import Path
from PIL import Image

TILE_SIZE = 16
TILES: list[tuple[str, tuple[int, int, int]]] = [
    ("WATER", (107, 127, 212)),
    ("GRASS", (123, 192, 58)),
    ("PATH", (224, 232, 200)),
    ("SAND", (200, 160, 80)),
    ("FENCE", (150, 150, 150)),
    ("RAILROAD", (96, 125, 139)),
]

def main() -> None:
    width = TILE_SIZE * len(TILES)
    img = Image.new("RGBA", (width, TILE_SIZE))

    for i, (name, color) in enumerate(TILES):
        for py in range(TILE_SIZE):
            for px in range(TILE_SIZE):
                img.putpixel((i * TILE_SIZE + px, py), (*color, 255))

    out = Path(__file__).resolve().parent.parent / "assets" / "tileset.png"
    out.parent.mkdir(parents=True, exist_ok=True)
    img.save(out)
    print(f"Wrote {out}  ({width}x{TILE_SIZE}, {len(TILES)} tiles)")

if __name__ == "__main__":
    main()
