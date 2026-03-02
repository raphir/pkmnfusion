mod components;
mod map;
mod player;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

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
        .add_plugins(TiledPlugin::default())
        .add_systems(
            Startup,
            (map::setup_map, player::spawn_player, setup_camera),
        )
        .add_systems(
            Update,
            (
                map::build_walkability.run_if(not(resource_exists::<map::WalkabilityMap>)),
                player::player_movement,
                player::update_sprite_positions,
                player::camera_follow,
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
