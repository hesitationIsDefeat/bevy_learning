use bevy::DefaultPlugins;
use bevy::prelude::{App, Component, Plugin, Resource, Startup};

use crate::camera::spawn_camera;
use crate::debug::DebugPlugin;
use crate::input::InputPlugin;
use crate::movement::MovementPlugin;
use crate::player::PlayerPlugin;

mod resources;
mod map;
mod components;
mod player;
mod movement;
mod debug;
mod camera;
mod input;

fn main() {
    App::new()
        .add_systems(Startup, spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_plugins(InputPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}



