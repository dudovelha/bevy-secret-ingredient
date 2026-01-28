use bevy::prelude::Vec3;

pub trait PhysicsWorld {
    fn is_aabb_blocked(&self, min: Vec3, max: Vec3) -> bool;
}
