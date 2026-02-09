use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub grid_x: i32,
    pub grid_y: i32,
}

#[derive(Component)]
pub struct Object {
    pub grid_x: i32,
    pub grid_y: i32,
    pub width: u32,
    pub height: u32,
}
