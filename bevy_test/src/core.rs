use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Gluable;

#[derive(Component)]
pub struct Sphere {
    pub color: Color,
    pub radius: f32,
    pub border: f32,
}
