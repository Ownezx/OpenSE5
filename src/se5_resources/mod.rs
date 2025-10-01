use bevy::prelude::*;

use crate::se5_resources::turn_data::TurnData;

pub mod turn_data;
pub struct SE5Resource;

impl Plugin for SE5Resource {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnData>();
    }
}