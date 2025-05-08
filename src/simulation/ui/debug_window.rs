use std::time::Duration;

use crate::simulation::components::body::Mass;
use crate::simulation::integration::{NBODY_STEPS, NBODY_STEP_TIME, NBODY_TOTAL_TIME};
use crate::simulation::ui::system_panel::system_panel;
use crate::simulation::ui::UiState;
use crate::simulation::SimState;
use bevy::app::{App, Plugin};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{in_state, IntoScheduleConfigs, Query, Res, ResMut};
use bevy_egui::egui::RichText;
use bevy_egui::{egui::{self}, EguiContextPass, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(EguiContextPass, (debug_window.after(system_panel)).run_if(in_state(SimState::Loaded)));
    }

}

fn debug_window(
    mut egui_ctx: EguiContexts,
    mut ui_state: ResMut<UiState>,
    diagnostics: Res<DiagnosticsStore>,
    bodies: Query<&Mass>,
    camera: Query<&PanOrbitCamera>
) {
    if !ui_state.visible || egui_ctx.try_ctx_mut().is_none() {
        return;
    }
    let cam = camera.single().unwrap();
    egui::Window::new("Debug Information")
        .open(&mut ui_state.show_debug)
        .collapsible(true)
        .constrain(true)
        .scroll([true, true])
        .default_width(250.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    // Update the value of the second section
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("FPS: ").strong());                            
                        ui.label(format!("{:.0}", value));
                    });
                }
                if let Some(value) = fps.average() {
                    // Update the value of the second section
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Avg. FPS: ").strong());                            
                        ui.label(format!("{:.0}", value));
                    });
                }
            }
            if let Some(frametime) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
                if let Some(value) = frametime.smoothed() {
                    // Update the value of the second section
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Frametime: ").strong());                            
                        ui.label(format!("{:.0}", value));
                    });
                }
            }
            if let Some(frametime) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT) {
                if let Some(value) = frametime.value() {
                    // Update the value of the second section
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Framecount: ").strong());                            
                        ui.label(format!("{:.0}", value));
                    });
                }
            }
            let body_count = bodies.iter().count();
            ui.horizontal(|ui| {
                ui.label(RichText::new("Total amount of bodies: ").strong());                            
                ui.label(format!("{:?}", body_count));
            });
            if let Some(frametime) = diagnostics.get(&NBODY_STEPS) {
                if let Some(value) = frametime.smoothed() {
                    // Update the value of the second section
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("N-Body steps / s: ").strong());                            
                        ui.label(format!("{:.0}", value));
                    });
                }
            }
            if let Some(frametime) = diagnostics.get(&NBODY_STEP_TIME) {
                if let Some(value) = frametime.average() {
                    // Update the value of the second section
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("N-Body step calculation time: ").strong());                            
                        ui.label(format!("{:?}", Duration::from_nanos(value as u64)));
                    });
                }
            }
            if let Some(frametime) = diagnostics.get(&NBODY_TOTAL_TIME) {
                if let Some(value) = frametime.average() {
                    // Update the value of the second section
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("N-Body total calculation time: ").strong());                            
                        ui.label(format!("{:?}", Duration::from_nanos(value as u64)));
                    });
                }
            }
            ui.horizontal(|ui| {
                ui.label(RichText::new("Camera focus: ").strong());                            
                ui.label(format!("{}", cam.focus));
            });
            ui.horizontal(|ui| {
                ui.label(RichText::new("Camera radius: ").strong());                            
                ui.label(format!("{}", cam.radius.unwrap_or(0.)));
            });
            ui.allocate_space(egui::vec2(ui.available_size().x, 0.0));
        });
}