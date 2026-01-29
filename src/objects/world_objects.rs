use crate::objects::object::{ObjectId, WorldObject};
use bevy::prelude::Resource;
use std::collections::HashMap;

#[derive(Default, Resource)]
pub struct WorldObjects {
    pub next_id: ObjectId,
    pub objects: HashMap<ObjectId, WorldObject>,
}