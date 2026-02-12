use bevy::prelude::*;

use crate::components::{Object, Player, TileStepEvent};
use crate::map::{MapDataResource, TILE_SIZE, MAP_H, MAP_W};
use crate::tiles::TileRegistryResource;

pub fn player_movement(
    kb: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
    object_query: Query<&Object>,
    map_data: Res<MapDataResource>,
    tile_registry: Res<TileRegistryResource>,
    mut tile_events: MessageWriter<TileStepEvent>,
) {
    let Ok(mut player) = player_query.single_mut() else {
        return;
    };

    let mut dx = 0;
    let mut dy = 0;

    if kb.just_pressed(KeyCode::KeyW) {
        dy = 1;
    }
    if kb.just_pressed(KeyCode::KeyS) {
        dy = -1;
    }
    if kb.just_pressed(KeyCode::KeyA) {
        dx = -1;
    }
    if kb.just_pressed(KeyCode::KeyD) {
        dx = 1;
    }

    if dx != 0 || dy != 0 {
        let new_x = player.grid_x + dx;
        let new_y = player.grid_y + dy;

        // Check map bounds
        if new_x < 0 || new_x >= MAP_W as i32 || new_y < 0 || new_y >= MAP_H as i32 {
            return;
        }

        // Check tile walkability using tile registry
        if let Some(tile_id) = map_data.0.get_tile(new_x as usize, new_y as usize) {
            if !tile_registry.0.is_walkable(tile_id) {
                return; // Tile not walkable
            }
        }

        // Check collision with objects
        let mut collision = false;
        for object in &object_query {
            let obj_min_x = object.grid_x;
            let obj_max_x = object.grid_x + object.width as i32 - 1;
            let obj_min_y = object.grid_y;
            let obj_max_y = object.grid_y + object.height as i32 - 1;

            if new_x >= obj_min_x && new_x <= obj_max_x && new_y >= obj_min_y && new_y <= obj_max_y
            {
                collision = true;
                break;
            }
        }

        if !collision {
            player.grid_x = new_x;
            player.grid_y = new_y;

            // Fire tile step event if tile has a trigger
            if let Some(tile_id) = map_data.0.get_tile(new_x as usize, new_y as usize) {
                if let Some(tile_def) = tile_registry.0.get(tile_id) {
                    if !matches!(tile_def.trigger, crate::tiles::TileTrigger::None) {
                        tile_events.write(TileStepEvent {
                            tile_id,
                            trigger: tile_def.trigger,
                        });
                    }
                }
            }
        }
    }
}

pub fn update_sprite_positions(
    mut player_query: Query<(&Player, &mut Transform), Without<Object>>,
    mut object_query: Query<(&Object, &mut Transform), Without<Player>>,
) {
    // Update player position - center sprite on tile
    for (player, mut transform) in &mut player_query {
        let world_x = (player.grid_x as f32 - MAP_W as f32 / 2.0 + 0.5) * TILE_SIZE;
        let world_y = (player.grid_y as f32 - MAP_H as f32 / 2.0 + 0.5) * TILE_SIZE;
        transform.translation.x = world_x;
        transform.translation.y = world_y;
    }

    // Update object positions - center sprite on tile
    for (object, mut transform) in &mut object_query {
        let world_x = (object.grid_x as f32 - MAP_W as f32 / 2.0 + 0.5) * TILE_SIZE;
        let world_y = (object.grid_y as f32 - MAP_H as f32 / 2.0 + 0.5) * TILE_SIZE;
        transform.translation.x = world_x;
        transform.translation.y = world_y;
    }
}

pub fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<Camera>>,
    kb: Res<ButtonInput<KeyCode>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok((mut camera_transform, mut projection)) = camera_query.single_mut() else {
        return;
    };

    // Follow player
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;

    // Zoom controls
    let Projection::Orthographic(ortho) = &mut *projection else {
        return;
    };
    if kb.pressed(KeyCode::KeyZ) {
        ortho.scale += 0.1;
    }
    if kb.pressed(KeyCode::KeyX) {
        ortho.scale -= 0.1;
    }
    ortho.scale = ortho.scale.clamp(0.5, 5.0);
}

pub fn spawn_player(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let player_start_x = (MAP_W / 2) as i32;
    let player_start_y = (MAP_H / 2) as i32;

    commands.spawn((
        Sprite::from_image(asset_server.load("player.png")),
        Transform::from_xyz(0.0, 0.0, 10.0),
        Player {
            grid_x: player_start_x,
            grid_y: player_start_y,
        },
    ));
}

pub fn spawn_example_objects(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Example tree
    commands.spawn((
        Sprite {
            image: asset_server.load("grass_biome_8x_16x16.png"),
            color: Color::srgb(0.5, 0.8, 0.5),
            custom_size: Some(Vec2::new(16.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 5.0),
        Object {
            grid_x: 10,
            grid_y: 10,
            width: 1,
            height: 2,
        },
    ));

    // Example building
    commands.spawn((
        Sprite {
            image: asset_server.load("grass_biome_8x_16x16.png"),
            color: Color::srgb(0.8, 0.6, 0.4),
            custom_size: Some(Vec2::new(48.0, 64.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 5.0),
        Object {
            grid_x: 6,
            grid_y: 6,
            width: 3,
            height: 4,
        },
    ));
}
