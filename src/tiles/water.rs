use super::TileDefinition;

// Water tile IDs: 200-299
pub const WATER_DEEP: TileDefinition = TileDefinition::new(
    201,
    "water_deep",
    "water_biome_1x_16x16.png",
    0,
    false, // Can't walk on water
);

// Export all water tiles as array for registry
pub const WATER_TILES: &[TileDefinition] = &[WATER_DEEP];
