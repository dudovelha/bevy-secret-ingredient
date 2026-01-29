use bevy::prelude::{App, Commands, Plugin, Startup};
use crate::objects::world_objects::WorldObjects;

struct WorldObjectsPlugin;

impl Plugin for WorldObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WorldObjects>()
            .add_systems(Startup, add_objects);
    }
}

fn add_objects(mut commands: Commands) {

}
