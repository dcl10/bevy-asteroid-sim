use bevy::prelude::*;

#[derive(Component)]
pub struct Planet {}

#[derive(Component, Default)]
pub struct Mass {
    pub mass: f32,
}