use bevy::app::{App, Plugin, Update};
use bevy::log::tracing_subscriber::fmt::format;
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, Res, ResMut, Resource};
use bevy_egui::{egui, EguiContext, EguiContexts};
use chrono::{NaiveTime, Timelike};
use egui_extras::DatePickerButton;
use crate::setup::ScenarioData;
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::components::speed::Speed;
use crate::simulation::SimState;
use crate::simulation::ui::bottom_bar::get_date_from_seconds;
use crate::simulation::ui::scenario_selection::{SelectedScenario, SelectionState};
use crate::simulation::units::text_formatter::format_seconds;
use crate::utils::sim_state_type_editor;

pub struct MetadataPlugin;

impl Plugin for MetadataPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<ShowMetadata>()
            .add_systems(Update, metadata_editor.run_if(sim_state_type_editor));
    }
}

#[derive(Default, Resource)]
pub struct ShowMetadata(pub bool);

fn metadata_editor(
    mut scenario_data: ResMut<ScenarioData>,
    mut scale: ResMut<SimulationScale>,
    mut show_metadata: ResMut<ShowMetadata>,
    mut egui_context: EguiContexts,
    mut speed: ResMut<Speed>,
) {
    egui::Window::new("Metadata Editor")
        .open(&mut show_metadata.0)
        .collapsible(true)
        .constrain(true)
        .scroll2([true, true])
        .auto_sized()
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Title");
                ui.text_edit_singleline(&mut scenario_data.title);
            });
            ui.horizontal(|ui| {
                ui.label("Description");
                ui.text_edit_multiline(&mut scenario_data.description);
            });
            let current_date = get_date_from_seconds(scenario_data.starting_time_millis, 0.0);
            let mut new_date = current_date.clone().date();
            ui.horizontal(|ui| {
                ui.label("Starting Date");
                ui.add(DatePickerButton::new(&mut new_date));
            });
            let mut hour = current_date.time().hour();
            let mut minute = current_date.time().minute();
            let mut second = current_date.time().second();
            ui.horizontal(|ui| {
                ui.label("Starting Time");
                ui.add(egui::DragValue::new(&mut hour).range(0..=23));
                ui.label(":");
                ui.add(egui::DragValue::new(&mut minute).range(0..=59));
                ui.label(":");
                ui.add(egui::DragValue::new(&mut second).range(0..=59));
            });
            let changed_date = new_date.and_time(NaiveTime::from_hms_opt(hour, minute, second).unwrap());
            if changed_date != current_date {
                scenario_data.starting_time_millis = changed_date.timestamp_millis();
            }
            ui.horizontal(|ui| {
                ui.label("Default Timestep (in seconds)");
                ui.add(egui::DragValue::new(&mut speed.0));
                ui.label(format!("({}/step)", format_seconds(speed.0)));
            });
            ui.horizontal(|ui| {
                ui.label("Simulation Scale").on_hover_text("Only applied on simulation start");
                ui.add(egui::DragValue::new(&mut scale.0).min_decimals(20));
            });
            ui.label(format!("(1m = {} units)", 1. / scale.0));
        });
}