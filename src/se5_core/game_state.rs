use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SE5GameState {
    #[default]
    Playing,
    ProcessingTurn,
}

pub struct SE5States;

impl Plugin for SE5States {
    fn build(&self, app: &mut App) {
        app.init_state::<SE5GameState>();
    }
}