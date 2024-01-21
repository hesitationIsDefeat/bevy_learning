use bevy::app::{App, Plugin};
use bevy::log::info;
use bevy::prelude::{Entity, Query, Transform};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn print_positions(query: Query<(Entity, &Transform)>) {
    for (ent, trans) in query.iter() {
        info!("Entity {:?} is at position ({:?})", ent, trans)
    }
}