use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use std::collections::HashMap;

use crate::tiles::{
    flowers::{
        get_animation_frames, ORANGE_FLOWER_1, ORANGE_FLOWER_MIRROR_1, PURPLE_FLOWER_1,
        PURPLE_FLOWER_MIRROR_1,
    },
    grass::GRASS_TILES,
    water::WATER_DEEP,
    TileRegistryResource,
};
use crate::components::AnimatedTile;

pub const MAP_W: usize = 16;
pub const MAP_H: usize = 16;
pub const TILE_SIZE: f32 = 16.0;

/// Map data - stores stable tile IDs
#[derive(Clone)]
pub struct MapData {
    pub width: usize,
    pub height: usize,
    tiles: Vec<u32>, // Tile IDs
}

impl MapData {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![0; width * height],
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile_id: u32) {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x] = tile_id;
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            Some(self.tiles[y * self.width + x])
        } else {
            None
        }
    }
}

pub fn setup_map(mut commands: Commands, asset_server: AssetServer) {
    // Create map data with grass and water
    let mut map_data = MapData::new(MAP_W, MAP_H);

    // Fill with grass (pseudorandom pattern)
    for y in 0..MAP_H {
        for x in 0..MAP_W {
            let tile_idx = (x * 7 + y * 13) % GRASS_TILES.len();
            map_data.set_tile(x, y, GRASS_TILES[tile_idx].id);
        }
    }

    // Add water in lower right (4x4 section)
    for y in 0..4 {
        for x in 12..MAP_W {
            map_data.set_tile(x, y, WATER_DEEP.id);
        }
    }

    // Lower left: animated flowers (4x4 section)
    // Example: explicit placement with different variants
    for y in 0..4 {
        for x in 0..4 {
            // Checkerboard pattern of orange and purple flowers
            if (x + y) % 2 == 0 {
                map_data.set_tile(x, y, ORANGE_FLOWER_1.id);
            } else {
                map_data.set_tile(x, y, PURPLE_FLOWER_1.id);
            }
        }
    }

    // Example: Mix in mirrored variants
    // map_data.set_tile(0, 0, ORANGE_FLOWER_MIRROR_1.id);
    // map_data.set_tile(1, 0, PURPLE_FLOWER_MIRROR_1.id);

    // Spawn tilemaps from map data
    spawn_tilemaps(&mut commands, asset_server, &map_data);
}

fn spawn_tilemaps(commands: &mut Commands, asset_server: AssetServer, map_data: &MapData) {
    let map_size = TilemapSize {
        x: map_data.width as u32,
        y: map_data.height as u32,
    };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size: TilemapGridSize = tile_size.into();

    // Group tiles by atlas
    let mut atlas_tiles: HashMap<String, Vec<(TilePos, u32, u32)>> = HashMap::new();

    let registry = TileRegistryResource::default().0;

    for y in 0..map_data.height {
        for x in 0..map_data.width {
            if let Some(tile_id) = map_data.get_tile(x, y) {
                if let Some(tile_def) = registry.get(tile_id) {
                    let atlas_key = tile_def.atlas.to_string();
                    atlas_tiles
                        .entry(atlas_key)
                        .or_default()
                        .push((
                            TilePos {
                                x: x as u32,
                                y: y as u32,
                            },
                            tile_def.atlas_index,
                            tile_id,
                        ));
                }
            }
        }
    }

    // Spawn separate tilemap for each atlas
    let mut z_index = 0.0;
    for (atlas_name, tiles) in atlas_tiles.iter() {
        let texture_handle: Handle<Image> = asset_server.load(atlas_name.clone());
        let tilemap_entity = commands.spawn_empty().id();
        let mut tile_storage = TileStorage::empty(map_size);

        for (tile_pos, atlas_index, tile_id) in tiles {
            let mut entity_commands = commands.spawn(TileBundle {
                position: *tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(*atlas_index),
                ..Default::default()
            });

            // Add animation component for flower tiles using helper
            if let Some(frames) = get_animation_frames(*tile_id) {
                entity_commands.insert(AnimatedTile {
                    frames,
                    frame_duration: 0.4,
                    timer: Timer::from_seconds(0.4, TimerMode::Repeating),
                    current_frame: 0,
                });
            }

            let tile_entity = entity_commands.id();
            tile_storage.set(tile_pos, tile_entity);
        }

        commands.entity(tilemap_entity).insert(TilemapBundle {
            grid_size,
            map_type: TilemapType::default(),
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            anchor: TilemapAnchor::Center,
            transform: Transform::from_xyz(0.0, 0.0, z_index),
            ..Default::default()
        });

        z_index += 1.0;
    }

    // Store map data as resource for collision checks
    commands.insert_resource(MapDataResource(map_data.clone()));
}

/// Resource to access map data
#[derive(Resource, Clone)]
pub struct MapDataResource(pub MapData);
