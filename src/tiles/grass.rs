use super::TileDefinition;

// Grass tile IDs: 100-199
pub const GRASS_PLAIN_1: TileDefinition = TileDefinition::new(
    101,
    "grass_plain_1",
    "grass_biome_8x_16x16.png",
    0,
    true,
);

pub const GRASS_PLAIN_2: TileDefinition = TileDefinition::new(
    102,
    "grass_plain_2",
    "grass_biome_8x_16x16.png",
    1,
    true,
);

pub const GRASS_FLOWERS_1: TileDefinition = TileDefinition::new(
    103,
    "grass_flowers_1",
    "grass_biome_8x_16x16.png",
    2,
    true,
);

pub const GRASS_FLOWERS_2: TileDefinition = TileDefinition::new(
    104,
    "grass_flowers_2",
    "grass_biome_8x_16x16.png",
    3,
    true,
);

pub const GRASS_TALL_1: TileDefinition = TileDefinition::new(
    105,
    "grass_tall_1",
    "grass_biome_8x_16x16.png",
    4,
    true,
);

pub const GRASS_TALL_2: TileDefinition = TileDefinition::new(
    106,
    "grass_tall_2",
    "grass_biome_8x_16x16.png",
    5,
    true,
);

pub const GRASS_VARIANT_1: TileDefinition = TileDefinition::new(
    107,
    "grass_variant_1",
    "grass_biome_8x_16x16.png",
    6,
    true,
);

pub const GRASS_VARIANT_2: TileDefinition = TileDefinition::new(
    108,
    "grass_variant_2",
    "grass_biome_8x_16x16.png",
    7,
    true,
);

// Export all grass tiles as array for registry
pub const GRASS_TILES: &[TileDefinition] = &[
    GRASS_PLAIN_1,
    GRASS_PLAIN_2,
    GRASS_FLOWERS_1,
    GRASS_FLOWERS_2,
    GRASS_TALL_1,
    GRASS_TALL_2,
    GRASS_VARIANT_1,
    GRASS_VARIANT_2,
];
