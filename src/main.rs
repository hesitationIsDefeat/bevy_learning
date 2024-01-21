mod constants;
mod resources;
mod systems;
mod map;
mod bundles;
mod components;
mod spawner;
mod plugins;

use bevy::DefaultPlugins;
use bevy::prelude::{App, Commands, Component, Plugin, Query, Res, ResMut, Resource, Startup, Time, Timer, Update, With};
use bevy::time::TimerMode;
use crate::plugins::PositionPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PositionPlugin))
        .run();
}



