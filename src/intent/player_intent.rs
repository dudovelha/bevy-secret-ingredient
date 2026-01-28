use bevy::prelude::{Resource, Vec3};

#[derive(Default, Resource)]
pub struct PlayerIntent {
    pub move_x: i8,
    pub move_y: i8,
    pub jump: bool,
}

impl PlayerIntent {
    pub fn clear(&mut self) {
        self.move_x = 0;
        self.move_y = 0;
        self.jump = false;
    }
}