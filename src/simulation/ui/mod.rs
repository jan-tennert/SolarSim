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

use bevy::app::Update;
use bevy::prelude::{in_state, AabbGizmoConfigGroup, ButtonInput, DefaultGizmoConfigGroup, GizmoConfigStore, Srgba, Window};
use bevy::{
    core_pipeline::Skybox,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::{
        App, Camera, Color, Commands, DespawnRecursiveExt, Entity, GizmoConfig,
        IntoSystemConfigs, KeyCode, Mut, Name, NextState, Plugin, PointLight, Query, Res, ResMut, Resource, Transform, Vec3, Visibility, With, Without,
    },
    reflect::Reflect, time::Time, window::PresentMode,
};
use bevy_egui::{egui::{self, InnerResponse, Response, ScrollArea, Ui}, EguiContexts};
use bevy_inspector_egui::egui::{RichText, TextEdit};
use chrono::{Days, NaiveDateTime};

use crate::constants::G;
//use crate::fps::Fps;
use crate::constants::{DAY_IN_SECONDS, M_TO_AU};
//use crate::fps::Fps;
use crate::simulation::components::apsis::ApsisBody;
use crate::simulation::components::billboard::BillboardSettings;
use crate::simulation::components::body::BodyParent;
//use crate::fps::Fps;
use crate::simulation::components::body::{BodyChildren, BodyShape, Mass, Moon, OrbitSettings, Planet, RotationSpeed, SimPosition, Star, Velocity};
//use crate::fps::Fps;
use crate::simulation::components::camera::PanOrbitCamera;
//use crate::fps::Fps;
use crate::simulation::components::lock_on::LockOn;
//use crate::fps::Fps;
use crate::simulation::components::orbit_lines::OrbitOffset;
use crate::simulation::components::physics::Pause;
//use crate::fps::Fps;
use crate::simulation::components::physics::{apply_physics, SubSteps};
//use crate::fps::Fps;
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::components::speed::Speed;
//use crate::fps::Fps;
use crate::simulation::render::skybox::Cubemap;
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

#[derive(Resource, Reflect, Default)]
pub struct SimTime(pub f32);

#[derive(Resource, Reflect, Default)]
pub struct Light {
    pub shadows_enabled: bool,
}

#[derive(Reflect)]
pub enum StepType {
    SUBSTEPS,
    TIMESTEPS    
}

#[derive(Resource, Reflect)]
pub struct UiState {
    pub visible: bool,
    pub step_type: StepType,
    pub show_debug: bool,
    pub show_keys: bool,
    pub edit_mass: bool,
    pub edit_vel: bool,
    pub dyn_hide_orbit_lines: bool
}

impl Default for UiState {
    fn default() -> Self {
        UiState { visible: true, step_type: StepType::SUBSTEPS, show_debug: false, dyn_hide_orbit_lines: true, show_keys: false, edit_mass: false, edit_vel: false }
    }
}

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiState>()
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
                     (editor_body_panel.run_if(sim_state_type_editor), sim_body_panel.run_if(sim_state_type_simulation).after(apply_physics)),
                     (simulation_bottom_bar.run_if(sim_state_type_simulation), editor_bottom_bar.run_if(sim_state_type_editor))
                ).chain()
            );
    }
}