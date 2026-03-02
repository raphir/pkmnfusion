use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::tiles::{AnimatedTile, TileTextureIndex};

// (start_inclusive, end_exclusive, speed)
const GROUND_ANIMS: &[(u32, u32, f32)] = &[
    (7, 11, 2.0),  // water_sea
    (23, 26, 2.0), // water_pond
    (39, 42, 1.5), // water_reef
];

const FLORA_ANIMS: &[(u32, u32, f32)] = &[
    (7, 13, 1.5),  // flowers_a
    (23, 29, 1.5), // flowers_b
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
            "ground" => GROUND_ANIMS,
            "flora_masked" => FLORA_ANIMS,
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
