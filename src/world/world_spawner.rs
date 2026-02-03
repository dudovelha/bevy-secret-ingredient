use crate::grid::grid::{VoxelGrid, CELL_SIZE_F};
use crate::grid::grid_position::GridPosition;
use crate::grid::tile::{GridTile, TileType};
use crate::objects::object::{GameObject, ObjectId, ObjectKind, WorldObject};
use crate::objects::world_objects::WorldObjects;
use crate::physics::data::{Collider, Position};
use crate::world::world_assets::WorldAssets;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::mesh::{Mesh, Mesh3d};
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{Commands, Cuboid, Cylinder, Res, ResMut, Transform, Visibility};

pub struct WorldSpawner;

impl WorldSpawner {
    pub(crate) fn spawn_grid(
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
                MeshMaterial3d(
                    world_assets
                        .grid_assets
                        .get(&tile_data.tile_type)
                        .unwrap()
                        .clone(),
                ),
                Transform::from_translation(grid_position.to_world(grid.origin)),
                Visibility::default(),
            ));
        }
    }

    pub(crate) fn spawn_objects(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        world_assets: ResMut<WorldAssets>,
        mut grid: ResMut<VoxelGrid>,
        world_objects: &mut ResMut<WorldObjects>,
    ) {
        let translation = Vec3::new(2.0, 0.5, 2.0);
        let entity = commands.spawn((
            GameObject,
            Transform::from_translation(translation),
            Mesh3d(meshes.add(Cylinder {
                radius: 0.2,
                half_height: 0.4,
            })),
            MeshMaterial3d(
                world_assets
                    .grid_assets
                    .get(&TileType::Stone)
                    .unwrap()
                    .clone()
            ),
        ));
        world_objects.objects.insert(
            1,
            WorldObject {
                id: 1,
                //entity,
                position: Position(translation),
                collider: Collider {
                    half_extents: Vec3::splat(0.2),
                },
                kind: ObjectKind::StaticObstacle,
            },
        );
        grid.objects.insert(
            GridPosition {
                x: translation.x as i32,
                y: translation.y as i32,
                z: translation.z as i32,
            },
            vec![1]
        );
    }
}
