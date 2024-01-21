use bevy::prelude::Bundle;
use crate::components::marker::Player;
use crate::components::normal::{Position, Velocity};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub position: Position,
    pub velocity: Velocity,
}