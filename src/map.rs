use crate::loading::MapAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_map);
    }
}

fn spawn_map(mut commands: Commands, map_assets: Res<MapAssets>) {
    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(0., 150., 0.),
            ..Default::default()
        },
        texture_atlas: map_assets.tile_maps.clone(),
        sprite: TextureAtlasSprite::new(1),
        ..Default::default()
    });
}
