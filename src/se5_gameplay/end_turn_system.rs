use bevy::prelude::*;

use crate::{se5_core::game_state::SE5GameState, se5_resources::turn_data::TurnData};

pub struct EndTurnSystem;

impl Plugin for EndTurnSystem {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, end_turn);
    }
}

fn end_turn(
    current_state: Res<State<SE5GameState>>,
    mut next_state: ResMut<NextState<SE5GameState>>,
    mut turn_data: ResMut<TurnData>,
) {
    if *current_state != SE5GameState::ProcessingTurn {
        return;
    }

    if true
    // turn_data.has_finished_movement // <- need movement to be done
    {
        turn_data.increment_turn();
        info!("Processed turn {}", turn_data.get_current_turn());
        next_state.set(SE5GameState::Playing);
    }
}
