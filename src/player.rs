use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{AssetServer, Bundle, Commands, default, Res, SpriteBundle, Transform};

use crate::components::marker::Player;
use crate::components::normal::{MoveSpeed, Velocity};

const PLAYER_SPEED: f32 = 5.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player);
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        marker: Player,
        model: SpriteBundle {
            transform: Transform::from_xyz(3., 4., 0.),
            texture: asset_server.load("sprites/tiles/grass_dirt.png"),
            ..default()
        },
        velocity: None,
        move_speed: MoveSpeed::new(PLAYER_SPEED),
    });
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub model: SpriteBundle,
    pub velocity: Option<Velocity>,
    pub move_speed: MoveSpeed,
}
