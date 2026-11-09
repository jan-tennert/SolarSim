use crate::simulation::{SimState, SimStateType};
use bevy::prelude::{Res, State};
use bevy_egui::egui::{LayerId, Ui, UiBuilder};
use bevy_egui::{egui, EguiContext};

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

pub fn create_viewport_ui(ctx: &mut egui::Context) -> Ui {
    Ui::new(
        ctx.clone(),
        "viewport".into(),
        UiBuilder::new()
            .layer_id(LayerId::background())
            .max_rect(ctx.viewport_rect()),
    )
}