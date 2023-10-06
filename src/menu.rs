use bevy::{prelude::*};
use bevy_egui::*;
use bevy_inspector_egui::egui::Frame;

use crate::SimState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (main_menu).run_if(in_state(SimState::Menu)));
    }
}

fn main_menu(
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<SimState>>
) {
    egui::CentralPanel::default().frame(Frame::none()).show(contexts.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::from_main_dir_and_cross_align(egui::Direction::BottomUp, egui::Align::Center), |ui| {
            if ui.button("Start Simulation").clicked() {
                let _ = state.set(SimState::Simulation);
            }
        })
    });
}