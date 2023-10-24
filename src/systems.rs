use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;

use crate::components::{Mass, Planet};

const PLANET_SIZE: f32 = 50.0;
const PLANET_MASS: f32 = 1.9e27;

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
    // Circle
    commands.spawn(
        (
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(PLANET_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                ..default()
            },
            Planet {},
            Mass { mass: PLANET_MASS }
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