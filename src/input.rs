use bevy::{prelude::{App, in_state, Input, IntoSystemConfigs, KeyCode, Plugin, Query, Res, ResMut, Update, Vec3}, window::{Window, WindowMode}};

use crate::{camera::PanOrbitCamera, SimState, ui::{UiState, StepType}, physics::{Pause, SubSteps}, speed::Speed};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, input_system.run_if(in_state(SimState::Simulation)));
    }
    
}

fn input_system(
    keys: Res<Input<KeyCode>>,
    mut ui_state: ResMut<UiState>,
    mut camera: Query<&mut PanOrbitCamera>,
    mut windows: Query<&mut Window>,
    mut pause: ResMut<Pause>,
    mut speed: ResMut<Speed>,
    mut sub_steps: ResMut<SubSteps>
) {
    let timestep_selected = match ui_state.step_type {
        StepType::SUBSTEPS => false,
        StepType::TIMESTEPS => true
    };
    if keys.just_pressed(KeyCode::F10) {
        ui_state.visible = !ui_state.visible
    } else if keys.just_pressed(KeyCode::C) {
        camera.single_mut().focus = Vec3::ZERO;
    } else if keys.just_pressed(KeyCode::F11) {
        let mut window = windows.single_mut();
        let current = window.mode;
        if current == WindowMode::Windowed {
            window.mode = WindowMode::BorderlessFullscreen;
        } else {
            window.mode = WindowMode::Windowed;
        }
    } else if keys.just_pressed(KeyCode::Space) {
        pause.0 = !pause.0;
    } else if keys.just_pressed(KeyCode::Left) {
        if timestep_selected {
            speed.small_step_down();
        } else {
            sub_steps.small_step_down();                                   
        }
    } else if keys.just_pressed(KeyCode::Right) {
        if timestep_selected {
            speed.small_step_up();
        } else {
            sub_steps.small_step_up();                                   
        }
    } else if keys.just_pressed(KeyCode::AltLeft) {
        if timestep_selected {
            ui_state.step_type = StepType::SUBSTEPS;
        } else {
            ui_state.step_type = StepType::TIMESTEPS;                                  
        }
    }
}
