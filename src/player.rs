use bevy::prelude::*;

use crate::components::Player;
use crate::map::{WalkabilityMap, TILE_SIZE};

const PLAYER_START_X: i32 = 18;
const PLAYER_START_Y: i32 = 0;

pub fn player_movement(
    kb: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
    walkability: Option<Res<WalkabilityMap>>,
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

    if dx == 0 && dy == 0 {
        return;
    }

    let new_x = player.grid_x + dx;
    let new_y = player.grid_y + dy;

    if let Some(ref map) = walkability {
        if !map.is_walkable(new_x, new_y) {
            return;
        }
    }

    player.grid_x = new_x;
    player.grid_y = new_y;
}

pub fn update_sprite_positions(
    mut player_query: Query<(&Player, &mut Transform)>,
    walkability: Option<Res<WalkabilityMap>>,
) {
    for (player, mut transform) in &mut player_query {
        if let Some(ref map) = walkability {
            let w = map.width as f32;
            let h = map.height as f32;
            transform.translation.x = (player.grid_x as f32 - w / 2.0 + 0.5) * TILE_SIZE;
            transform.translation.y = (player.grid_y as f32 - h / 2.0 + 0.5) * TILE_SIZE;
        } else {
            transform.translation.x = player.grid_x as f32 * TILE_SIZE;
            transform.translation.y = player.grid_y as f32 * TILE_SIZE;
        }
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

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;

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

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
        Transform::from_xyz(
            PLAYER_START_X as f32 * TILE_SIZE,
            PLAYER_START_Y as f32 * TILE_SIZE,
            10.0,
        ),
        Player {
            grid_x: PLAYER_START_X,
            grid_y: PLAYER_START_Y,
        },
    ));
}
