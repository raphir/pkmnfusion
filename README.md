# PkmnFusion

A Pokémon-style fusion game built with Bevy and Rust, featuring Gen 2 pixel art aesthetics.

## Features

- 🎮 Grid-based movement system
- 🗺️ Multiple biomes (grass, water)
- 🎨 Pixel-perfect 16x16 tile rendering
- 🔧 Extensible tile ID system
- 📦 Modular code architecture

## Tech Stack

- **Engine:** [Bevy 0.18](https://bevyengine.org/)
- **Tilemap:** [bevy_ecs_tilemap 0.18](https://github.com/StarArawn/bevy_ecs_tilemap)
- **Language:** Rust (2024 edition)

## Building & Running

```bash
# Development build
cargo run

# Optimized build
cargo run --release
```

## Controls

- **WASD** - Move player
- **Z/X** - Zoom in/out

## Project Structure

```
src/
├── main.rs          # App entry point
├── components.rs    # ECS components
├── tiles/          # Tile definitions & registry
├── map.rs          # Map generation
└── player.rs       # Player systems
```

## Development

See [CLAUDE.md](CLAUDE.md) for detailed project documentation and development guidelines.

## License

[Add license info]
