use crate::physics::data::{Collider, Position};
use bevy::prelude::{Component, EntityCommands};

pub type ObjectId = u32;

#[derive(Default, Component)]
pub struct GameObject;

pub struct WorldObject {
    pub id: ObjectId,
    //pub entity: EntityCommands,
    pub position: Position,
    pub collider: Collider,
    pub kind: ObjectKind,
}

pub enum ObjectKind {
    StaticObstacle,
    Pickup,
    Trigger,
}