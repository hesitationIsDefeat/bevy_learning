use bevy::app::{App, Plugin};
use bevy::log::info;
use bevy::prelude::{Entity, Query, Transform, With};

use crate::components::marker::Player;
use crate::components::normal::Velocity;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, _: &mut App) {}
}

pub fn print_positions(query: Query<(Entity, &Transform)>) {
    for (ent, trans) in query.iter() {
        info!("Entity {:?} is at position ({:?})", ent, trans)
    }
}

/// Deprecated
#[deprecated(note = "This function doesn't take the isometric speed into account, thus should be updated")]
pub fn print_player_speed(query: Query<&Velocity, With<Player>>) {
    let velocity = query.single();
    let speed = velocity.direction.length() * velocity.speed;
    info!("Player speed: {speed}")
}