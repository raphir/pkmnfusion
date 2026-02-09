mod components;
mod map;
mod player;
mod tiles;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use tiles::TileRegistryResource;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("PkmnFusion"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .insert_resource(TileRegistryResource::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player::player_movement,
                player::camera_follow,
                player::update_sprite_positions,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Setup map with tile ID system
    map::setup_map(commands.reborrow(), asset_server.clone());

    // Spawn player and objects
    player::spawn_player(&mut commands, &asset_server);
    player::spawn_example_objects(&mut commands, &asset_server);
}
