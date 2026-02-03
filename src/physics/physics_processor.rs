use crate::intent::player_intent::PlayerIntent;
use crate::physics::data::{Collider, PhysicsFlags, Position, Velocity};
use crate::physics::physics_world::PhysicsWorld;
use bevy::log;
use bevy::prelude::Vec3;
use crate::objects::object::ObjectId;
use crate::objects::world_objects::WorldObjects;

const GROUND_EPSILON: Vec3 = Vec3::new(0.0, -0.05, 0.0);
const GRAVITY: f32 = 9.8;
const VELOCITY: f32 = 5.0;
const VELOCITY_AIR: f32 = 3.0;
const JUMP_IMPULSE: f32 = 10.0;

pub struct PhysicsProcessor;

impl PhysicsProcessor {
    pub fn apply_input(velocity: &mut Velocity, flags: &PhysicsFlags, intent: &PlayerIntent) {
        let velocity_constant = (if flags.is_grounded() {
            VELOCITY
        } else {
            VELOCITY_AIR
        });
        velocity.x = intent.move_x as f32 * velocity_constant;
        velocity.z = intent.move_y as f32 * velocity_constant;

        if intent.jump && flags.is_grounded() {
            velocity.y = JUMP_IMPULSE;
        }
    }

    pub fn apply_gravity(velocity: &mut Velocity, flags: &PhysicsFlags, delta: f32) {
        if !flags.is_grounded() {
            velocity.y -= GRAVITY * delta;
        }
    }

    pub fn move_and_collide(
        position: &mut Position,
        velocity: &mut Velocity,
        collider: &Collider,
        flags: &mut PhysicsFlags,
        world: &dyn PhysicsWorld,
        dt: f32,
    ) {
        Self::move_axis_y(Vec3::Y, position, velocity, collider, flags, world, dt);
        Self::move_axis(Vec3::Z, position, velocity, collider, world, dt);
        Self::move_axis(Vec3::X, position, velocity, collider, world, dt);
    }

    fn ground_check(
        position: &mut Position,
        velocity: &mut Velocity,
        collider: &Collider,
        flags: &mut PhysicsFlags,
        world: &dyn PhysicsWorld,
    ) {
        if velocity.y <= 0.0
            && world.is_aabb_blocked(position.0 + GROUND_EPSILON, collider.half_extents)
        {
            velocity.y = 0.0;
            flags.on_ground = true;
            log::info!("Player is on ground! vel: {}", velocity.y);
        } else {
            flags.on_ground = false;
        }
    }

    fn move_axis_y(
        axis: Vec3,
        position: &mut Position,
        velocity: &mut Velocity,
        collider: &Collider,
        flags: &mut PhysicsFlags,
        world: &dyn PhysicsWorld,
        dt: f32,
    ) {
        let axis = axis.normalize();
        let delta = velocity.0.dot(axis) * dt;

        if delta.abs() < 1e-6 {
            return;
        }

        let new_pos = position.0 + axis * delta;
        let (min, max) = Self::aabb_at(new_pos, collider.half_extents);

        if world.is_aabb_blocked(min, max) {
            let mask = Vec3::ONE - axis.abs();
            velocity.0 *= mask;
            flags.on_ground = true;
        } else {
            position.0 = new_pos;
        }
    }

    fn move_axis(
        axis: Vec3,
        position: &mut Position,
        velocity: &mut Velocity,
        collider: &Collider,
        world: &dyn PhysicsWorld,
        dt: f32,
    ) {
        let axis = axis.normalize();
        let delta = velocity.0.dot(axis) * dt;

        if delta.abs() < 1e-6 {
            return;
        }

        let new_pos = position.0 + axis * delta;
        let (min, max) = Self::aabb_at(new_pos, collider.half_extents);

        if world.is_aabb_blocked(min, max) {
            let mask = Vec3::ONE - axis.abs();
            velocity.0 *= mask;
        } else {
            position.0 = new_pos;
        }
    }

    fn aabb_at(pos: Vec3, half: Vec3) -> (Vec3, Vec3) {
        (pos - half, pos + half)
    }
}
