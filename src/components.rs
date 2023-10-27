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
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct AngularVelocity {
    pub velocity: f32,
}
