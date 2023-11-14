use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use rand::seq::SliceRandom;
use rand::{random, Rng};

use crate::components::{AngularVelocity, Asteroid, Mass, Moon, Orbit, Planet, Velocity};
use crate::resources::AsteroidSpawnTimer;
use crate::traits::Between;

const SCALE_FACTOR: f32 = 10e9;
const PLANET_RADIUS: f32 = 50.0;
const PLANET_MASS: f32 = 1.9e27 / SCALE_FACTOR;
const PLANET_ANGULAR_SPEED: f32 = 0.5;
const PLANET_PNG: &str = "images/planet05.png";

const ASTEROID_RADIUS: f32 = 10.0;
const ASTEROID_MASS: f32 = 9.3e20 / SCALE_FACTOR;
const ASTEROID_SPEED: f32 = 100.0;
const ASTEROID_ANGULAR_SPEED: f32 = 20.0;
const ASTEROID_PNGS: [&str; 4] = [
    "images/meteorBrown_big1.png",
    "images/meteorBrown_big2.png",
    "images/meteorGrey_big1.png",
    "images/meteorGrey_big2.png",
];

const G: f32 = 6.67e-11;

/// Spawn the planet in the centre of the screen.
///
/// # Arguments
/// * `commands` - a `bevy` `Commands` struct
/// * `meshes` - the resource for getting `Mesh`s
/// * `materials` - the resource for the `ColorMaterial`s
/// * `window_query` - a query to get the primary window of the app
pub fn spawn_planet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    // Set planet coordinates
    let x = window.width() / 2.0;
    let y = window.height() / 2.0;

    // Circle
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(PLANET_PNG),
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
        Planet {},
        Mass { mass: PLANET_MASS },
        AngularVelocity {
            velocity: PLANET_ANGULAR_SPEED,
        },
    ));
}

