use bevy::prelude::*;

use crate::se5_gameplay::end_turn_system::EndTurnSystem;

mod end_turn_system;
pub struct SE5Gameplay;

impl Plugin for SE5Gameplay {
    fn build(&self, app: &mut App) {
        app.add_plugins(EndTurnSystem);
    }
}