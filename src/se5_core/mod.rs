use bevy::prelude::*;

use crate::se5_core::game_state::SE5States;

pub mod game_state;

pub struct SE5Core;

impl Plugin for SE5Core {
    fn build(&self, app: &mut App) {
        app.add_plugins(SE5States);
    }
}