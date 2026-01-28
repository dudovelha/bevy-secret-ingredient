use crate::grid::grid::VoxelGrid;
use crate::physics::data::{Collider, PhysicsFlags, Position, Velocity};
use crate::physics::physics_processor::PhysicsProcessor;
use crate::player::player::Player;
use bevy::app::{App, FixedUpdate};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::mesh::{Mesh, Mesh3d};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Commands, Plugin, Query, Res, ResMut, Sphere, Startup, Transform, Vec3, With};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character)
            .add_systems(FixedUpdate, move_character);
    }
}

fn spawn_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Sphere { radius: 0.5 });
    let position = Vec3::new(0.0, 10.0, 0.0);

    commands.spawn((
        Player,
        Mesh3d(mesh),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_translation(position),
        Position { 0: position },
        Velocity { 0: Vec3::ZERO },
        Collider {
            half_extents: Vec3::ONE * 0.5,
        },
        PhysicsFlags { on_ground: false },
    ));
}

fn move_character(
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
) {
    for (mut transform, mut position, mut velocity, collider, mut flags) in &mut query {
        // TEMP sanity check
        let dt = 0.016;
        velocity.y -= 9.8 * dt;

        PhysicsProcessor::move_and_collide(
            &mut position,
            &mut velocity,
            collider,
            &mut flags,
            &*grid,
            dt,
        );

        // Sync render
        transform.translation = position.0;
    }
}
