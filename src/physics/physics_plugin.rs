use bevy::log;
use crate::grid::grid::VoxelGrid;
use crate::intent::player_intent::PlayerIntent;
use crate::physics::data::{Collider, PhysicsFlags, Position, Velocity};
use crate::physics::physics_processor::PhysicsProcessor;
use crate::physics::physics_world::PhysicsWorld;
use crate::player::player::Player;
use bevy::prelude::*;
use crate::objects::world_objects::WorldObjects;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                reset_physics_flags,
                apply_input,
                apply_gravity,
                move_kinematic_bodies,
            )
                .chain(),
        );
    }
}

fn reset_physics_flags(mut query: Query<&mut PhysicsFlags>) {
    for mut flags in &mut query {
        flags.begin_tick();
    }
}

fn apply_input(
    mut intent: ResMut<PlayerIntent>,
    mut query: Query<(&mut Velocity, &PhysicsFlags), With<Player>>,
) {
    if let Ok((velocity, flags)) = query.single_mut() {
        PhysicsProcessor::apply_input(velocity.into_inner(), flags, &intent);
        intent.clear()
    }
}

fn apply_gravity(mut query: Query<(&mut Velocity, &mut PhysicsFlags)>, time: Res<Time>) {
    let delta = time.delta_secs();
    for (velocity, flags) in &mut query.iter_mut() {
        PhysicsProcessor::apply_gravity(velocity.into_inner(), flags.into_inner(), delta);
    }
}
fn move_kinematic_bodies(
    mut query: Query<
        (
            &mut Transform,
            &mut Position,
            &mut Velocity,
            &Collider,
            &mut PhysicsFlags,
        ),
        With<Player>,
    >,
    grid: Res<VoxelGrid>,
    time: Res<Time>,
) {
    let world: &dyn PhysicsWorld = grid.into_inner();
    let dt = time.delta_secs();

    for (mut transform, mut position, mut velocity, collider, mut flags) in &mut query {
        PhysicsProcessor::move_and_collide(
            &mut position,
            &mut velocity,
            collider,
            &mut flags,
            world,
            dt,
        );

        transform.translation = position.0;
    }
}
