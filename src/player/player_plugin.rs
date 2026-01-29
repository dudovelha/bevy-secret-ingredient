use crate::physics::data::{Collider, PhysicsFlags, Position, Velocity};
use crate::player::player::Player;
use bevy::app::App;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::mesh::{Mesh, Mesh3d};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Commands, Plugin, ResMut, Sphere, Startup, Transform, Vec3};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character);
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
        PhysicsFlags::new(),
    ));
}
