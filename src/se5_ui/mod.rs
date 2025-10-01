use bevy::prelude::*;

use crate::se5_ui::next_turn_button::NextTurnButtonPlugin;

mod next_turn_button;
pub struct SE5Ui;

impl Plugin for SE5Ui {
    fn build(&self, app: &mut App) {
        app.add_plugins(NextTurnButtonPlugin);
    }
}