use bevy::prelude::{Entity, info, Query};
use crate::components::normal::{Position, Velocity};

pub fn update_positions(mut query: Query<(&Velocity, &mut Position)>) {
    for (vel, mut pos) in query.iter_mut() {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}

pub fn print_positions(query: Query<(Entity, &Position)>) {
    for (ent, pos) in query.iter() {
        info!("Entity {:?} is at position ({:?})", ent, pos)
    }
}