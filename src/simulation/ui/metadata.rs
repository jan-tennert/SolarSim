use crate::simulation::components::anise::AlmanacHolder;
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::components::speed::Speed;
use crate::simulation::scenario::loading::LoadingState;
use crate::simulation::scenario::setup::ScenarioData;
use crate::simulation::ui::bottom_bar::get_date_from_seconds;
use crate::simulation::ui::toast::{error_toast, success_toast, ToastContainer};
use crate::simulation::units::text_formatter::format_seconds;
use crate::utils::sim_state_type_editor;
use anise::almanac::Almanac;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoSystemConfigs, ResMut, Resource};
use bevy_async_task::AsyncTaskRunner;
use bevy_egui::egui::{Button, ComboBox};
use bevy_egui::{egui, EguiContexts};
use chrono::{NaiveTime, Timelike};
use egui_extras::DatePickerButton;
use std::fs;
use std::path::Path;
use std::task::Poll;

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
    mut almanac_holder: ResMut<AlmanacHolder>,
    mut task_executor: AsyncTaskRunner<Result<(Almanac, String), String>>,
    mut loading_state: ResMut<LoadingState>
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
            edit_spk_files(ui, &mut scenario_data, &mut selected_spk_file, &mut new_spk_file, &mut toasts, &mut almanac_holder, &mut task_executor, &mut loading_state);
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
        if ui.button("Update bodies (TODO)").on_hover_text("Update bodies to new date").clicked() {

        }
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
    selected_spice_file: &mut String,
    new_spice_file: &mut String,
    toasts: &mut ToastContainer,
    almanac_holder: &mut AlmanacHolder,
    task_executor: &mut AsyncTaskRunner<Result<(Almanac, String), String>>,
    loading_state: &mut ResMut<LoadingState>
) {
    let mut loading = false;
    match task_executor.poll() {
        Poll::Pending => {
            loading = true;
        }
        Poll::Ready(v) => {
            match v {
                Ok(e) => {
                    if let Ok((almanac, name)) = e {
                        scenario_data.spice_files.insert(name, true);
                        almanac_holder.0 = almanac;
                        toasts.0.add(success_toast("SPICE file loaded"));
                    } else {
                        toasts.0.add(error_toast("Failed to load SPICE file"));
                    }
                }
                Err(e) => {
                    toasts.0.add(error_toast(format!("Couldn't load SPICE file: {}", e).as_str()));
                }
            }
        }
        _ => {}
    }
    ui.heading("SPICE Files");
    ui.horizontal(|ui| {
        let mut selected = selected_spice_file.clone();
        if selected.is_empty() {
            selected = "None".to_string();
        }
        ui.label("Loaded SPICE Files:");
        ComboBox::from_label("").selected_text(selected).show_ui(ui, |ui| {
            for (path, loaded) in scenario_data.spice_files.clone() {
                ui.selectable_value(selected_spice_file, path.clone(), format!("{} ({})", path, if loaded { "Loaded" } else { "Not Loaded" }));
            }
        });
        if ui.button("Remove").on_hover_text("Remove selected SPICE file").clicked() {
            scenario_data.spice_files.remove(selected_spice_file);
            *selected_spice_file = "".to_string();
            toasts.0.add(success_toast("SPICE file removed"));
            loading_state.reload_spice_files();
        }
    });
    ui.text_edit_singleline(new_spice_file);
    ui.horizontal(|ui| {
        if ui.button("Select SPICE File").clicked() {
            match tinyfiledialogs::open_file_dialog("Select SPICE file", "data.bsp", Some((&["*.bsp", "*.pca"], "SPICE files"))) {
                Some(file) => {
                    *new_spice_file = file;
                },
                None => {
                    toasts.0.add(error_toast("No file selected"));
                },
            }
        }
        let loading_button = ui.add_enabled(!loading && loading_state.loaded_spice_files, Button::new("Load SPICE File"));
        if loading_button.clicked() {
            task_executor.start(load_scenario_file(scenario_data.clone(), new_spice_file.clone(), almanac_holder.0.clone()));
        }
        let reload_button = ui.add_enabled(!loading && loading_state.loaded_spice_files, Button::new("Reload SPICE Files"));
        if reload_button.clicked() {
            loading_state.reload_spice_files();
        }
        if loading || !loading_state.loaded_spice_files {
            ui.spinner();
        }
    });
}

async fn load_scenario_file(
    scenario_data: ScenarioData,
    new_spk_file: String,
    almanac: Almanac
) -> Result<(Almanac, String), String> {
    if !fs::exists("data").unwrap_or(false) {
        fs::create_dir("data").map_err(|_| "Failed to create data directory".to_string())?;
    }
    let file_name = Path::new(&new_spk_file).file_name().and_then(|f| f.to_str()).map(|s| s.to_string()).ok_or("Invalid file name")?;
    let data_path = format!("data/{}", file_name);
    let exists = fs::exists(new_spk_file.clone()).unwrap_or(false) || fs::exists(data_path.clone()).unwrap_or(false);
    if !exists {
        return Err("File not found".to_string());
    }
    if new_spk_file.is_empty() {
        return Err("No file selected".to_string());
    }
    let mut copied = false;
    if !fs::exists(&data_path).unwrap_or(false) {
        copied = true;
        fs::copy(new_spk_file.clone(), &data_path).map_err(|_| "Failed to copy file".to_string())?;
    } else if scenario_data.spice_files.contains_key(&file_name) {
        return Err("SPICE file already added".to_string());
    }
    match load_spk(data_path.clone(), &almanac) {
        Ok(almanac) => {
            Ok((almanac, file_name))
        },
        Err(e) => {
            if copied {
                fs::remove_file(data_path).map_err(|_| "Failed to remove file".to_string())?;
            }
            Err("Failed to load SPICE file".to_string())
        }
    }
}

fn load_spk(path: String, almanac: &Almanac) -> Result<Almanac, ()> {
    let almanac = almanac.load(&*path).map_err(|_| ())?;
    Ok(almanac)
}