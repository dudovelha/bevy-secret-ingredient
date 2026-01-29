use crate::intent::player_intent::PlayerIntent;
use bevy::app::FixedUpdate;
use bevy::log;
use bevy::prelude::{App, ButtonInput, KeyCode, Plugin, Res, ResMut, Update};

pub struct IntentPlugin;

impl Plugin for IntentPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerIntent>()
            .add_systems(Update, apply_input);
    }
}

fn apply_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut intent: ResMut<PlayerIntent>) {
    if keys.just_pressed(KeyCode::Space) {
        intent.jump = true;
    }
    if keys.pressed(KeyCode::KeyW) {
        intent.move_y -= 1;
    }
    if keys.pressed(KeyCode::KeyS) {
        intent.move_y += 1;
    }
    if keys.pressed(KeyCode::KeyA) {
        intent.move_x -= 1;
    }
    if keys.pressed(KeyCode::KeyD) {
        intent.move_x += 1;
    }
}