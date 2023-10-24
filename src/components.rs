use bevy::prelude::*;

#[derive(Component)]
struct Planet {}

#[derive(Component, Default)]
struct Mass {
    mass: f32,
}