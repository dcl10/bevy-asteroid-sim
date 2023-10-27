use bevy::prelude::*;

use crate::resources::AsteroidSpawnTimer;
use crate::systems::{
    collide_asteroids, collide_asteroids_with_planet, despawn_off_screen_asteroid, gravity,
    move_asteroids, rotate_body, setup, spawn_asteroid, spawn_planet, tick_asteroid_spawn_timer,
};

mod components;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<AsteroidSpawnTimer>()
        .add_systems(Startup, (setup, spawn_planet).chain())
        .add_systems(PreUpdate, spawn_asteroid)
        .add_systems(Update, tick_asteroid_spawn_timer)
        .add_systems(Update, move_asteroids)
        .add_systems(Update, collide_asteroids_with_planet)
        .add_systems(Update, collide_asteroids)
        .add_systems(Update, despawn_off_screen_asteroid)
        .add_systems(Update, gravity)
        .add_systems(Update, rotate_body)
        .run();
}
