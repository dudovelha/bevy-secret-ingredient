use bevy::prelude::Vec3;
use crate::intent::player_intent::PlayerIntent;
use crate::physics::data::{Collider, PhysicsFlags, Position, Velocity};
use crate::physics::physics_world::PhysicsWorld;

pub struct PhysicsProcessor;

impl PhysicsProcessor {
    pub fn apply_input(
        velocity: &mut Velocity,
        flags: &PhysicsFlags,
        intent: &PlayerIntent,
    ) {

    }

    pub fn apply_gravity(velocity: &mut Velocity, delta: f32) {}

    pub fn move_and_collide(
        position: &mut Position,
        velocity: &mut Velocity,
        collider: &Collider,
        flags: &mut PhysicsFlags,
        world: &impl PhysicsWorld,
        dt: f32,
    ) {
        flags.on_ground = false;

        //
        // Y AXIS (gravity / ground)
        //
        let dy = velocity.y * dt;
        if dy != 0.0 {
            let new_pos = position.0 + Vec3::new(0.0, dy, 0.0);
            let (min, max) = aabb_at(new_pos, collider.half_extents);

            if world.is_aabb_blocked(min, max) {
                if velocity.y < 0.0 {
                    flags.on_ground = true;
                }
                velocity.y = 0.0;
            } else {
                position.y = new_pos.y;
            }
        }

        //
        // X AXIS
        //
        let dx = velocity.x * dt;
        if dx != 0.0 {
            let new_pos = position.0 + Vec3::new(dx, 0.0, 0.0);
            let (min, max) = aabb_at(new_pos, collider.half_extents);

            if world.is_aabb_blocked(min, max) {
                velocity.x = 0.0;
            } else {
                position.x = new_pos.x;
            }
        }

        //
        // Z AXIS
        //
        let dz = velocity.z * dt;
        if dz != 0.0 {
            let new_pos = position.0 + Vec3::new(0.0, 0.0, dz);
            let (min, max) = aabb_at(new_pos, collider.half_extents);

            if world.is_aabb_blocked(min, max) {
                velocity.z = 0.0;
            } else {
                position.z = new_pos.z;
            }
        }
    }
}

fn aabb_at(pos: Vec3, half: Vec3) -> (Vec3, Vec3) {
    (pos - half, pos + half)
}
