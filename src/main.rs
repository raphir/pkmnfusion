mod components;
mod map;
mod player;
mod tiles;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use components::{AnimatedTile, TileStepEvent};
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
        .add_message::<TileStepEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player::player_movement,
                player::camera_follow,
                player::update_sprite_positions,
                animate_tiles,
            ),
        )
        .run();
}

fn animate_tiles(time: Res<Time>, mut query: Query<(&mut AnimatedTile, &mut TileTextureIndex)>) {
    for (mut anim, mut texture_index) in query.iter_mut() {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished() {
            anim.current_frame = (anim.current_frame + 1) % anim.frames.len();
            texture_index.0 = anim.frames[anim.current_frame];
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Setup map with tile ID system
    map::setup_map(commands.reborrow(), asset_server.clone());

    // Spawn player and objects
    player::spawn_player(&mut commands, &asset_server);
    player::spawn_example_objects(&mut commands, &asset_server);
}
