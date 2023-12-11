use std::time::Duration;

use bevy::app::{App, Plugin};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{Resource, Entity, Update, IntoSystemConfigs, in_state, Res, ResMut, Query, Name, With};
use bevy_egui::egui::RichText;
use bevy_egui::{egui::{self, InnerResponse, Response, Ui}, EguiContexts};

use crate::SimState;
use crate::body::Mass;
use crate::physics::{NBodyStats, SubSteps, NBODY_TOTAL_TIME, NBODY_STEP_TIME};
use crate::ui::{system_ui, UiState};


pub struct ConstellationPlugin;

impl Plugin for ConstellationPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<ConstellationState>()
            .init_resource::<ConstellationUiState>()
            .add_systems(Update, (constellation_window.after(system_ui)).run_if(in_state(SimState::Simulation)));
    }

}

#[derive(Resource, Default)]
struct ConstellationState {
    pub running: bool,
    pub constellation_type: Option<ConstellationTypeState>
}

#[derive(Resource, Default)]
struct ConstellationUiState {
    pub constellation_type: ConstellationUiType
}

enum ConstellationTypeState {
    Apoapsis(Entity),
    Periapsis(Entity),
    UpperConjunction(Entity, Entity),
    LowerConjunction(Entity, Entity),
    Opposition(Entity, Entity)
}

#[derive(Debug, Default, PartialEq)]
enum ConstellationUiType {
    Apoapsis,
    Periapsis,
    UpperConjunction,
    LowerConjunction,
    Opposition,
    #[default]
    None
}

fn constellation_window(
    mut egui_ctx: EguiContexts,
    mut top_ui_state: ResMut<UiState>,
    bodies: Query<(Entity, &Name), With<Mass>>,
    mut ui_state: ResMut<ConstellationUiState>
) {
    if !top_ui_state.visible {
        return;
    }
    egui::Window::new("Constellation Tool")
        .open(&mut top_ui_state.show_constellation)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                let const_type = &ui_state.constellation_type;
                egui::ComboBox::from_label("Type")
                    .selected_text(format!("{const_type:?}"))
                    .show_ui(ui, |ui| {
               //         ui.style_mut().wrap = Some(false);
             //           ui.set_min_width(60.0);
                        ui.selectable_value(&mut ui_state.constellation_type, ConstellationUiType::Apoapsis, "Apoapsis");
                        ui.selectable_value(&mut ui_state.constellation_type, ConstellationUiType::Periapsis, "Periapsis");
                        ui.selectable_value(&mut ui_state.constellation_type, ConstellationUiType::UpperConjunction, "Upper Conjunction");
                        ui.selectable_value(&mut ui_state.constellation_type, ConstellationUiType::LowerConjunction, "Lower Conjunction");
                        ui.selectable_value(&mut ui_state.constellation_type, ConstellationUiType::Opposition, "Opposition");
                    });
                if ui.button("Start").clicked() {
                    
                }
            })
    });
}