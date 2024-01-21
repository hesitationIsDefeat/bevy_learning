use bevy::prelude::{Camera2dBundle, Commands, default, Query, Transform, Window, With};
use bevy::window::PrimaryWindow;

use crate::resources::WindowHeight;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
    commands.insert_resource(WindowHeight(window.height()));
}