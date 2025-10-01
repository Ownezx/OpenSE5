use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct TurnData {
    current_turn: isize,
    pub has_finished_movement: bool,
}

impl TurnData {
    pub fn increment_turn(&mut self) {
        self.current_turn += 1;
    }

    pub fn get_current_turn(&self) -> isize {
        self.current_turn
    }
}
