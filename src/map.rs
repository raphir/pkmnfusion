use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub const TILE_SIZE: f32 = 16.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TileMovement {
    #[default]
    Free,
    Blocked,
    Ledge {
        dx: i32,
        dy: i32,
    },
}

#[derive(Resource)]
pub struct WalkabilityMap {
    pub width: usize,
    pub height: usize,
    tiles: Vec<TileMovement>,
}

impl WalkabilityMap {
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    pub fn can_enter(&self, x: i32, y: i32, dx: i32, dy: i32) -> bool {
        if !self.in_bounds(x, y) {
            return false;
        }
        match self.tiles[y as usize * self.width + x as usize] {
            TileMovement::Free => true,
            TileMovement::Blocked => false,
            TileMovement::Ledge { dx: lx, dy: ly } => dx == lx && dy == ly,
        }
    }
}

fn parse_jump_direction(value: &str) -> Option<(i32, i32)> {
    match value {
        "down" => Some((0, -1)),
        "up" => Some((0, 1)),
        "left" => Some((-1, 0)),
        "right" => Some((1, 0)),
        _ => None,
    }
}

pub fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TiledMap(asset_server.load("maps/prototype.tmx")),
        TilemapAnchor::Center,
    ));
}

pub fn build_walkability(
    map_query: Query<&TiledMap>,
    assets: Res<Assets<TiledMapAsset>>,
    mut commands: Commands,
) {
    let Ok(tiled_map) = map_query.single() else {
        return;
    };

    let Some(asset) = assets.get(&tiled_map.0) else {
        return;
    };

    let map = &asset.map;
    let width = map.width as usize;
    let height = map.height as usize;

    let mut tiles = vec![TileMovement::Free; width * height];

    // Layers iterate bottom-to-top; higher layers override lower
    for layer in map.layers() {
        let Some(tile_layer) = layer.as_tile_layer() else {
            continue;
        };

        for tiled_y in 0..height {
            for x in 0..width {
                let Some(layer_tile) = tile_layer.get_tile(x as i32, tiled_y as i32) else {
                    continue;
                };

                let Some(tile) = layer_tile.get_tile() else {
                    continue;
                };

                // Tiled y=0 is top, bevy y=0 is bottom
                let bevy_y = (height - 1) - tiled_y;
                let idx = bevy_y * width + x;

                // jumpDirection takes priority over walkable
                if let Some(::tiled::PropertyValue::StringValue(dir)) =
                    tile.properties.get("jumpDirection")
                {
                    if let Some((dx, dy)) = parse_jump_direction(dir) {
                        tiles[idx] = TileMovement::Ledge { dx, dy };
                    }
                } else if let Some(::tiled::PropertyValue::BoolValue(walkable)) =
                    tile.properties.get("walkable")
                {
                    tiles[idx] = if *walkable {
                        TileMovement::Free
                    } else {
                        TileMovement::Blocked
                    };
                }
            }
        }
    }

    let blocked = tiles
        .iter()
        .filter(|t| **t == TileMovement::Blocked)
        .count();
    let ledges = tiles
        .iter()
        .filter(|t| matches!(t, TileMovement::Ledge { .. }))
        .count();
    info!("Built movement map: {width}x{height}, {blocked} blocked, {ledges} ledges");

    commands.insert_resource(WalkabilityMap {
        width,
        height,
        tiles,
    });
}
