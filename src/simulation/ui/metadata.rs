use std::fs;
use std::path::Path;
use anise::almanac::Almanac;
use anise::prelude::SPK;
use bevy::app::{App, Plugin, Update};
use bevy::log::tracing_subscriber::fmt::format;
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, Res, ResMut, Resource};
use bevy_egui::{egui, EguiContext, EguiContexts};
use bevy_egui::egui::ComboBox;
use chrono::{NaiveTime, Timelike};
use egui_extras::DatePickerButton;
use crate::simulation::scenario::setup::ScenarioData;
use crate::simulation::components::anise::{load_spk_files, AlmanacHolder};
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::components::speed::Speed;
use crate::simulation::SimState;
use crate::simulation::ui::bottom_bar::get_date_from_seconds;
use crate::simulation::ui::scenario_selection::{SelectedScenario, SelectionState};
use crate::simulation::ui::toast::{error_toast, success_toast, ToastContainer};
use crate::simulation::units::text_formatter::format_seconds;
use crate::utils::sim_state_type_editor;

pub struct MetadataPlugin;

impl Plugin for MetadataPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<MetadataUiState>()
            .add_systems(Update, metadata_editor.run_if(sim_state_type_editor));
    }
}

#[derive(Default, Resource)]
pub struct MetadataUiState {

    pub show: bool,
    pub selected_spk_file: String,
    pub new_spk_file: String,

}

fn metadata_editor(
    mut scenario_data: ResMut<ScenarioData>,
    mut scale: ResMut<SimulationScale>,
    mut state: ResMut<MetadataUiState>,
    mut egui_context: EguiContexts,
    mut speed: ResMut<Speed>,
    mut toasts: ResMut<ToastContainer>,
    mut almanac_holder: ResMut<AlmanacHolder>
) {
    let mut show = state.show;
    let mut selected_spk_file = state.selected_spk_file.clone();
    let mut new_spk_file = state.new_spk_file.clone();

    egui::Window::new("Metadata Editor")
        .open(&mut show)
        .collapsible(true)
        .constrain(true)
        .scroll2([true, true])
        .auto_sized()
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Basic Information");
            edit_basic_info(ui, &mut scenario_data);
            edit_starting_time(ui, &mut scenario_data);
            edit_simulation_settings(ui, &mut scale, &mut speed);
            edit_spk_files(ui, &mut scenario_data, &mut selected_spk_file, &mut new_spk_file, &mut toasts, &mut almanac_holder);
        });

    state.show = show;
    state.selected_spk_file = selected_spk_file;
    state.new_spk_file = new_spk_file;
}

fn edit_basic_info(ui: &mut egui::Ui, scenario_data: &mut ScenarioData) {
    ui.horizontal(|ui| {
        ui.label("Title");
        ui.text_edit_singleline(&mut scenario_data.title);
    });
    ui.horizontal(|ui| {
        ui.label("Description");
        ui.text_edit_multiline(&mut scenario_data.description);
    });
}

fn edit_starting_time(ui: &mut egui::Ui, scenario_data: &mut ScenarioData) {
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
}

fn edit_simulation_settings(ui: &mut egui::Ui, scale: &mut SimulationScale, speed: &mut Speed) {
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
}

fn edit_spk_files(
    ui: &mut egui::Ui,
    scenario_data: &mut ScenarioData,
    selected_spk_file: &mut String,
    new_spk_file: &mut String,
    toasts: &mut ToastContainer,
    almanac_holder: &mut AlmanacHolder
) {
    ui.heading("SPK Files");
    ui.horizontal(|ui| {
        let mut selected = selected_spk_file.clone();
        if selected.is_empty() {
            selected = "None".to_string();
        }
        ui.label("Included SPK Files:");
        ComboBox::from_label("").selected_text(selected).show_ui(ui, |ui| {
            for file in scenario_data.spk_files.clone() {
                ui.selectable_value(selected_spk_file, file.clone(), file);
            }
        });
        if ui.button("Remove").on_hover_text("Remove selected SPK file").clicked() {
            if let Some(index) = scenario_data.spk_files.iter().position(|f| f == selected_spk_file) {
                scenario_data.spk_files.remove(index);
                *selected_spk_file = "".to_string();
                toasts.0.add(success_toast("SPK file removed"));
                load_spk_files(scenario_data.spk_files.clone(), almanac_holder, toasts);
            }
        }
    });
    ui.text_edit_singleline(new_spk_file);
    ui.horizontal(|ui| {
        if ui.button("Select SPK File").clicked() {
            match tinyfiledialogs::open_file_dialog("Select BSP file", "data.bsp", Some((&["*.bsp"], "BSP files"))) {
                Some(file) => {
                    *new_spk_file = file;
                },
                None => {
                    toasts.0.add(error_toast("No file selected"));
                },
            }
        }
        if ui.button("Load SPK File").clicked() {
            if let Err(e) = load_scenario_file(scenario_data, selected_spk_file, new_spk_file, toasts, almanac_holder) {
                toasts.0.add(error_toast(&e));
            }
        }
    });
}

fn load_scenario_file(
    scenario_data: &mut ScenarioData,
    selected_spk_file: &mut String,
    new_spk_file: &mut String,
    toasts: &mut ToastContainer,
    almanac_holder: &mut AlmanacHolder
) -> Result<(), String> {
    let file_name = Path::new(new_spk_file).file_name().and_then(|f| f.to_str()).map(|s| s.to_string()).ok_or("Invalid file name")?;
    let data_path = format!("data/{}", file_name);
    let exists = fs::exists(new_spk_file.clone()).unwrap_or(false) || fs::exists(data_path.clone()).unwrap_or(false);
    if !exists {
        return Err("File not found".to_string());
    }
    if new_spk_file.is_empty() {
        return Err("No file selected".to_string());
    }
    if !fs::exists(&data_path).unwrap_or(false) {
        fs::copy(new_spk_file.clone(), &data_path).map_err(|_| "Failed to copy file".to_string())?;
    } else if scenario_data.spk_files.contains(&file_name) {
        return Err("SPK file already added".to_string());
    }
    match load_spk(data_path.clone(), &almanac_holder.0) {
        Ok(almanac) => {
            almanac_holder.0 = almanac;
            scenario_data.spk_files.push(file_name.clone());
            *selected_spk_file = file_name;
            *new_spk_file = "".to_string();
            toasts.0.add(success_toast("SPK file added and loaded"));
            Ok(())
        },
        Err(_) => {
            fs::remove_file(data_path).map_err(|_| "Failed to remove file".to_string())?;
            Err("Failed to load SPK file".to_string())
        }
    }
}

fn load_spk(path: String, almanac: &Almanac) -> Result<Almanac, ()> {
    let daf = SPK::load(path.as_str()).map_err(|_| ())?;
    let almanac = almanac.with_spk(daf).map_err(|_| ())?;
    Ok(almanac)
}