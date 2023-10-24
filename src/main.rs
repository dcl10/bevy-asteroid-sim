use bevy::{prelude::*};

use crate::systems::{spawn_camera, spawn_planet};

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_planet))
        .run();
}