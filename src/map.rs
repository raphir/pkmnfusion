use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub const TILE_SIZE: f32 = 16.0;

pub fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TiledMap(asset_server.load("maps/prototype.tmx")),
        TilemapAnchor::Center,
    ));
}
