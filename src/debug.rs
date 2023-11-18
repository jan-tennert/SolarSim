use bevy::app::{App, Plugin};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{Resource, Update, IntoSystemConfigs, in_state, Res, ResMut};
use bevy_egui::egui::RichText;
use bevy_egui::{egui::{self, InnerResponse, Response, Ui}, EguiContexts};

use crate::SimState;
use crate::physics::NBodyTime;
use crate::ui::{system_ui, UiState};


pub struct DebugPlugin;

impl Plugin for DebugPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (debug_window.after(system_ui)).run_if(in_state(SimState::Simulation)));
    }

}

fn debug_window(
    mut egui_ctx: EguiContexts,
    mut ui_state: ResMut<UiState>,
    nbody_time: Res<NBodyTime>,
    diagnostics: Res<DiagnosticsStore>,
) {
    egui::Window::new("Debug Information")
        .vscroll(true)
        .open(&mut ui_state.show_debug)
        .show(egui_ctx.ctx_mut(), |ui| {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.value() {
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
            if let Some(frametime) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
                if let Some(value) = frametime.value() {
                    // Update the value of the second section
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Frametime: ").strong());                            
                        ui.label(format!("{:.0}", value));
                    });
                }
            }
            if let Some(frametime) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_COUNT) {
                if let Some(value) = frametime.value() {
                    // Update the value of the second section
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Framecount: ").strong());                            
                        ui.label(format!("{:.0}", value));
                    });
                }
            }
            ui.horizontal(|ui| {
                ui.label(RichText::new("N-Body calculation time: ").strong());                            
                ui.label(format!("{:?}", nbody_time.0));
            });
        });
}