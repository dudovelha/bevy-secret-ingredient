use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 20.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, CAMERA_DISTANCE, CAMERA_DISTANCE/2.0)).looking_at(Vec3::Y, Vec3::Y)
    ));
}