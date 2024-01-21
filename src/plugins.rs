use bevy::app::{App, Plugin, Startup, Update};
use crate::spawner::spawn_player;
use crate::systems::{print_positions, update_positions};

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (update_positions, print_positions));
    }
}