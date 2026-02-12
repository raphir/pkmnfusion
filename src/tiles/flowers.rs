use super::{TileDefinition, TileTrigger};

// Flower tile IDs: 110-129 (within grass biome range 100-199)

// ===== Orange Flowers (Non-mirrored) =====
pub const ORANGE_FLOWER_1: TileDefinition = TileDefinition::new(
    110,
    "orange_flower_1",
    "grass_biome_4x8_16x16.png",
    2,
    true,
    TileTrigger::None,
);

pub const ORANGE_FLOWER_2: TileDefinition = TileDefinition::new(
    111,
    "orange_flower_2",
    "grass_biome_4x8_16x16.png",
    3,
    true,
    TileTrigger::None,
);

pub const ORANGE_FLOWER_3: TileDefinition = TileDefinition::new(
    112,
    "orange_flower_3",
    "grass_biome_4x8_16x16.png",
    4,
    true,
    TileTrigger::None,
);

// ===== Orange Flowers (Mirrored) =====
pub const ORANGE_FLOWER_MIRROR_1: TileDefinition = TileDefinition::new(
    116,
    "orange_flower_mirror_1",
    "grass_biome_4x8_16x16.png",
    5,  // Adjust based on actual atlas layout
    true,
    TileTrigger::None,
);

pub const ORANGE_FLOWER_MIRROR_2: TileDefinition = TileDefinition::new(
    117,
    "orange_flower_mirror_2",
    "grass_biome_4x8_16x16.png",
    6,
    true,
    TileTrigger::None,
);

pub const ORANGE_FLOWER_MIRROR_3: TileDefinition = TileDefinition::new(
    118,
    "orange_flower_mirror_3",
    "grass_biome_4x8_16x16.png",
    7,
    true,
    TileTrigger::None,
);

// ===== Purple Flowers (Non-mirrored) =====
pub const PURPLE_FLOWER_1: TileDefinition = TileDefinition::new(
    113,
    "purple_flower_1",
    "grass_biome_4x8_16x16.png",
    6,  // Adjusted from user's correction
    true,
    TileTrigger::None,
);

pub const PURPLE_FLOWER_2: TileDefinition = TileDefinition::new(
    114,
    "purple_flower_2",
    "grass_biome_4x8_16x16.png",
    7,
    true,
    TileTrigger::None,
);

pub const PURPLE_FLOWER_3: TileDefinition = TileDefinition::new(
    115,
    "purple_flower_3",
    "grass_biome_4x8_16x16.png",
    8,
    true,
    TileTrigger::None,
);

// ===== Purple Flowers (Mirrored) =====
pub const PURPLE_FLOWER_MIRROR_1: TileDefinition = TileDefinition::new(
    119,
    "purple_flower_mirror_1",
    "grass_biome_4x8_16x16.png",
    9,  // Adjust based on actual atlas layout
    true,
    TileTrigger::None,
);

pub const PURPLE_FLOWER_MIRROR_2: TileDefinition = TileDefinition::new(
    120,
    "purple_flower_mirror_2",
    "grass_biome_4x8_16x16.png",
    10,
    true,
    TileTrigger::None,
);

pub const PURPLE_FLOWER_MIRROR_3: TileDefinition = TileDefinition::new(
    121,
    "purple_flower_mirror_3",
    "grass_biome_4x8_16x16.png",
    11,
    true,
    TileTrigger::None,
);

// ===== Animation Frame Sequences =====
// Use these in the animation system to determine which frames to cycle through
pub const ORANGE_FRAMES: [u32; 4] = [2, 3, 2, 4];
pub const ORANGE_MIRROR_FRAMES: [u32; 4] = [5, 6, 5, 7];
pub const PURPLE_FRAMES: [u32; 4] = [6, 7, 8, 7];
pub const PURPLE_MIRROR_FRAMES: [u32; 4] = [9, 10, 11, 10];

// Export all flower tiles for registry
pub const FLOWER_TILES: &[TileDefinition] = &[
    ORANGE_FLOWER_1,
    ORANGE_FLOWER_2,
    ORANGE_FLOWER_3,
    PURPLE_FLOWER_1,
    PURPLE_FLOWER_2,
    PURPLE_FLOWER_3,
    ORANGE_FLOWER_MIRROR_1,
    ORANGE_FLOWER_MIRROR_2,
    ORANGE_FLOWER_MIRROR_3,
    PURPLE_FLOWER_MIRROR_1,
    PURPLE_FLOWER_MIRROR_2,
    PURPLE_FLOWER_MIRROR_3,
];

/// Helper to get animation frames for a given tile ID
pub fn get_animation_frames(tile_id: u32) -> Option<Vec<u32>> {
    match tile_id {
        110 => Some(ORANGE_FRAMES.to_vec()),        // Orange non-mirrored
        116 => Some(ORANGE_MIRROR_FRAMES.to_vec()), // Orange mirrored
        113 => Some(PURPLE_FRAMES.to_vec()),        // Purple non-mirrored
        119 => Some(PURPLE_MIRROR_FRAMES.to_vec()), // Purple mirrored
        _ => None,
    }
}
