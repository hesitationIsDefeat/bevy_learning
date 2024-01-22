use bevy::app::App;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Entity, Input, MouseButton, Plugin, Query, Res, Time, Transform, Update, Window, With};
use bevy::window::PrimaryWindow;

use crate::components::marker::Player;
use crate::components::normal::{MoveSpeed, Target, Velocity};
use crate::resources::WindowHeight;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_input);
    }
}

pub fn mouse_input(mut commands: Commands, window_height: Res<WindowHeight>, time: Res<Time>, buttons: Res<Input<MouseButton>>, q_windows: Query<&Window, With<PrimaryWindow>>, mut q_player: Query<(Entity, &mut Transform, &MoveSpeed), With<Player>>) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(mouse_pos) = q_windows.single().cursor_position() {
            let (entity, mut transform, move_speed) = q_player.single_mut();
            let movement_vector = Vec2::new(mouse_pos.x - transform.translation.x, (window_height.0 - mouse_pos.y) - transform.translation.y).normalize();
            commands.entity(entity)
                .insert(Velocity::from_v2(movement_vector))
                .insert(Target::new(mouse_pos.x, window_height.0 - mouse_pos.y));
        }
    }
}