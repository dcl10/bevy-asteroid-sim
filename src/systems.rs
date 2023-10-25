use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use itertools::Itertools;
use rand::{random, Rng};

use crate::components::{Asteroid, Mass, Planet, Velocity};
use crate::resources::AsteroidSpawnTimer;

const PLANET_RADIUS: f32 = 50.0;
const PLANET_MASS: f32 = 100.0;

const ASTEROID_RADIUS: f32 = 10.0;
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
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(PLANET_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
        Planet {},
        Mass { mass: PLANET_MASS },
    ));
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
    let mut rng = rand::thread_rng();
    let vel_x = rng.gen_range(-1.0 * ASTEROID_SPEED..ASTEROID_SPEED);
    let vel_y = rng.gen_range(-1.0 * ASTEROID_SPEED..ASTEROID_SPEED);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(ASTEROID_RADIUS).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
        Asteroid {},
        Mass {
            mass: ASTEROID_MASS,
        },
        Velocity { x: vel_x, y: vel_y },
    ));
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
        position.translation +=
            Vec3::new(velocity.x * elapsed_time, velocity.y * elapsed_time, 0.0);
    }
}

/// Collide asteroids with the planet and despawn them.
///
/// # Arguments
/// * `commands` - a `bevy` `Commands` struct
/// * `asteroids_query` - query to get asteroid entities and their coordinates
/// * `planet_query` - query to get the coordinates of the planet
pub fn collide_asteroids_with_planet(
    mut commands: Commands,
    asteroids_query: Query<(Entity, &Transform), With<Asteroid>>,
    planet_query: Query<&Transform, With<Planet>>,
) {
    let planet_transform = planet_query.get_single().unwrap();

    // Collide with the planet
    for (entity, asteroid_transform) in asteroids_query.iter() {
        let abs_dist_x =
            (planet_transform.translation.x - asteroid_transform.translation.x).powf(2.0);
        let abs_dist_y =
            (planet_transform.translation.y - asteroid_transform.translation.y).powf(2.0);
        let abs_dist = (abs_dist_x + abs_dist_y).sqrt();
        if abs_dist <= ASTEROID_RADIUS + PLANET_RADIUS {
            commands.entity(entity).despawn();
        }
    }
}

/// Collide asteroids with each other and despawn them.
///
/// # Arguments
/// * `commands` - a `bevy` `Commands` struct
/// * `asteroids_query` - query to get asteroid entities and their coordinates
pub fn collide_asteroids(
    mut commands: Commands,
    asteroids_query: Query<(Entity, &Transform), With<Asteroid>>,
) {
    let transforms = asteroids_query.iter().combinations(2);

    for combination in transforms.into_iter() {
        // unpack entities
        let c1 = combination.first();
        let c2 = combination.last();

        match (c1, c2) {
            (Some((e1, t1)), Some((e2, t2))) => {
                // calculate the absolute distance between them
                let abs_dist_x = (t1.translation.x - t2.translation.x).powf(2.0);
                let abs_dist_y = (t1.translation.y - t2.translation.y).powf(2.0);
                let abs_dist = (abs_dist_x + abs_dist_y).sqrt();

                // delete the entities
                if abs_dist <= ASTEROID_RADIUS * 2.0 {
                    match commands.get_entity(*e1) {
                        None => {}
                        Some(mut e) => e.despawn(),
                    }

                    match commands.get_entity(*e2) {
                        None => {}
                        Some(mut e) => e.despawn(),
                    }
                }
            }
            _ => {} // ignore if there aren't 2 entities to collide
        }
    }
}
