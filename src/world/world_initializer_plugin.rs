use crate::grid::grid::VoxelGrid;
use crate::objects::world_objects::WorldObjects;
use crate::world::world_assets::WorldAssets;
use crate::world::world_resources::WorldResources;
use crate::world::world_spawner::WorldSpawner;
use bevy::app::App;
use bevy::asset::Assets;
use bevy::mesh::Mesh;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Commands, IntoScheduleConfigs, Plugin, Res, ResMut, Startup};

pub struct WorldInitializerPlugin;

impl Plugin for WorldInitializerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldAssets>()
            .init_resource::<VoxelGrid>()
            .init_resource::<WorldObjects>()
            .add_systems(
                Startup,
                (load_assets, initialize_grid, initialize_objects).chain(),
            );
    }
}

fn load_assets(
    mut world_assets: ResMut<WorldAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    world_assets.load_assets(&mut materials);
}

fn initialize_grid(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    world_assets: ResMut<WorldAssets>,
    mut grid: ResMut<VoxelGrid>,
) {
    WorldResources::load_grid(&mut grid, "assets/grid/level1.json");
    WorldSpawner::spawn_grid(commands, meshes, world_assets, grid.into());
}

fn initialize_objects(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    world_assets: ResMut<WorldAssets>,
    grid: ResMut<VoxelGrid>,
    mut world_objects: ResMut<WorldObjects>,
) {
    WorldSpawner::spawn_objects(commands, meshes, world_assets, grid, &mut world_objects);
}
