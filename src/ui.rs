use std::collections::HashMap;

use bevy::{
    prelude::{
        App, Camera, Commands, DespawnRecursiveExt, Entity, Input, KeyCode,
        Mut, Name, Plugin, PointLight, Query, Res, ResMut, Resource, Transform, Vec3, Visibility, With, Without, NextState,
    },
    reflect::Reflect,
    time::Time,
    window::PresentMode,
};
use bevy::app::Update;
use bevy::prelude::{in_state, IntoSystemConfigs, Window};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::egui::{RichText, TextEdit};
use chrono::{Days, NaiveDate};
//use crate::fps::Fps;
use crate::{input::BlockInputPlugin, body::{Mass, Selectable, Velocity, Parent}, constants::DAY_IN_SECONDS};
use crate::physics::{Pause, update_position};
use crate::SimState;
use crate::speed::Speed;

#[derive(Resource, Reflect, Default)]
pub struct SimTime(pub f32);

#[derive(Resource, Reflect, Default)]
pub struct Light {
    pub shadows_enabled: bool,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
         //   .add_plugins(EguiPlugin)
            .register_type::<SimTime>()
            .init_resource::<SimTime>()
            .add_plugins(BlockInputPlugin)
            .add_systems(
                Update,
                (system_ui.after(time_ui), body_ui.after(update_position), time_ui.after(body_ui)).run_if(in_state(SimState::Simulation))
            );
    }
}

pub fn time_ui(
    time: Res<Time>,
    mut sim_time: ResMut<SimTime>,
    mut egui_context: EguiContexts,
    mut speed: ResMut<Speed>,
   // fps: Res<Fps>,
    mut windows: Query<&mut Window>,
   // mut lock_on_sun: ResMut<LockSun>,
    mut pause: ResMut<Pause>,
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<SimState>>,
) {
    let mut window = windows.single_mut();
    if !pause.0 {
        sim_time.0 += time.delta_seconds() * (speed.0 / DAY_IN_SECONDS);
    }
    let date = NaiveDate::from_ymd_opt(2023, 10, 1)
        .unwrap()
        .checked_add_days(Days::new(sim_time.0.round() as u64))
        .unwrap();
    egui::TopBottomPanel::bottom("time_panel")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    if ui.small_button("<<").clicked() {
                        speed.0 /= 10.0;
                    }
                    if ui.small_button("<").clicked() || keys.just_pressed(KeyCode::Left) {
                        speed.0 /= 2.0;
                    }
                    ui.label(format!(
                        "{} ({})",
                        date.format("%d.%m.%Y"),
                        speed.format()
                    ));
                    let time_text = if pause.0 { "Pause" } else { "Resume" };
                    if ui.button(time_text).clicked() || keys.just_pressed(KeyCode::Space) {
                        pause.0 = !pause.0;
                    }
                    if ui.small_button(">").clicked() || keys.just_pressed(KeyCode::Right) {
                        speed.0 *= 2.0;
                    }
                    if ui.small_button(">>").clicked() {
                        speed.0 *= 10.0;
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    if ui.button("Reset").clicked() {
                        let _ = state.set(SimState::Reset);
                    }
                //    ui.checkbox(&mut lock_on_sun.0, "Lock on Sun");
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
             //       ui.label(format!("{:.0} FPS", fps.0));
                })
            });
        });
}

pub fn system_ui(
    mut egui_context: EguiContexts,
    mut body_query: Query<(
        &Name,
        &mut Selectable,
        &Parent,
        With<Mass>,
        &mut Visibility,
    )>,
    mut camera: Query<&mut Camera>,
    mut light: Query<&mut PointLight>,
    mut state: ResMut<NextState<SimState>>
) {
    let mut new_selected: Option<&Name> = None;
    let mut bodies: Vec<(&Name, Mut<Selectable>)> = Vec::new();
    egui::SidePanel::left("system_panel")
        .default_width(400.0)
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Bodies");
            for (name, selectable, _, _, _) in body_query.iter_mut() {
                ui.horizontal(|ui| {
               //     ui.checkbox(&mut visibility.is_visible, "");
                    if ui.button(name.as_str()).clicked() {
                        new_selected = Some(name);
                    }
                }); 
                bodies.push((name, selectable))
              //  points.push((name, selected));
            }
            ui.heading("Options");
            if let Ok(mut camera) = camera.get_single_mut() {
                ui.checkbox(&mut camera.hdr, "HDR/Bloom");
            }
            if let Ok(mut light) = light.get_single_mut() {
                ui.checkbox(&mut light.shadows_enabled, "Shadows");
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if ui.button("Back to Menu").clicked() {
                    let _ = state.set(SimState::ExitToMainMenu);
                }
            });
        });
    if let Some(new_name) = new_selected {
        for (name, ref mut selectable) in bodies.iter_mut() {
            if name.clone() == new_name.clone() && !selectable.0  {
                selectable.0 = true;
            } else if name.clone() != new_name.clone() && selectable.0 {
                selectable.0 = false;
            }
        }
    }
}

fn body_ui(
    mut egui_context: EguiContexts,
    mut commands: Commands,
    mut query: Query<(&Name, &Selectable, Entity, &Transform, &Velocity, &mut Mass)>,
) {
    let sun_pos = Vec3::splat(0.0);
    if let Some((name, _, entity, transform, velocity, mut mass)) = query.iter_mut().find(|(_, s, _, _, _, _)| s.0) {
        egui::SidePanel::right("body_panel")
                .max_width(250.0)
                .resizable(true)
                .show(egui_context.ctx_mut(), |ui| {
                    ui.heading(name.as_str());

                    //Mass block
                    ui.label(RichText::new("Mass").size(16.0).underline());
                    ui.horizontal(|ui| {
                        let mut new_mass = mass.0.to_string();
                        if ui
                            .add(TextEdit::singleline(&mut new_mass).desired_width(100.0))
                            .changed()
                        {
                            if let Ok(f_mass) = new_mass.parse::<f64>() {
                                mass.0 = f_mass;
                            }
                        }
                        ui.label(" kg");
                    });
                    ui.horizontal(|ui| {
                        if ui.button(":10").clicked() {
                            mass.0 /= 10.0;
                        }
                        if ui.button(":2").clicked() {
                            mass.0 /= 2.0;
                        }
                        if ui.button("x10").clicked() {
                            mass.0 *= 10.0;
                        }
                        if ui.button("x2").clicked() {
                            mass.0 *= 2.0;
                        }
                    });
                    // Position
                    ui.label(
                        RichText::new("Vector Position (unit)")
                            .size(16.0)
                            .underline(),
                    );
                    ui.label(format!(
                        "X: {:.2} Y: {:.2} Z: {:.2}",
                        transform.translation.x, transform.translation.y, transform.translation.z
                    ));
                    // Velocity
                    ui.label(RichText::new("Velocity").size(16.0).underline());
                    ui.label(format!("{:.3} km/s", velocity.0.length() / 1000.0));
                    // Distance from Sun
                    ui.label(RichText::new("Distance from sun").size(16.0).underline());
                    let distance_in_au = transform.translation.distance(sun_pos) / 1000.0;
                    ui.label(format!("{} km", (distance_in_au * 1.496e+8) as f64));
                    ui.label(format!("{:.3} au", distance_in_au));
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        if ui.button("Delete").clicked() {
                            commands.entity(entity).despawn_recursive()
                        }
                    });
                });
    }
    
}
