use bevy::prelude::{App, Camera, Commands, DespawnRecursiveExt, Entity, NextState, OnEnter, OnExit, Plugin, Query, ResMut, Vec3, With, Without};

use crate::{body::Mass, camera::{DEFAULT_CAM_RADIUS, PanOrbitCamera}, constants::{DEFAULT_SUB_STEPS, DEFAULT_TIMESTEP}, loading::LoadingState, physics::{Pause, SubSteps}, selection::SelectedEntity, setup::BodiesHandle, SimState, speed::Speed, ui::{SimTime, UiState, StepType}};

pub struct ResetPlugin;

impl Plugin for ResetPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnExit(SimState::Simulation), clean_up)
        .add_systems(OnEnter(SimState::ExitToMainMenu), switch_to_menu)
        .add_systems(OnEnter(SimState::Reset), reset);
    }
    
}

fn clean_up(
    m_entities: Query<(Entity, With<Mass>, Without<Camera>)>,
    mut speed: ResMut<Speed>,
    mut pause: ResMut<Pause>,
    mut sim_time: ResMut<SimTime>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut bodies: ResMut<BodiesHandle>,
    mut sub_steps: ResMut<SubSteps>,
    mut loading_state: ResMut<LoadingState>,
    mut commands: Commands,
    mut camera: Query<&mut PanOrbitCamera>,
    mut ui_state: ResMut<UiState>
) {
    for (entity, _, _) in m_entities.iter() {
        commands.entity(entity).despawn_recursive()
    }
    speed.0 = DEFAULT_TIMESTEP;
    pause.0 = false;
    sim_time.0 = 0.0;
    selected_entity.entity = None;
    sub_steps.0 = DEFAULT_SUB_STEPS;
    bodies.spawned = false;
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