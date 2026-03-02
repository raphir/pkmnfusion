use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub grid_x: i32,
    pub grid_y: i32,
}
