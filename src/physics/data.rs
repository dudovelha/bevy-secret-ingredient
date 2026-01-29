use std::ops::{Deref, DerefMut};
use bevy::prelude::{Component, Deref, DerefMut, Vec3};

#[derive(Debug, Copy, Clone, Component, Deref, DerefMut)]
pub struct Position (pub Vec3);

#[derive(Debug, Copy, Clone, Component, Deref, DerefMut)]
pub struct Velocity (pub Vec3);

#[derive(Debug, Copy, Clone, Component)]
pub struct PhysicsFlags {
    pub(crate) on_ground: bool,
    was_on_ground: bool,
}

#[derive(Debug, Copy, Clone, Component)]
pub struct Collider {
    pub half_extents: Vec3,
}

impl PhysicsFlags {
    pub fn new() -> Self {
        Self { on_ground: false, was_on_ground: false }
    }
    pub fn is_grounded(&self) -> bool {
        self.was_on_ground
    }

    pub(crate) fn begin_tick(&mut self) {
        self.was_on_ground = self.on_ground;
        self.on_ground = false;
    }
}