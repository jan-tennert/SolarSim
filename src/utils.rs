use crate::simulation::{SimState, SimStateType};
use bevy::prelude::{Res, State};

pub fn sim_state_type_simulation(
    res: Res<SimStateType>,
    state: Res<State<SimState>>
) -> bool {
    *res == SimStateType::Simulation && *state == SimState::Loaded
}

pub fn sim_state_type_editor(
    res: Res<SimStateType>,
    state: Res<State<SimState>>
) -> bool {
    *res == SimStateType::Editor && *state == SimState::Loaded
}