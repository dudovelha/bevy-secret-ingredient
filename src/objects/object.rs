use bevy::prelude::{Entity, Vec3};
use crate::physics::data::{Collider, Position};

pub type ObjectId = u32;

pub struct WorldObject {
    pub id: ObjectId,
    pub entity: Entity,
    pub position: Position,
    pub collider: Collider,
    pub kind: ObjectKind,
}

pub enum ObjectKind {
    StaticObstacle,
    Pickup,
    Trigger,
}