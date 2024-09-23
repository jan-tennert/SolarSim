use bevy::prelude::{App, Camera, Commands, DespawnRecursiveExt, Entity, NextState, OnEnter, OnExit, Plugin, Query, ResMut, Vec3, With, Without};

use crate::{constants::{DEFAULT_SUB_STEPS, DEFAULT_TIMESTEP}};
use crate::simulation::components::body::Mass;
use crate::simulation::components::camera::{PanOrbitCamera, DEFAULT_CAM_RADIUS};
use crate::simulation::components::physics::{Pause, SubSteps};
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::components::speed::Speed;
use crate::simulation::loading::LoadingState;
use crate::simulation::SimState;
use crate::simulation::ui::{SimTime, StepType, UiState};
use crate::simulation::ui::scenario_selection::SelectedScenario;

pub struct ResetPlugin;

impl Plugin for ResetPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnExit(SimState::Loaded), clean_up)
        .add_systems(OnEnter(SimState::ExitToMainMenu), switch_to_menu)
        .add_systems(OnEnter(SimState::Reset), reset);
    }
    
}

fn clean_up(
    m_entities: Query<Entity, (With<Mass>, Without<Camera>)>,
    mut speed: ResMut<Speed>,
    mut pause: ResMut<Pause>,
    mut sim_time: ResMut<SimTime>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut scenario: ResMut<SelectedScenario>,
    mut sub_steps: ResMut<SubSteps>,
    mut loading_state: ResMut<LoadingState>,
    mut commands: Commands,
    mut camera: Query<&mut PanOrbitCamera>,
    mut ui_state: ResMut<UiState>
) {
    for entity in m_entities.iter() {
        commands.entity(entity).despawn_recursive()
    }
    speed.0 = DEFAULT_TIMESTEP;
    pause.0 = false;
    sim_time.0 = 0.0;
    selected_entity.entity = None;
    sub_steps.0 = DEFAULT_SUB_STEPS;
    scenario.spawned = false;
    loading_state.reset();
    let mut cam = camera.single_mut();
    cam.focus = Vec3::ZERO;
    cam.radius = DEFAULT_CAM_RADIUS;
    ui_state.visible = true;
    ui_state.step_type = StepType::SUBSTEPS;
    ui_state.show_debug = false;
}

fn switch_to_menu(
    mut state: ResMut<NextState<SimState>>
) {
    let _ = state.set(SimState::Menu);
}

fn reset(
    mut state: ResMut<NextState<SimState>>
) {
    let _ = state.set(SimState::Loading);
}