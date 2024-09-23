use bevy::app::{App, Plugin, Update};
use bevy::asset::LoadedFolder;
use bevy::prelude::{in_state, AssetServer, Assets, Commands, Handle, IntoSystemConfigs, OnEnter, OnExit, Res, ResMut, Resource};
use bevy_egui::egui::CentralPanel;
use bevy_egui::{egui, EguiContexts};
use image::load;
use crate::serialization::SimulationData;
use crate::simulation::SimState;

pub struct ScenarioSelectionPlugin;

impl Plugin for ScenarioSelectionPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SimState::ScenarioSelection), load_scenarios)
            .add_systems(Update, spawn_menu.run_if(in_state(SimState::ScenarioSelection)));
    }
}

#[derive(Resource)]
pub struct ScenarioFolder(pub Handle<LoadedFolder>);

fn load_scenarios(
    assets: Res<AssetServer>,
    mut commands: Commands
) {
    let handle = assets.load_folder("scenarios");
    commands.insert_resource(ScenarioFolder(handle));
}

fn spawn_menu(
    mut commands: Commands,
    mut egui_context: EguiContexts,
    scenario_folder: Res<ScenarioFolder>,
    folders: Res<Assets<LoadedFolder>>,
    bodies_asset: ResMut<Assets<SimulationData>>,
) {
    CentralPanel::default()
        .show(&egui_context.ctx_mut(), |ui| {
            ui.heading("Scenario Selection");
            ui.separator();
            ui.label("Select a scenario to load:");
            ui.separator();
            if let Some(loaded_folder) = folders.get(&scenario_folder.0) {
                for handle in loaded_folder.handles.clone() {
                    let path = handle.path().unwrap().path();
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    if let Some(scenario) = bodies_asset.get(&handle.typed()) {
                        let title = &scenario.title;
                        let description = &scenario.description;
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.heading(title);
                                ui.label(description);
                                if ui.button("Load").clicked() {
                                    // Load scenario
                                }
                            });
                        });
                        ui.separator();
                    }
                }
            }
        });
}
