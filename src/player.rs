use bevy::prelude::*;

use crate::components::Player;
use crate::map::TILE_SIZE;

const PLAYER_START_X: i32 = 12;
const PLAYER_START_Y: i32 = 10;

pub fn player_movement(kb: Res<ButtonInput<KeyCode>>, mut player_query: Query<&mut Player>) {
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
        player.grid_x += dx;
        player.grid_y += dy;
    }
}

pub fn update_sprite_positions(mut player_query: Query<(&Player, &mut Transform)>) {
    for (player, mut transform) in &mut player_query {
        transform.translation.x = player.grid_x as f32 * TILE_SIZE;
        transform.translation.y = player.grid_y as f32 * TILE_SIZE;
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
