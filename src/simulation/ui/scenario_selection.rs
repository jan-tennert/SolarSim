use bevy::app::{App, Plugin, Update};
use bevy::asset::LoadedFolder;
use bevy::prelude::{in_state, AssetServer, Assets, Commands, Handle, Image, IntoSystemConfigs, Local, NextState, OnEnter, OnExit, Res, ResMut, Resource};
use bevy::utils::HashMap;
use bevy_egui::egui::{Align, CentralPanel, Layout, TextureId};
use bevy_egui::{egui, EguiContexts};
use image::load;
use crate::serialization::SimulationData;
use crate::simulation::{SimState, SimStateType};

pub struct ScenarioSelectionPlugin;

impl Plugin for ScenarioSelectionPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedScenario>()
            .add_systems(OnEnter(SimState::ScenarioSelection), load_scenarios)
            .add_systems(Update, spawn_menu.run_if(in_state(SimState::ScenarioSelection)));
    }
}

#[derive(Resource)]
pub struct ScenarioFolder(pub Handle<LoadedFolder>);

#[derive(Resource, Default)]
pub struct SelectedScenario {

    pub handle: Handle<SimulationData>,
    pub spawned: bool

}

fn load_scenarios(
    assets: Res<AssetServer>,
    mut commands: Commands
) {
    let handle = assets.load_folder("scenarios");
    commands.insert_resource(ScenarioFolder(handle));
}

fn spawn_menu(
    mut egui_context: EguiContexts,
    scenario_folder: Res<ScenarioFolder>,
    folders: Res<Assets<LoadedFolder>>,
    bodies_asset: ResMut<Assets<SimulationData>>,
    assets: Res<AssetServer>,
    mut selected_scenario: ResMut<SelectedScenario>,
    mut sim_state: ResMut<NextState<SimState>>,
    mut images: Local<HashMap<String, TextureId>>,
    mut sim_state_type: ResMut<SimStateType>
) {
    CentralPanel::default()
        .show(&egui_context.ctx_mut().clone(), |ui| {
            ui.heading("Scenario Selection");
            ui.separator();
            ui.label("Select a scenario to load:");
            ui.separator();
            if let Some(loaded_folder) = folders.get(&scenario_folder.0) {
                for handle in loaded_folder.handles.clone() {
                    let path = handle.path().unwrap().path();
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    if !file_name.ends_with(".sim") {
                        continue;
                    }
                    let image_handle: TextureId = if images.get(file_name).is_some() {
                        images.get(file_name).unwrap().clone()
                    } else {
                        let handle: Handle<Image> = assets.load(format!("scenarios/{}", file_name.replace("sim", "png")));
                        let t_id = egui_context.add_image(handle);
                        images.insert(file_name.to_string(), t_id);
                        t_id
                    };
                    let typed_handle: Handle<SimulationData> = handle.typed();
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
                                    if loading_button.clicked() {
                                        selected_scenario.handle = typed_handle;
                                        sim_state.set(SimState::Loading);
                                        *sim_state_type = SimStateType::Simulation;
                                    } else if edit_button.clicked() {
                                        selected_scenario.handle = typed_handle;
                                        sim_state.set(SimState::Loading);
                                        *sim_state_type = SimStateType::Editor;
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
