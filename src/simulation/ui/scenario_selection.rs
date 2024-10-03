use std::fs;
use std::path::Path;
use bevy::app::{App, Plugin, Update};
use bevy::asset::io::AssetSourceId;
use bevy::asset::{AssetPath, LoadedFolder};
use bevy::prelude::{in_state, AssetServer, Assets, Commands, Handle, Image, IntoSystemConfigs, Local, NextState, OnEnter, OnExit, Res, ResMut, Resource};
use bevy::utils::HashMap;
use bevy_egui::egui::{Align, CentralPanel, Layout, SidePanel, TextureId};
use bevy_egui::{egui, EguiContexts};
use image::load;
use crate::simulation::scenario::setup::ScenarioData;
use crate::simulation::{SimState, SimStateType};
use crate::simulation::asset::{from_scenario_source, SCENARIO_ASSET_SOURCE};
use crate::simulation::asset::serialization::SimulationData;
use crate::simulation::components::anise::{load_spk_files, AlmanacHolder};
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::components::speed::Speed;
use crate::simulation::ui::toast::{error_toast, ToastContainer};

pub struct ScenarioSelectionPlugin;

impl Plugin for ScenarioSelectionPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedScenario>()
            .init_resource::<SelectionState>()
            .add_systems(OnEnter(SimState::ScenarioSelection), load_scenarios)
            .add_systems(Update, (creation_sidebar, show_menu).chain().run_if(in_state(SimState::ScenarioSelection)));
    }
}

#[derive(Resource)]
pub struct ScenarioFolder(pub Handle<LoadedFolder>);

#[derive(Resource, Default)]
pub struct SelectedScenario {

    pub handle: Handle<SimulationData>,
    pub spawned: bool

}

#[derive(Resource, Default, Clone)]
pub struct SelectionState {

    pub show_creation: bool,
    pub title: String,
    pub description: String,
    pub file_name: String,
    pub image_path: String,
    pub delete_confirm: Option<String>

}

fn load_scenarios(
    assets: Res<AssetServer>,
    mut commands: Commands
) {
    let handle = assets.load_folder(from_scenario_source(""));
    commands.insert_resource(ScenarioFolder(handle));
}

fn creation_sidebar(
    mut egui_context: EguiContexts,
    mut selection_state: ResMut<SelectionState>,
    mut toasts: ResMut<ToastContainer>
) {
    if !selection_state.show_creation {
        return;
    }
    SidePanel::right("Create Scenario").default_width(300.0).resizable(true).show(&egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.heading("Create new scenario");
                ui.separator();
                if ui.button("Cancel").clicked() {
                    selection_state.show_creation = false;
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("File name:");
                ui.text_edit_singleline(&mut selection_state.file_name);
            });
            ui.horizontal(|ui| {
                ui.label("Title:");
                ui.text_edit_singleline(&mut selection_state.title);
            });
            ui.horizontal(|ui| {
                ui.label("Description:");
                ui.text_edit_singleline(&mut selection_state.description);
            });
            ui.horizontal(|ui| {
                ui.label("Image path:");
                ui.text_edit_singleline(&mut selection_state.image_path);
                if ui.button("Select").on_hover_text("Select image").clicked() {
                    match tinyfiledialogs::open_file_dialog("Open image", "image.png", Some((&["*.png"], "PNG files"))) {
                        Some(file) => {
                            selection_state.image_path = file;
                        },
                        None => {
                            toasts.0.add(error_toast("No file selected"));
                        },
                    }
                };
            });
            ui.horizontal(|ui| {
                if ui.button("Create").on_hover_text("Create scenario").clicked() {
                    if let Err(e) = validate_input(selection_state.clone()) {
                        toasts.0.add(error_toast(&e));
                    } else {
                        let initial_data = SimulationData {
                            bodies: Vec::new(),
                            starting_time_millis: 0,
                            title: selection_state.title.clone(),
                            description: selection_state.description.clone(),
                            scale: SimulationScale::default().0,
                            timestep: Speed::default().0 as i32,
                            data_sets: Vec::new()
                        };
                        create_scenario(selection_state.file_name.clone(), selection_state.image_path.clone(), initial_data);
                        selection_state.show_creation = false;
                    }
                };
            });
        });
}

fn validate_input(selection_state: SelectionState) -> Result<(), String> {
    if let Ok(e) = fs::exists(&selection_state.image_path) {
        if !e {
            return Err("Image path cannot be accessed".to_string());
        }
    } else {
        return Err("Image path cannot be accessed".to_string());
    }
    if let Ok(e) = fs::exists(format!("scenarios/{}.sim", selection_state.file_name)) {
        if e {
            return Err("Scenario with file name already exists".to_string());
        }
    } else {
        return Err("File path cannot be accessed".to_string());
    }
    if selection_state.file_name.is_empty() {
        return Err("File name cannot be empty".to_string());
    }
    if selection_state.title.is_empty() {
        return Err("Title cannot be empty".to_string());
    }
    Ok(())
}

