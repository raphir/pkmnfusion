use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::tiles::{AnimatedTile, TileTextureIndex};

// (start_inclusive, end_exclusive, speed)
const WATER_ANIMS: &[(u32, u32, f32)] = &[
    (0, 4, 0.5),   // water_sea
    (8, 11, 0.5),  // water_pond
    (16, 19, 0.5), // water_reef
];

const FLORA_ANIMS: &[(u32, u32, f32)] = &[
    (32, 36, 0.5), // flowers_a
    (48, 52, 0.5), // flowers_b
    (64, 68, 0.5), // flowers_c
    (80, 84, 0.5), // flowers_d
];

#[derive(Resource)]
pub struct TileAnimationsApplied;

pub fn apply_tile_animations(
    tiles: Query<(Entity, &TileTextureIndex, &TilemapId), With<TiledTile>>,
    tilemaps: Query<&TiledName, With<TiledTilemap>>,
    mut commands: Commands,
) {
    if tiles.is_empty() {
        return;
    }

    let mut count = 0u32;
    for (entity, tex_idx, tilemap_id) in &tiles {
        let Ok(name) = tilemaps.get(tilemap_id.0) else {
            continue;
        };

        let anims = match name.0.as_str() {
            "water_animated" => WATER_ANIMS,
            "flora_animated" => FLORA_ANIMS,
            _ => continue,
        };

        for &(start, end, speed) in anims {
            if tex_idx.0 >= start && tex_idx.0 < end {
                commands
                    .entity(entity)
                    .insert(AnimatedTile { start, end, speed });
                count += 1;
                break;
            }
        }
    }

    info!("Applied tile animations to {count} tiles");
    commands.insert_resource(TileAnimationsApplied);
}
