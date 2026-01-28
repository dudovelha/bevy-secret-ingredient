use crate::grid::grid::{CELL_SIZE_F, VoxelGrid};
use crate::grid::grid_assets::GridAssets;
use bevy::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridAssets>()
            .init_resource::<VoxelGrid>()
            .add_systems(Startup, (load_assets, spawn_voxel_visuals).chain());
    }
}

fn load_assets(
    grid: ResMut<VoxelGrid>,
    mut grid_assets: ResMut<GridAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    grid_assets.load_grid(grid.into_inner());
    grid_assets.load_tile_materials(&mut materials);
}

fn spawn_voxel_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    grid: Res<VoxelGrid>,
    grid_assets: ResMut<GridAssets>,
) {
    let half_size = Vec3::splat(CELL_SIZE_F * 0.5);
    let origin = Vec3::new(
        grid.size as f32 * 0.5 * CELL_SIZE_F,
        CELL_SIZE_F/2.0,
        grid.size as f32 * 0.5 * CELL_SIZE_F,
    );
    let mesh = meshes.add(Cuboid { half_size });
    for (grid_position, tile_data) in grid.tiles.iter() {
        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(grid_assets.get_tile_material(tile_data.tile_type).unwrap().clone()),
            Transform::from_translation(grid_position.to_world() - origin),
            Visibility::default(),
        ));
    }
}
