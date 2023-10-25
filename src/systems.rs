use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use rand::prelude::SliceRandom;
use rand::random;

use crate::components::{Asteroid, Mass, Planet, Position, Velocity};
use crate::resources::AsteroidSpawnTimer;

const PLANET_SIZE: f32 = 50.0;
const PLANET_MASS: f32 = 100.0;

const ASTEROID_SIZE: f32 = 10.0;
const ASTEROID_MASS: f32 = 10.0;
const ASTEROID_SPEED: f32 = 100.0;

/// Spawn the planet in the centre of the screen.
///
/// # Arguments
/// * `commands` - a `bevy` `Commands` struct
/// * `meshes` - the resource for getting `Mesh`s
/// * `materials` - the resource for the `ColorMaterial`s
/// * `window_query` - a query to get the primary window of the app
pub fn spawn_planet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    // Set planet coordinates
    let x = window.width() / 2.0;
    let y = window.height() / 2.0;

    // Circle
    commands.spawn(
        (
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(PLANET_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Planet {},
            Mass { mass: PLANET_MASS },
            Position { x, y }
        )
    );
}

/// Spawn the camera in the centre of the screen.
///
/// # Arguments
/// * `commands` - a `bevy` `Commands` struct
/// * `window_query` - a query to get the primary window of the app
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}


/// Spawn an asteroid randomly.
///
/// # Arguments
/// * `commands` - a `bevy` `Commands` struct
/// * `meshes` - the resource for getting `Mesh`s
/// * `materials` - the resource for the `ColorMaterial`s
/// * `window_query` - a query to get the primary window of the app
/// * `timer` - the timer controlling asteroid spawning.
pub fn spawn_asteroid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    timer: Res<AsteroidSpawnTimer>,
) {
    let window = window_query.get_single().unwrap();

    if !timer.timer.finished() {
        return;
    }

    // Set spawn coordinates
    let x = random::<f32>() * window.width();
    let y = random::<f32>() * window.height();

    // Set initial velocities
    let vel_x = vec![-1.0 * ASTEROID_SPEED, ASTEROID_SPEED].choose(&mut rand::thread_rng()).cloned().unwrap();
    let vel_y = vec![-1.0 * ASTEROID_SPEED, ASTEROID_SPEED].choose(&mut rand::thread_rng()).cloned().unwrap();

    commands.spawn(
        (
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(ASTEROID_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Asteroid {},
            Mass { mass: ASTEROID_MASS },
            Position { x, y },
            Velocity { x: vel_x, y: vel_y }
        )
    );
}

/// Tick the timer controlling the spawning of new asteroids.
///
/// # Arguments
/// * `spawn_timer` - the timer controlling asteroid spawning
/// * `time` - the clock tracking the passage of time in game
pub fn tick_asteroid_spawn_timer(mut spawn_timer: ResMut<AsteroidSpawnTimer>, time: Res<Time>) {
    spawn_timer.timer.tick(time.delta());
}

/// Change the positions of the asteroids based on their velocities.
///
/// # Arguments
/// * `asteroids` - query to get `Asteroid`s and their `Position`s and `Velocity`s
/// * `time` - the clock tracking the passage of time in game
pub fn move_asteroids(
    mut asteroids: Query<(&Asteroid, &mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (_, mut position, velocity) in asteroids.iter_mut() {
        let elapsed_time = time.delta_seconds();
        position.translation += Vec3::new(
            velocity.x * elapsed_time,
            velocity.y * elapsed_time,
            0.0,
        );
    }
}