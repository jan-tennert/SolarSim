use bevy::{prelude::{Res, SystemSet, App, Plugin, OnExit, Entity, Name, With, ResMut, Commands, Query, NextState, Update, IntoSystemConfigs, in_state, OnEnter, Camera, Without, DespawnRecursiveExt, Input, KeyCode, Vec3}, window::{Window, WindowMode}};

use crate::{ui::UiState, SimState, camera::PanOrbitCamera};

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
    mut windows: Query<&mut Window>
) {
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
    }
}
