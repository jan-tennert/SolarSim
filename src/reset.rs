use bevy::prelude::{SystemSet, App, Plugin, OnExit, Entity, Name, With, ResMut, Commands, Query, NextState, Update, IntoSystemConfigs, in_state, OnEnter, Camera, Without, DespawnRecursiveExt};

use crate::{SimState, speed::Speed, ui::SimTime, body::Mass, physics::Pause, selection::SelectedEntity, constants::HOUR_IN_SECONDS};

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
    entities: Query<(Entity, With<Mass>, Without<Camera>)>,
    mut speed: ResMut<Speed>,
    mut pause: ResMut<Pause>,
    mut sim_time: ResMut<SimTime>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut commands: Commands
) {
    for (entity, _, _) in entities.iter() {
        commands.entity(entity).despawn_recursive()
    }
    speed.0 = HOUR_IN_SECONDS as f64;
    pause.0 = false;
    sim_time.0 = 0.0;
    selected_entity.0 = None;
}

fn switch_to_menu(
    mut state: ResMut<NextState<SimState>>
) {
    let _ = state.set(SimState::Menu);
}

fn reset(
    mut state: ResMut<NextState<SimState>>
) {
    let _ = state.set(SimState::Simulation);
}