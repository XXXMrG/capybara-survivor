use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(400.0, 300.0)),
                ..default()
            },
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        },
        Player { speed: 200.0 },
    ));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &Player), With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    for (mut player_transform, player) in &mut player_query {
        let speed = player.speed;
        let movement = Vec3::new(
            actions.player_movement.unwrap().x * speed * time.delta_seconds(),
            actions.player_movement.unwrap().y * speed * time.delta_seconds(),
            0.,
        );

        player_transform.translation += movement;
    }
}
