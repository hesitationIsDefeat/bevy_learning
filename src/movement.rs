use bevy::app::{App, Plugin, Update};
use bevy::math::Vec2;
use bevy::prelude::{info, Query, Res, Time, Transform, Vec3Swizzles, With};

use crate::components::marker::Player;
use crate::components::normal::{Target, Velocity};

/// A lower bound to the fps, used for
const FPS_LOWER_BOUND: f32 = 50.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, player_movement);
    }
}

/// Checks if the player is supposed to move. If so, checks if the destination is reached. If so, resets the velocity and
/// sets the target to it's current position. If not, moves the player in the given direction.
pub fn player_movement(time: Res<Time>, mut query: Query<(&mut Velocity, &mut Transform, &mut Target), With<Player>>) {
    let (mut velocity, mut transform, mut target) = query.single_mut();
    if velocity.direction != Vec2::ZERO {
        let distance = transform.translation.distance(target.value.extend(0.));
        info!("Distance is {:?}", distance);
        if distance < velocity.speed / FPS_LOWER_BOUND {
            velocity.direction = Vec2::ZERO;
            target.value = transform.translation.xy();
        } else {
            transform.translation += velocity.direction.extend(0.) * velocity.speed * speed_isometric_multiplier(velocity.direction) * time.delta_seconds();
        }
    }
}

/// Since the movement in the y direction must be slower due to the isometric camera angle, it adjusts the speed
/// so that the movement does fit the isometric perspective
fn speed_isometric_multiplier(direction: Vec2) -> f32 {
    let angle_radians = direction.angle_between(Vec2::X);
    ((angle_radians * 2.).cos() + 3.) / 4.
}