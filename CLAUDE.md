# PkmnFusion - Project Instructions

A Pokémon-style fusion game built with Bevy and Rust.

## Project Overview

Gen 2 Pokémon-inspired game with:
- Pixel art aesthetic (16x16 tiles)
- Grid-based movement
- Multiple biomes (grass, water, etc.)
- Tile ID system for extensibility

## Code Structure

```
src/
├── main.rs           # App setup, entry point
├── components.rs     # Shared ECS components
├── tiles/            # Tile system
│   ├── mod.rs       # Registry + TileDefinition
│   ├── grass.rs     # Grass tiles (IDs 100-199)
│   └── water.rs     # Water tiles (IDs 200-299)
├── map.rs           # Map spawning, tile placement
└── player.rs        # Player movement, camera, collision
```

## Tile ID System

**Stable IDs** - Never change, used in maps:
```rust
pub const GRASS_PLAIN_1: TileDefinition = TileDefinition::new(
    101,                           // Stable ID
    "grass_plain_1",               // Name
    "grass_biome_8x_16x16.png",    // Atlas file
    0,                             // Atlas index (can change)
    true,                          // Walkable
);
```

**Benefits:**
- Add tiles without breaking maps
- Reorganize atlases freely
- Descriptive tile names

## Asset Naming Convention

```
<biome>_biome_<columns>x<tile_size>.png

Examples:
- grass_biome_8x_16x16.png   (8 tiles, 16x16 each)
- water_biome_1x_16x16.png   (1 tile, 16x16)
```

Atlases can be 1D (1x8) or 2D (4x8) grids. Index is linear.

## Adding New Tiles

1. Add to atlas PNG
2. Define in `tiles/grass.rs` (or new biome file):
```rust
pub const NEW_TILE: TileDefinition = TileDefinition::new(
    109,                      // Next available ID
    "descriptive_name",
    "grass_biome_8x_16x16.png",
    8,                        // Index in atlas
    true,                     // Walkable
);
```
3. Add to `GRASS_TILES` array
4. Use in maps: `map_data.set_tile(x, y, NEW_TILE.id)`

## Map Layout

- Size: 16x16 tiles (configurable in `map.rs`)
- Coordinate system: (0,0) = bottom-left
- Multiple tilemap layers auto-generated per atlas

## Controls

- WASD: Move player
- Z/X: Zoom in/out

## Coding Style

- Concise, no unnecessary abstractions
- Use Bevy ECS patterns (components, systems, resources)
- Pixel-perfect rendering (nearest-neighbor filtering)
- Grid-aligned positions (16px steps)

## Future Ideas

Track in GitHub issues with `enhancement` or `idea` labels:
- Animation system
- Tile transitions (borders)
- Wild encounters
- Battle system
- NPC dialogues
- Weather effects
