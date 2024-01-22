use bevy::app::{App, Plugin};
use bevy::log::info;
use bevy::prelude::{Entity, Query, Transform, Update, With};

use crate::components::marker::Player;
use crate::components::normal::Velocity;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, print_player_speed);
    }
}

pub fn print_positions(query: Query<(Entity, &Transform)>) {
    for (ent, trans) in query.iter() {
        info!("Entity {:?} is at position ({:?})", ent, trans)
    }
}

pub fn print_player_speed(query: Query<&Velocity, With<Player>>) {
    let velocity = query.single();
    let speed = velocity.direction.length() * velocity.speed;
    info!("Player speed: {speed}")
}