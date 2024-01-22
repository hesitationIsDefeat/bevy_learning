use bevy::app::{App, Plugin, Update};
use bevy::math::Vec2;
use bevy::prelude::{info, Query, Transform, Vec3Swizzles, With};

use crate::components::marker::Player;
use crate::components::normal::{MoveSpeed, Target, Velocity};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, player_movement);
    }
}

pub fn player_movement(mut query: Query<(&MoveSpeed, &mut Velocity, &mut Transform, &mut Target), With<Player>>) {
    let (speed, mut velocity, mut transform, mut target) = query.single_mut();
    if velocity.value != Vec2::ZERO {
        let distance = transform.translation.distance(target.value.extend(0.));
        info!("Distance is {:?}", distance);
        if distance < speed.value / 50. {
            velocity.value = Vec2::ZERO;
            target.value = transform.translation.xy();
        } else {
            transform.translation += velocity.value.extend(0.);
        }
    }
}