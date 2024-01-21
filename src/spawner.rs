use bevy::prelude::Commands;
use crate::bundles::PlayerBundle;
use crate::components::marker::Player;
use crate::components::normal::{Position, Velocity};

pub fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        marker: Player,
        position: Position::new(0.0, 0.0),
        velocity: Velocity::new(0.1, 0.1),
    });
}