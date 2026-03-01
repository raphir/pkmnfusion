pub mod grass;
pub mod water;
pub mod flowers;

use bevy::prelude::*;

/// Trigger that fires when player steps on a tile
#[derive(Clone, Copy, Debug)]
pub enum TileTrigger {
    None,
    GrassEncounter { rate: f32 },
    Sign { text: &'static str },
    Warp { map: &'static str, x: i32, y: i32 },
}

/// Stable tile definition - ID never changes even if atlas reorganized
#[derive(Clone, Copy, Debug)]
pub struct TileDefinition {
    pub id: u32,
    pub name: &'static str,
    pub atlas: &'static str,
    pub atlas_index: u32,
    pub walkable: bool,
    pub trigger: TileTrigger,
}

impl TileDefinition {
    pub const fn new(
        id: u32,
        name: &'static str,
        atlas: &'static str,
        atlas_index: u32,
        walkable: bool,
        trigger: TileTrigger,
    ) -> Self {
        Self {
            id,
            name,
            atlas,
            atlas_index,
            walkable,
            trigger,
        }
    }
}

/// Registry of all tiles in the game
pub struct TileRegistry {
    tiles: Vec<TileDefinition>,
}

impl TileRegistry {
    pub fn new() -> Self {
        let mut tiles = Vec::new();

        // Register all grass tiles
        tiles.extend_from_slice(&grass::GRASS_TILES);

        // Register all water tiles
        tiles.extend_from_slice(&water::WATER_TILES);

        // Register all flower tiles
        tiles.extend_from_slice(&flowers::FLOWER_TILES);

        Self { tiles }
    }

    pub fn get(&self, id: u32) -> Option<&TileDefinition> {
        self.tiles.iter().find(|t| t.id == id)
    }

    pub fn is_walkable(&self, id: u32) -> bool {
        self.get(id).map(|t| t.walkable).unwrap_or(false)
    }
}

impl Default for TileRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Bevy resource to access tile registry
#[derive(Resource)]
pub struct TileRegistryResource(pub TileRegistry);

impl Default for TileRegistryResource {
    fn default() -> Self {
        Self(TileRegistry::new())
    }
}
