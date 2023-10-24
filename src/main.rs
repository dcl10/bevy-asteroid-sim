use bevy::{prelude::*};

use crate::resources::AsteroidSpawnTimer;
use crate::systems::{spawn_camera, spawn_planet};

mod components;
mod systems;
mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<AsteroidSpawnTimer>()
        .add_systems(Startup, (spawn_camera, spawn_planet))
        .run();
}