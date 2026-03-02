use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub const TILE_SIZE: f32 = 16.0;

#[derive(Resource)]
pub struct WalkabilityMap {
    pub width: usize,
    pub height: usize,
    walkable: Vec<bool>,
}

impl WalkabilityMap {
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if !self.in_bounds(x, y) {
            return false;
        }
        self.walkable[y as usize * self.width + x as usize]
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

    // Start all walkable
    let mut walkable = vec![true; width * height];

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

                if let Some(::tiled::PropertyValue::BoolValue(false)) =
                    tile.properties.get("walkable")
                {
                    // Tiled y=0 is top, bevy y=0 is bottom
                    let bevy_y = (height - 1) - tiled_y;
                    walkable[bevy_y * width + x] = false;
                }
            }
        }
    }

    info!(
        "Built walkability map: {}x{}, {} blocked tiles",
        width,
        height,
        walkable.iter().filter(|w| !**w).count()
    );

    commands.insert_resource(WalkabilityMap {
        width,
        height,
        walkable,
    });
}
