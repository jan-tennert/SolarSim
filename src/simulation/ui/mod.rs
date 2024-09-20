mod bottom_bar;
pub mod system_panel;
mod body_panel;
pub mod debug_window;

use bevy::{
    core_pipeline::Skybox,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::{
        App, Camera, Color, Commands, DespawnRecursiveExt, Entity, GizmoConfig,
        IntoSystemConfigs, KeyCode, Mut, Name, NextState, Plugin, PointLight, Query, Res, ResMut, Resource, Transform, Vec3, Visibility, With, Without,
    },
    reflect::Reflect, time::Time, window::PresentMode,
};
use bevy::app::Update;
use bevy::prelude::{in_state, AabbGizmoConfigGroup, ButtonInput, DefaultGizmoConfigGroup, GizmoConfigStore, Srgba, Window};
use bevy_egui::{egui::{self, InnerResponse, Response, ScrollArea, Ui}, EguiContexts};
use bevy_inspector_egui::egui::{RichText, TextEdit};
use chrono::{Days, NaiveDateTime};

//use crate::fps::Fps;
use crate::{constants::{DAY_IN_SECONDS, M_TO_AU, M_TO_UNIT}, setup::StartingTime, unit::format_length};
use crate::simulation::components::billboard::BillboardSettings;
use crate::simulation::components::body::BodyParent;
use crate::constants::G;
use crate::simulation::components::physics::Pause;
use crate::SimState;
//use crate::fps::Fps;
use crate::simulation::components::apsis::ApsisBody;
//use crate::fps::Fps;
use crate::simulation::components::body::{BodyChildren, Diameter, Mass, Moon, OrbitSettings, Planet, RotationSpeed, Scale, SimPosition, Star, Velocity};
//use crate::fps::Fps;
use crate::simulation::components::camera::PanOrbitCamera;
//use crate::fps::Fps;
use crate::simulation::components::lock_on::LockOn;
//use crate::fps::Fps;
use crate::simulation::components::orbit_lines::OrbitOffset;
//use crate::fps::Fps;
use crate::simulation::components::physics::{apply_physics, SubSteps};
//use crate::fps::Fps;
use crate::simulation::components::selection::SelectedEntity;
//use crate::fps::Fps;
use crate::simulation::render::skybox::Cubemap;
use crate::simulation::ui::body_panel::body_panel;
use crate::simulation::ui::bottom_bar::bottom_bar;
use crate::simulation::ui::system_panel::system_panel;
use crate::simulation::components::speed::Speed;
use crate::unit::format_seconds;

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
            .register_type::<SimTime>()
            .init_resource::<SimTime>()
            .add_systems(
                Update,
                (system_panel.after(bottom_bar), body_panel.after(system_panel), bottom_bar.after(apply_physics)).run_if(in_state(SimState::Simulation)),
            );
    }
}