mod bottom_bar;
pub mod system_panel;
pub mod editor_body_panel;
pub mod debug_window;
pub mod components;
pub mod scenario_selection;
mod sim_body_panel;
pub mod sim_bottom_bar;
pub mod editor_bottom_bar;
pub mod toast;
mod metadata;
pub mod menu;

//use crate::fps::Fps;
//use crate::fps::Fps;
//use crate::fps::Fps;
//use crate::fps::Fps;
//use crate::fps::Fps;
//use crate::fps::Fps;
//use crate::fps::Fps;
//use crate::fps::Fps;
use crate::simulation::integration::SimulationStep;
//use crate::fps::Fps;
use crate::simulation::ui::debug_window::DebugPlugin;
use crate::simulation::ui::editor_body_panel::{editor_body_panel, EditorPanelState};
use crate::simulation::ui::editor_bottom_bar::editor_bottom_bar;
use crate::simulation::ui::metadata::MetadataPlugin;
use crate::simulation::ui::scenario_selection::ScenarioSelectionPlugin;
use crate::simulation::ui::sim_body_panel::sim_body_panel;
use crate::simulation::ui::sim_bottom_bar::simulation_bottom_bar;
use crate::simulation::ui::system_panel::system_panel;
use crate::simulation::ui::toast::ToastPlugin;
use crate::simulation::SimState;
//use crate::fps::Fps;
use crate::utils::{sim_state_type_editor, sim_state_type_simulation};
use bevy::app::Update;
use bevy::prelude::in_state;
use bevy::utils::default;
use bevy::{
    prelude::{
        App,
        IntoSystemConfigs, Plugin, Resource,
    },
    reflect::Reflect,
};

#[derive(Resource, Reflect, Default)]
pub struct SimTime(pub f32);

#[derive(Resource, Reflect, Default)]
pub struct Light {
    pub shadows_enabled: bool,
}

#[derive(Reflect, Default)]
pub enum StepType {
    #[default]
    SUBSTEPS,
    TIMESTEPS    
}

#[derive(Resource, Reflect, Default)]
pub struct UiState {
    pub visible: bool,
    pub step_type: StepType,
    pub show_debug: bool,
    pub show_keys: bool,
    pub edit_mass: bool,
    pub vel_multiplier: f64,
    pub mass_value: f64,
    pub edit_vel: bool,
    pub dyn_hide_orbit_lines: bool,
}

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UiState {
                visible: true,
                dyn_hide_orbit_lines: true,
                ..default()
            })
            .init_resource::<EditorPanelState>()
            .register_type::<SimTime>()
            .init_resource::<SimTime>()
            .add_plugins(DebugPlugin)
            .add_plugins(ScenarioSelectionPlugin)
            .add_plugins(ToastPlugin)
            .add_plugins(MetadataPlugin)
            .add_systems(
                Update,
                (
                    system_panel.run_if(in_state(SimState::Loaded)),
                    (editor_body_panel.run_if(sim_state_type_editor), sim_body_panel.run_if(sim_state_type_simulation).after(SimulationStep)),
                    (simulation_bottom_bar.run_if(sim_state_type_simulation), editor_bottom_bar.run_if(sim_state_type_editor))
                ).chain()
            );
    }
}