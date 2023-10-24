use bevy::{prelude::*};

use crate::resources::AsteroidSpawnTimer;
use crate::systems::{spawn_asteroid, spawn_camera, spawn_planet, tick_asteroid_spawn_timer};

mod components;
mod systems;
mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<AsteroidSpawnTimer>()
        .add_systems(Startup, (spawn_camera, spawn_planet).chain())
        .add_systems(Update, (tick_asteroid_spawn_timer, spawn_asteroid))
        .run();
}