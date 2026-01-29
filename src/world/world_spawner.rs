use crate::grid::grid::{VoxelGrid, CELL_SIZE_F};
use crate::grid::tile::GridTile;
use crate::world::world_assets::WorldAssets;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::mesh::{Mesh, Mesh3d};
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{Commands, Cuboid, Res, ResMut, Transform, Visibility};

pub struct WorldSpawner;

impl WorldSpawner {
    pub fn spawn_grid(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        world_assets: ResMut<WorldAssets>,
        grid: Res<VoxelGrid>,
    ) {
        let half_size = Vec3::splat(CELL_SIZE_F * 0.5);
        let mesh = meshes.add(Cuboid { half_size });
        
        for (grid_position, tile_data) in grid.tiles.iter() {
            commands.spawn((
                GridTile,
                Mesh3d(mesh.clone()),
                MeshMaterial3d(world_assets.grid_assets.get(&tile_data.tile_type).unwrap().clone()),
                Transform::from_translation(grid_position.to_world(grid.origin)),
                Visibility::default(),
            ));
        }
    }
}