/// Spawn the camera in the centre of the screen.
///
/// # Arguments
/// * `commands` - a `bevy` `Commands` struct
/// * `window_query` - a query to get the primary window of the app
pub fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let background = asset_server.load("images/background.png");
    commands.spawn(SpriteBundle {
        texture: background,
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -10.0),
        ..default()
    });
    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        })
        .commands();
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
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    timer: Res<AsteroidSpawnTimer>,
) {
    let window = window_query.get_single().unwrap();

    if !timer.timer.finished() {
        return;
    }

    // Set spawn coordinates
    let mut x = random::<f32>() * window.width();
    let mut y = random::<f32>() * window.height();

    // Randomly assign the off-screen asteroid spawn location
    if random::<bool>() {
        // spawn from either left or right of the screen
        x = if random::<bool>() {
            0.0
        } else {
            window.width()
        };
    } else {
        // spawn from either top or bottom of the screen
        y = if random::<bool>() {
            0.0
        } else {
            window.height()
        };
    }

    // Set initial velocities
    let mut rng = rand::thread_rng();
    let vel_x = rng.gen_range(-1.0 * ASTEROID_SPEED..ASTEROID_SPEED);
    let vel_y = rng.gen_range(-1.0 * ASTEROID_SPEED..ASTEROID_SPEED);
    let omega = (random::<f32>() * ASTEROID_ANGULAR_SPEED) * [-1.0, 1.0].choose(&mut rng).unwrap();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(ASTEROID_PNGS.choose(&mut rng).unwrap().to_string()),
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
        Asteroid {},
        Mass {
            mass: ASTEROID_MASS,
        },
        Velocity { x: vel_x, y: vel_y },
        AngularVelocity { velocity: omega },
        Orbit::default(),
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
/// * `satellites` - query to get `Asteroid`s and `Moon`s, along with their positions and `Velocity`s
/// * `time` - the clock tracking the passage of time in game
pub fn update_velocities(mut satellites: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in satellites.iter_mut() {
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
    for [c1, c2] in asteroids_query.iter_combinations() {
        // unpack entities
        let (e1, t1) = c1;
        let (e2, t2) = c2;

        // calculate the absolute distance between them
        let abs_dist_x = (t1.translation.x - t2.translation.x).powf(2.0);
        let abs_dist_y = (t1.translation.y - t2.translation.y).powf(2.0);
        let abs_dist = (abs_dist_x + abs_dist_y).sqrt();

        // delete the entities
        if abs_dist <= ASTEROID_RADIUS * 2.0 {
            match commands.get_entity(e1) {
                None => {}
                Some(mut e) => e.despawn(),
            }

            match commands.get_entity(e2) {
                None => {}
                Some(mut e) => e.despawn(),
            }
        }
    }
}

/// Despawn asteroids that have gone off-screen.
///
/// # Arguments
/// * `commands` - a `bevy` `Commands` struct
/// * `asteroids_query` - query to get asteroid entities and their coordinates
/// * `window_query` - a query to get the primary window of the app
pub fn despawn_off_screen_asteroid(
    mut commands: Commands,
    asteroids_query: Query<(Entity, &Transform), With<Asteroid>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (entity, transform) in asteroids_query.iter() {
        if transform.translation.x < 0.0 - ASTEROID_RADIUS
            || transform.translation.x > window.width() + ASTEROID_RADIUS
            || transform.translation.y < 0.0 - ASTEROID_RADIUS
            || transform.translation.y > window.height() + ASTEROID_RADIUS
        {
            commands.entity(entity).despawn()
        }
    }
}

/// Alter the velocities of asteroids due to gravity from the planet.
///
/// # Arguments
/// * `asteroids_query` - query to get asteroid coordinates, masses and velocities
/// * `planet_query` - query to get the coordinates and mass of the planet
pub fn gravity(
    mut asteroids_query: Query<(&Transform, &Mass, &mut Velocity)>,
    planet_query: Query<(&Transform, &Mass), With<Planet>>,
    time: Res<Time>,
) {
    // Get elapsed time
    let elapsed_time = time.delta_seconds();

    // Get the planet
    let (tp, mp) = planet_query.get_single().unwrap();

    for (ta, ma, mut va) in asteroids_query.iter_mut() {
        // Get squared absolute distance between planet and asteroid
        let dist_sq = tp.translation.distance_squared(ta.translation);

        // Calculate force of gravity
        let f = (G * mp.mass * ma.mass) / dist_sq;

        // Calculate acceleration due to gravity
        let acceleration = f / ma.mass;

        // Calculate component acceleration
        let theta = (tp.translation.y - ta.translation.y)
            .to_radians()
            .atan2((tp.translation.x - ta.translation.x).to_radians());
        let acceleration_x = theta.cos() * acceleration;
        let acceleration_y = theta.sin() * acceleration;

        // Accelerate
        va.x += acceleration_x * elapsed_time;
        va.y += acceleration_y * elapsed_time;
    }
}

pub fn rotate_body(mut query: Query<(&mut Transform, &AngularVelocity)>, time: Res<Time>) {
    for (mut transform, omega) in query.iter_mut() {
        transform.rotate_z(omega.velocity * time.delta_seconds())
    }
}

/// Update the minimum and maximum radii of asteroids' orbits.
///
/// # Arguments
/// * `asteroids_query` - query to get asteroid coordinates and orbit
/// * `planet_query` - query to get the coordinates of the planet
pub fn update_orbits(
    mut asteroids_query: Query<(&Transform, &mut Orbit), With<Asteroid>>,
    planet_query: Query<&Transform, With<Planet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let planet = planet_query.get_single().unwrap();
    let window = window_query.get_single().unwrap();

    for (asteroid, mut orbit) in asteroids_query.iter_mut() {
        let distance = asteroid
            .translation
            .distance_squared(planet.translation)
            .sqrt();

        // update orbit.r_min
        if distance.between(ASTEROID_RADIUS + PLANET_RADIUS, orbit.r_min) {
            orbit.r_min = distance;
            continue;
        }

        // update orbit.r_max
        if distance.between(orbit.r_min, (window.height() / 2f32) - ASTEROID_RADIUS)
            && distance.between(orbit.r_min, (window.width() / 2f32) - ASTEROID_RADIUS)
            && distance > orbit.r_max
        {
            orbit.r_max = distance;
        }
    }
}

pub fn spawn_moon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asteroids_query: Query<
        (
            Entity,
            &Transform,
            &Orbit,
            &AngularVelocity,
            &Velocity,
            &Mass,
        ),
        With<Asteroid>,
    >,
) {
    for (entity, transform, orbit, angular_velocity, velocity, mass) in asteroids_query.iter() {
        if orbit.is_elliptical() {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Circle::new(ASTEROID_RADIUS).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    transform: transform.clone(),
                    ..default()
                },
                Moon {},
                AngularVelocity {
                    velocity: angular_velocity.velocity,
                },
                Velocity {
                    x: velocity.x,
                    y: velocity.y,
                },
                Mass { mass: mass.mass },
            ));
            commands.entity(entity).despawn()
        }
    }
}
