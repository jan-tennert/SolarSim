use crate::constants::DAY_IN_SECONDS;
use crate::setup::StartingTime;
use crate::simulation::{SimState, SimStateType};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{NextState, Query, Res, ResMut, Time, Window};
use bevy::window::PresentMode;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Align, Direction};
use bevy_inspector_egui::egui::TextEdit;
use chrono::{Days, NaiveDateTime};
use crate::simulation::components::lock_on::LockOn;
use crate::simulation::components::physics::{Pause, SubSteps};
use crate::simulation::components::speed::Speed;
use crate::simulation::ui::{SimTime, StepType, UiState};

pub fn bottom_bar(
    time: Res<Time>,
    mut sim_time: ResMut<SimTime>,
    mut egui_context: EguiContexts,
    mut speed: ResMut<Speed>,
    mut windows: Query<&mut Window>,
    mut lock_on_parent: ResMut<LockOn>,
    mut pause: ResMut<Pause>,
    mut state: ResMut<NextState<SimState>>,
    starting_time: Res<StartingTime>,
    mut sub_steps: ResMut<SubSteps>,
    mut ui_state: ResMut<UiState>,
    diagnostics: Res<DiagnosticsStore>,
    sim_type: Res<SimStateType>
) {
    if !ui_state.visible || windows.is_empty() || egui_context.try_ctx_mut().is_none() {
        return;
    }
    let mut window = windows.single_mut();
    if !pause.0 && *sim_type == SimStateType::Simulation {
        sim_time.0 += time.delta_seconds() * (((speed.0 * (sub_steps.0 as f64)) / (DAY_IN_SECONDS as f64)) as f32);
    }
    let date = NaiveDateTime::from_timestamp_millis(starting_time.0)
        .unwrap()
        .checked_add_days(Days::new((((sim_time.0 * 100.0).round()) / 100.0) as u64))
        .unwrap();
    match *sim_type {
        SimStateType::None => {}
        SimStateType::Simulation => {
            simulation_bottom_bar(
                date,
                &mut window,
                &mut egui_context,
                &mut speed,
                &mut lock_on_parent,
                &mut pause,
                &mut state,
                &mut sub_steps,
                &mut ui_state,
                diagnostics,
            )
        }
        SimStateType::Editor => {
            editor_bottom_bar(
                date,
                &mut window,
                &mut egui_context,
                &mut lock_on_parent,
                diagnostics,
                &mut state,
            )
        }
    }
}

fn editor_bottom_bar(
    date: NaiveDateTime,
    window: &mut Window,
    egui_context: &mut EguiContexts,
    lock_on_parent: &mut ResMut<LockOn>,
    diagnostics: Res<DiagnosticsStore>,
    state: &mut ResMut<NextState<SimState>>,
) {
    egui::TopBottomPanel::bottom("time_panel")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.label(format!(
                            "Date: {}",
                            date.format("%d.%m.%Y"),
                        ));
                    });
                });

                ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                    if ui.button("Reset").on_hover_text("Reset scenario from file").clicked() {
                        let _ = state.set(SimState::Reset);
                    }
                    ui.button("Save").on_hover_text("Save scenario to file").clicked();
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    ui.checkbox(&mut lock_on_parent.enabled, "Lock on Parent");
                    let mut vsync = window.present_mode == PresentMode::AutoVsync;
                    let old_option = vsync;
                    ui.checkbox(&mut vsync, "VSync");
                    if old_option != vsync {
                        if vsync {
                            window.present_mode = PresentMode::AutoVsync;
                        } else {
                            window.present_mode = PresentMode::AutoNoVsync;
                        }
                    }
                    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                        if let Some(value) = fps.smoothed() {
                            // Update the value of the second section
                            ui.label(format!("{:.0} FPS", value));
                        }
                    }
                })
            });
        });
}

fn simulation_bottom_bar(
    date: NaiveDateTime,
    window: &mut Window,
    egui_context: &mut EguiContexts,
    speed: &mut ResMut<Speed>,
    lock_on_parent: &mut ResMut<LockOn>,
    pause: &mut ResMut<Pause>,
    state: &mut ResMut<NextState<SimState>>,
    sub_steps: &mut ResMut<SubSteps>,
    ui_state: &mut ResMut<UiState>,
    diagnostics: Res<DiagnosticsStore>,
) {
    egui::TopBottomPanel::bottom("time_panel")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.horizontal_centered(|ui| {
                        let mut timestep_selected = match ui_state.step_type {
                            StepType::SUBSTEPS => false,
                            StepType::TIMESTEPS => true
                        };
                        if ui.small_button("<<").clicked() {
                            if timestep_selected {
                                speed.big_step_down();
                            } else {
                                sub_steps.big_step_down();
                            }
                        }
                        if ui.small_button("<").clicked() {
                            if timestep_selected {
                                speed.small_step_down();
                            } else {
                                sub_steps.small_step_down();
                            }
                        }
                        ui.label(format!(
                            "{} ({}/s)",
                            date.format("%d.%m.%Y"),
                            speed.format(sub_steps.0)
                        ));
                        let time_text = if !pause.0 { "Pause" } else { "Resume" };
                        if ui.button(time_text).clicked() {
                            pause.0 = !pause.0;
                        }
                        if ui.small_button(">").clicked() {
                            if timestep_selected {
                                speed.small_step_up();
                            } else {
                                sub_steps.small_step_up();
                            }
                        }
                        if ui.small_button(">>").clicked() {
                            if timestep_selected {
                                speed.big_step_up();
                            } else {
                                sub_steps.big_step_up();
                            }
                        }
                        //       ui.add_space(20.0);

                        if ui.toggle_value(&mut !timestep_selected, "Substeps per frame").clicked() {
                            timestep_selected = false;
                        }
                        let mut new_sub_steps = sub_steps.0.to_string();
                        if ui
                            .add(TextEdit::singleline(&mut new_sub_steps).desired_width(50.0))
                            .changed()
                        {
                            if let Ok(new_sub_steps_num) = new_sub_steps.parse::<i32>() {
                                sub_steps.0 = new_sub_steps_num;
                            }
                        }
                        //     ui.add_space(20.0);
                        if ui.toggle_value(&mut timestep_selected, "Timestep in seconds").clicked() {
                            timestep_selected = true;
                        }
                        let mut new_speed = speed.0.to_string();
                        if ui
                            .add(TextEdit::singleline(&mut new_speed).desired_width(50.0))
                            .changed()
                        {
                            if let Ok(new_speed_num) = new_speed.parse::<f64>() {
                                speed.0 = new_speed_num;
                            }
                        }
                        ui.label(format!("({})", speed.format(1)));

                        if timestep_selected {
                            ui_state.step_type = StepType::TIMESTEPS
                        } else {
                            ui_state.step_type = StepType::SUBSTEPS
                        }
                    });
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    if ui.button("Reset").clicked() {
                        let _ = state.set(SimState::Reset);
                    }
                    ui.checkbox(&mut lock_on_parent.enabled, "Lock on Parent");
                    let mut vsync = window.present_mode == PresentMode::AutoVsync;
                    let old_option = vsync;
                    ui.checkbox(&mut vsync, "VSync");
                    if old_option != vsync {
                        if vsync {
                            window.present_mode = PresentMode::AutoVsync;
                        } else {
                            window.present_mode = PresentMode::AutoNoVsync;
                        }
                    }
                    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                        if let Some(value) = fps.smoothed() {
                            // Update the value of the second section
                            ui.label(format!("{:.0} FPS", value));
                        }
                    }
                })
            });
        });
}