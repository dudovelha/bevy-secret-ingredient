mod environment;
mod grid;
mod physics;
mod intent;
mod player;

use crate::grid::grid_plugin::GridPlugin;
use crate::intent::intent_plugin::IntentPlugin;
use crate::physics::physics_plugin::PhysicsPlugin;
use crate::player::player_plugin::PlayerPlugin;
use bevy::prelude::*;
use environment::camera::CameraPlugin;
use environment::light::LightPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraPlugin, LightPlugin))
        .add_plugins(GridPlugin)
        .add_plugins(IntentPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
