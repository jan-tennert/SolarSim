use bevy::{prelude::{App, in_state, IntoSystemConfigs, KeyCode, Plugin, Query, Res, ResMut, Update, Vec3}, window::{Window, WindowMode}};
use bevy::prelude::ButtonInput;
use bevy_egui::{egui::{self}, EguiContexts, EguiSettings};

use crate::{camera::PanOrbitCamera, SimState, ui::{UiState, StepType}, physics::{Pause, SubSteps}, speed::Speed};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, global_input_system)
        .add_systems(Update, key_window.run_if(in_state(SimState::Simulation)))
        .add_systems(Update, sim_input_system.run_if(in_state(SimState::Simulation)));
    }
    
}

fn key_window(
    mut egui_ctx: EguiContexts,
    mut ui_state: ResMut<UiState>,
) {
    if !ui_state.visible {
        return;
    }
    egui::Window::new("Keybind Information")
        .open(&mut ui_state.show_keys)
        .collapsible(true)
        .constrain(true)
        .scroll2([true, true])
        .default_width(250.0)
        .show(egui_ctx.ctx_mut(), |ui| {
                ui.label("F11 - Toggle Fullscreen");
                ui.label("F10 - Hide Ui");
                ui.label("Space - Pause");
                ui.label("Left Arrow - 2x Speed");
                ui.label("Right Arrow - 1/2 Speed");
                ui.label("Left Alt - Change Step Type");
                ui.label("C - Reset Camera");
                ui.label("Left Mouse - Rotate Camera");
                ui.label("Right Mouse - Move Camera");
                ui.label("Ctrl + , - Increase Ui Scale");
                ui.label("Ctrl + . - Decrease Ui Scale");
        });
}

fn global_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    if keys.just_pressed(KeyCode::F11) {
        let mut window = windows.single_mut();
        let current = window.mode;
        if current == WindowMode::Windowed {
            window.mode = WindowMode::BorderlessFullscreen;
        } else {
            window.mode = WindowMode::Windowed;
        }
    }   
}

fn sim_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<UiState>,
    mut camera: Query<&mut PanOrbitCamera>,
    mut pause: ResMut<Pause>,
    mut speed: ResMut<Speed>,
    mut sub_steps: ResMut<SubSteps>,
    mut egui_settings: ResMut<EguiSettings>,
) {
    let timestep_selected = match ui_state.step_type {
        StepType::SUBSTEPS => false,
        StepType::TIMESTEPS => true
    };
    if keys.just_pressed(KeyCode::F10) {
        ui_state.visible = !ui_state.visible
    } else if keys.just_pressed(KeyCode::KeyC) {
        camera.single_mut().focus = Vec3::ZERO;
    } else if keys.just_pressed(KeyCode::Space) {
        pause.0 = !pause.0;
    } else if keys.just_pressed(KeyCode::ArrowLeft) {
        if timestep_selected {
            speed.small_step_down();
        } else {
            sub_steps.small_step_down();                                   
        }
    } else if keys.just_pressed(KeyCode::ArrowRight) {
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
    } else if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::Comma) {
            egui_settings.scale_factor *= 1.1;
    } else if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::Period) {
            egui_settings.scale_factor *= 0.9;
    }
}
