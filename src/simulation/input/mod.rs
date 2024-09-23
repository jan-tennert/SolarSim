mod input;
mod egui_input_block;

use bevy::prelude::{in_state, App, IntoSystemConfigs, Plugin, Update};
use crate::simulation::SimState;
use crate::simulation::input::egui_input_block::BlockInputPlugin;
use crate::simulation::input::input::{global_input_system, key_window, sim_input_system};

pub struct SimInputPlugin;

impl Plugin for SimInputPlugin {
    
    fn build(&self, app: &mut App) {
        app
            .add_plugins(BlockInputPlugin)
            .add_systems(Update, global_input_system)
            .add_systems(Update, key_window.run_if(in_state(SimState::Loaded)))
            .add_systems(Update, sim_input_system.run_if(in_state(SimState::Loaded)));
    }
    
}