fn show_menu(
    mut egui_context: EguiContexts,
    scenario_folder: Res<ScenarioFolder>,
    folders: Res<Assets<LoadedFolder>>,
    bodies_asset: ResMut<Assets<SimulationData>>,
    assets: Res<AssetServer>,
    mut selected_scenario: ResMut<SelectedScenario>,
    mut sim_state: ResMut<NextState<SimState>>,
    mut images: Local<HashMap<String, TextureId>>,
    mut sim_state_type: ResMut<SimStateType>,
    mut selection_state: ResMut<SelectionState>,
    mut scale: ResMut<SimulationScale>,
    mut speed: ResMut<Speed>,
) {
    CentralPanel::default()
        .show(&egui_context.ctx_mut().clone(), |ui| {
            ui.horizontal(|ui| {
                ui.heading("Scenario Selection");
                ui.separator();
                if ui.button("Create new").clicked() {
                    selection_state.show_creation = true;
                }
            });
            ui.separator();
            ui.label("Select a scenario to load:");
            ui.separator();
            if let Some(loaded_folder) = folders.get(&scenario_folder.0) {
                for handle in loaded_folder.handles.clone() {
                    let path = handle.path().unwrap().path();
                    let file_name = path.file_name().unwrap().to_str().unwrap().clone();
                    if !file_name.ends_with(".sim") {
                        continue;
                    }
                    let image_handle: TextureId = if images.get(file_name).is_some() {
                        images.get(file_name).unwrap().clone()
                    } else {
                        let handle: Handle<Image> = assets.load(from_scenario_source(file_name.replace("sim", "png").as_str()));
                        let t_id = egui_context.add_image(handle);
                        images.insert(file_name.to_string(), t_id);
                        t_id
                    };
                    let typed_handle: Handle<SimulationData> = handle.clone().typed();
                    if let Some(scenario) = bodies_asset.get(&typed_handle) {
                        let title = &scenario.title;
                        let description = &scenario.description;
                        ui.horizontal(|ui| {
                            ui.image(egui::load::SizedTexture::new(image_handle, [100.0, 100.0]));
                            ui.vertical(|ui| {
                                ui.heading(title);
                                ui.label(description);
                                ui.with_layout(Layout::left_to_right(Align::BOTTOM), |ui| {
                                    let loading_button = ui.button("Load").on_hover_text("Load scenario");
                                    let edit_button = ui.button("Edit").on_hover_text("Edit scenario in editor");
                                    if let Some(delete_confirm) = &selection_state.delete_confirm {
                                        if delete_confirm == file_name.clone() {
                                            ui.label("Are you sure you want to delete this scenario?");
                                            ui.horizontal(|ui| {
                                                if ui.button("Yes").on_hover_text("Delete scenario").clicked() {
                                                    delete_scenario(file_name);
                                                    selection_state.delete_confirm = None;
                                                }
                                                if ui.button("No").on_hover_text("Cancel").clicked() {
                                                    selection_state.delete_confirm = None;
                                                }
                                            });
                                        }
                                    } else {
                                        let del_button = ui.button("Delete").on_hover_text("Delete scenario");
                                        if del_button.clicked() {
                                            selection_state.delete_confirm = Some(file_name.to_string());
                                        }
                                    }
                                    if loading_button.clicked() {
                                        select_scenario(&mut selected_scenario, &mut sim_state, &mut sim_state_type, &mut scale, &mut speed, scenario, typed_handle, SimStateType::Simulation);
                                    } else if edit_button.clicked() {
                                        select_scenario(&mut selected_scenario, &mut sim_state, &mut sim_state_type, &mut scale, &mut speed, scenario, typed_handle, SimStateType::Editor);
                                    }
                                });
                            });
                        });
                        ui.separator();
                    }
                }
            }
        });
}

fn select_scenario(
    selected_scenario: &mut ResMut<SelectedScenario>,
    sim_state: &mut ResMut<NextState<SimState>>,
    sim_state_type: &mut ResMut<SimStateType>,
    scale: &mut ResMut<SimulationScale>,
    speed: &mut ResMut<Speed>,
    data: &SimulationData,
    handle: Handle<SimulationData>,
    sim_type: SimStateType
) {
    scale.0 = data.scale;
    speed.0 = data.timestep as f64;
    selected_scenario.handle = handle;
    sim_state.set(SimState::Loading);
    **sim_state_type = sim_type;
}

fn delete_scenario(file_name: &str) {
    fs::remove_file(format!("scenarios/{}", file_name)).unwrap();
    fs::remove_file(format!("scenarios/{}", file_name.replace("sim", "png")).replace("sim", "png")).unwrap();
}

fn create_scenario(
    file_name: String,
    image_path: String,
    initial_data: SimulationData
) {
    fs::write(format!("scenarios/{}.sim", file_name), serde_json::to_string(&initial_data).unwrap()).unwrap();
    fs::copy(&image_path, format!("scenarios/{}.png", file_name)).unwrap();
}