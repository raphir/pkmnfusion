use bevy::prelude::*;
use crate::tiles::TileTrigger;

#[derive(Component)]
pub struct Player {
    pub grid_x: i32,
    pub grid_y: i32,
}

#[derive(Message, Clone)]
pub struct TileStepEvent {
    pub tile_id: u32,
    pub trigger: TileTrigger,
}

#[derive(Component)]
pub struct Object {
    pub grid_x: i32,
    pub grid_y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Component)]
pub struct AnimatedTile {
    pub frames: Vec<u32>,      // Atlas indices for animation frames
    pub frame_duration: f32,    // Duration per frame in seconds
    pub timer: Timer,
    pub current_frame: usize,
}
