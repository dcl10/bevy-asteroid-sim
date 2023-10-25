use bevy::prelude::*;

#[derive(Component)]
pub struct Planet {}

#[derive(Component, Default)]
pub struct Mass {
    pub mass: f32,
}

#[derive(Component)]
pub struct Asteroid {}

#[derive(Component)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
pub struct Velocity {
    x: f32,
    y: f32,
}