use std::ops::{Deref, DerefMut};
use bevy::prelude::{Component, Deref, DerefMut, Vec3};

#[derive(Debug, Copy, Clone, Component, Deref, DerefMut)]
pub struct Position (pub Vec3);

#[derive(Debug, Copy, Clone, Component, Deref, DerefMut)]
pub struct Velocity (pub Vec3);

#[derive(Debug, Copy, Clone, Component)]
pub struct PhysicsFlags {
    pub on_ground: bool,
    pub was_on_ground: bool,
}

#[derive(Debug, Copy, Clone, Component)]
pub struct Collider {
    pub half_extents: Vec3,
}