use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Planet {}

#[derive(Component, Default)]
pub(crate) struct Mass {
    pub mass: f32,
}