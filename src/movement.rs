use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Query, Transform};

use crate::components::normal::Velocity;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, player_movement);
    }
}

pub fn player_movement(mut query: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut pos) in query.iter_mut() {
        pos.translation.x += vel.value.x;
        pos.translation.y += vel.value.y;
    }
}