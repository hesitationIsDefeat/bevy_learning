use bevy::app::App;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Entity, info, Input, MouseButton, Plugin, Query, Res, Time, Transform, Update, Window, With};
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
            info!("Cursor is inside the primary window, at {:?}", mouse_pos);
            let (entity, mut transform, mut move_speed) = q_player.single_mut();
            let x_len = mouse_pos.x - transform.translation.x;
            let y_len = (window_height.0 - mouse_pos.y) - transform.translation.y;
            let adjustment_constant = move_speed.value / (x_len.powi(2) + y_len.powi(2)).sqrt();
            commands.entity(entity)
                .insert(Velocity::from_v2(Vec2::new(x_len, y_len) * adjustment_constant * time.delta_seconds()))
                .insert(Target::new(mouse_pos.x, window_height.0 - mouse_pos.y));
        }
    }
}