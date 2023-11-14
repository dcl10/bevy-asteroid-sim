use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};

use crate::resources::AsteroidSpawnTimer;
use crate::systems::{
    collide_asteroids, collide_asteroids_with_planet, despawn_off_screen_asteroid, gravity,
    move_asteroids, rotate_body, setup, spawn_asteroid, spawn_moon, spawn_planet,
    tick_asteroid_spawn_timer, update_orbits,
};

mod components;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy-asteroid-sim".into(),
                resolution: (1175., 800.).into(),
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<AsteroidSpawnTimer>()
        .add_systems(Startup, (setup, spawn_planet))
        .add_systems(
            PreUpdate,
            (tick_asteroid_spawn_timer, spawn_asteroid, spawn_moon),
        )
        .add_systems(
            Update,
            (
                move_asteroids,
                collide_asteroids_with_planet,
                collide_asteroids,
                gravity,
                rotate_body,
                update_orbits,
            ),
        )
        .add_systems(PostUpdate, despawn_off_screen_asteroid)
        .run();
}
