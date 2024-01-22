use bevy::app::App;
use bevy::math::Vec2;
use bevy::prelude::{Input, MouseButton, Plugin, Query, Res, Transform, Update, Window, With};
use bevy::window::PrimaryWindow;

use crate::components::marker::Player;
use crate::components::normal::{Target, Velocity};
use crate::resources::WindowHeight;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_input);
    }
}

pub fn mouse_input(window_height: Res<WindowHeight>, buttons: Res<Input<MouseButton>>, q_windows: Query<&Window, With<PrimaryWindow>>, mut q_player: Query<(&mut Transform, &mut Velocity, &mut Target), With<Player>>) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(mouse_pos) = q_windows.single().cursor_position() {
            let (mut transform, mut velocity, mut target) = q_player.single_mut();
            let movement_vector = Vec2::new(mouse_pos.x - transform.translation.x, (window_height.0 - mouse_pos.y) - transform.translation.y);
            velocity.direction = movement_vector.normalize_or_zero();
            target.value = Vec2::new(mouse_pos.x, window_height.0 - mouse_pos.y);
        }
    }
}