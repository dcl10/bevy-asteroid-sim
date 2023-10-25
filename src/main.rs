use bevy::prelude::*;

use crate::resources::AsteroidSpawnTimer;
use crate::systems::{
    collide_asteroids, collide_asteroids_with_planet, move_asteroids, spawn_asteroid, spawn_camera,
    spawn_planet, tick_asteroid_spawn_timer,
};

mod components;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<AsteroidSpawnTimer>()
        .add_systems(Startup, (spawn_camera, spawn_planet).chain())
        .add_systems(Update, (tick_asteroid_spawn_timer, spawn_asteroid))
        .add_systems(Update, move_asteroids)
        .add_systems(Update, collide_asteroids_with_planet)
        .add_systems(Update, collide_asteroids)
        .run();
}
