use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::se5_core::game_state::SE5GameState;

pub struct NextTurnButtonPlugin;

impl Plugin for NextTurnButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, draw_next_turn_button);
    }
}

fn draw_next_turn_button(
    mut contexts: EguiContexts,
    current_state: Res<State<SE5GameState>>,
    mut next_state: ResMut<NextState<SE5GameState>>,
) {
    let ctx = contexts.ctx_mut().unwrap();

    // Determine button label and whether it should be enabled
    let (label, enabled) = match current_state.get() {
        SE5GameState::Playing => ("Next Turn", true),
        SE5GameState::ProcessingTurn => ("Processing...", false),
    };

    egui::Window::new("next_turn_button")
        .anchor(egui::Align2::RIGHT_TOP, [-10.0, 10.0])
        .resizable(false)
        .title_bar(false)
        .show(ctx, |ui| {
            if ui.add_enabled(enabled, egui::Button::new(label)).clicked() {
                next_state.set(SE5GameState::ProcessingTurn);
            }
        });
}
