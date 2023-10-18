use std::collections::HashMap;

use bevy::{
    prelude::{
        App, Camera, Commands, DespawnRecursiveExt, Entity, Input, KeyCode,
        Mut, Name, Plugin, PointLight, Query, Res, ResMut, Resource, Transform, Vec3, Visibility, With, Without, NextState, Children, IntoSystemConfigs, GizmoConfig, Color, AssetServer,
    },
    reflect::Reflect,
    time::Time,
    window::PresentMode, render::camera::TemporalJitter, pbr::{ScreenSpaceAmbientOcclusionSettings, ScreenSpaceAmbientOcclusionQualityLevel}, core_pipeline::Skybox, transform::commands,
};
use bevy::app::Update;
use bevy::prelude::{in_state, Window};
use bevy_egui::{egui::{self, Ui, InnerResponse, Response, ComboBox}, EguiContexts, EguiPlugin};
use bevy_inspector_egui::egui::{RichText, TextEdit};
use chrono::{Days, NaiveDate, Utc, DateTime, NaiveDateTime};
//use crate::fps::Fps;
use crate::{egui_input_block::BlockInputPlugin, body::{Mass, Velocity, Star, Moon, Planet, BodyChildren, OrbitSettings, SimPosition}, constants::{DAY_IN_SECONDS, M_TO_UNIT}, selection::SelectedEntity, orbit_lines, fps::Fps, skybox::Cubemap, setup::StartingTime, lock_on::LockOn, physics::{apply_physics, SubSteps}};
use crate::physics::{Pause, update_position};
use crate::SimState;
use crate::speed::Speed;

#[derive(Resource, Reflect, Default)]
pub struct SimTime(pub f32);

#[derive(Resource, Reflect, Default)]
pub struct Light {
    pub shadows_enabled: bool,
}

#[derive(Resource, Reflect)]
pub struct UiState {
    pub visible: bool,
}

impl Default for UiState {
    fn default() -> Self {
        UiState { visible: true }
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_plugins(EguiPlugin)
            .init_resource::<UiState>()
            .register_type::<SimTime>()
            .init_resource::<SimTime>()
            .add_plugins(BlockInputPlugin)
            .add_systems(
                Update,
                (system_ui.after(time_ui), body_ui.after(apply_physics), time_ui.after(body_ui)).run_if(in_state(SimState::Simulation))
            );
    }
}

pub fn time_ui(
    time: Res<Time>,
    mut sim_time: ResMut<SimTime>,
    mut egui_context: EguiContexts,
    mut speed: ResMut<Speed>,
    fps: Res<Fps>,
    mut windows: Query<&mut Window>,
    mut lock_on_parent: ResMut<LockOn>,
    mut pause: ResMut<Pause>,
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<SimState>>,
    starting_time: Res<StartingTime>,
    mut sub_steps: ResMut<SubSteps>,
    ui_state: Res<UiState>
) {
    if !ui_state.visible {
        return;
    }
    let mut window = windows.single_mut();
    if !pause.0 {
        sim_time.0 += time.delta_seconds() * (((speed.0 * (sub_steps.0 as f64)) / (DAY_IN_SECONDS as f64)) as f32);
    }
    let date = NaiveDateTime::from_timestamp_millis(starting_time.0)
        .unwrap()
        .checked_add_days(Days::new((((sim_time.0 * 100.0).round()) / 100.0) as u64))
        .unwrap();
    egui::TopBottomPanel::bottom("time_panel")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.horizontal_centered(|ui| {
                    if ui.small_button("<<").clicked() {
                        sub_steps.big_step_down();
                    }
                    if ui.small_button("<").clicked() || keys.just_pressed(KeyCode::Left) {
                        sub_steps.small_step_down();
                    }
                    ui.label(format!(
                        "{} ({})",
                        date.format("%d.%m.%Y"),
                        speed.format(sub_steps.0)
                    ));
                    let time_text = if !pause.0 { "Pause" } else { "Resume" };
                    if ui.button(time_text).clicked() || keys.just_pressed(KeyCode::Space) {
                        pause.0 = !pause.0;
                    }
                    if ui.small_button(">").clicked() || keys.just_pressed(KeyCode::Right) {
                        sub_steps.small_step_up();
                    }
                    if ui.small_button(">>").clicked() {
                        sub_steps.big_step_up();
                    }
             //       ui.add_space(20.0);
                    ui.label("Substeps per second");
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
                    ui.label("Timestep in seconds");
                    let mut new_speed = speed.0.to_string();
                    if ui
                        .add(TextEdit::singleline(&mut new_speed).desired_width(50.0))
                        .changed()
                    {
                        if let Ok(new_speed_num) = new_speed.parse::<f64>() {
                            speed.0 = new_speed_num;
                        }
                    }
                    });
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    if ui.button("Reset").clicked() {
                        let _ = state.set(SimState::Reset);
                    }
                    ui.checkbox(&mut lock_on_parent.0, "Lock on Parent");
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
                    ui.label(format!("{:.0} FPS", fps.0));
                })
            });
        });
}

pub fn system_ui(
    mut egui_context: EguiContexts,
    mut star_query: Query<(
        &Name,
        &BodyChildren,
        Entity,
        &mut Visibility,        
        With<Star>,
        Without<Planet>,
        Without<Planet>
    )>,
    mut planet_query: Query<(
        &Name,
        &BodyChildren,
        Entity,
        &mut Visibility,        
        With<Planet>,
        Without<Star>,
        Without<Moon>
    )>,
    mut moon_query: Query<(
        &Name,
        Entity,
        &mut Visibility,        
        With<Moon>,
        Without<Planet>,
        Without<Star>
    )>,
  //  mut camera: Query<&mut Camera>,
    mut light: Query<&mut PointLight>,
    mut state: ResMut<NextState<SimState>>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut config: ResMut<GizmoConfig>,
    mut camera: Query<(Entity, &mut Camera, Option<&Skybox>)>,    
    mut commands: Commands,
    mut cubemap: ResMut<Cubemap>,
    ui_state: Res<UiState>
) {
    if !ui_state.visible {
        return;
    }
    if let Ok((entity, mut camera, skybox)) = camera.get_single_mut() {
        egui::SidePanel::left("system_panel")
                .default_width(400.0)
                .resizable(true)
                .show(egui_context.ctx_mut(), |ui| {
                    ui.heading("Bodies");
                    for (s_name, s_children, s_entity,  _, _, _, _) in &mut star_query {
                        let s_old_selected = selected_entity.0 == Some(s_entity);
                        let mut s_selected = s_old_selected;
                        body_tree(ui, &mut s_selected, s_name, true, |ui| {
                            for planet_child in &s_children.0 {
                                if let Ok((p_name, p_children, p_entity, _, _, _, _)) = planet_query.get_mut(*planet_child) {
                                    let p_old_selected = selected_entity.0 == Some(p_entity);
                                    let mut p_selected = p_old_selected;
                                    body_tree(ui, &mut p_selected, p_name, false, |ui| {
                                        for moon_child in &p_children.0 {
                                            if let Ok((m_name, m_entity,  _, _, _, _)) = moon_query.get_mut(*moon_child) {
                                                let m_old_selected = selected_entity.0 == Some(m_entity);
                                                let mut m_selected = m_old_selected;
                                                ui.horizontal(|ui| {
                                                    ui.toggle_value(&mut m_selected, m_name.as_str());
                                                });
                                                if m_selected && !m_old_selected {
                                                    selected_entity.0 = Some(m_entity);
                                                }
                                            }
                                        }          
                                    });
                                    if p_selected && !p_old_selected {
                                        selected_entity.0 = Some(p_entity);
                                    }
                                }
                            } 
                        });
                        if s_selected && !s_old_selected {
                            selected_entity.0 = Some(s_entity);
                        }
                    }
                    ui.heading("Options");
                    ui.checkbox(&mut camera.hdr, "HDR/Bloom");
                    if let Ok(mut light) = light.get_single_mut() {
                        ui.checkbox(&mut light.shadows_enabled, "Shadows");
                    }
                    let skybox_enabled = skybox.is_some();
                    let mut skybox_setting = skybox_enabled;
                    ui.checkbox(&mut skybox_setting, "Milky Way Skybox");
                    
                    if skybox_enabled && !skybox_setting {
                        commands.entity(entity).remove::<Skybox>();
                        cubemap.activated = false;
                    } else if !skybox_enabled && skybox_setting {
                        commands.entity(entity).insert(Skybox(cubemap.image_handle.clone()));
                        cubemap.activated = true;
                    }
                    
                    ui.checkbox(&mut config.aabb.draw_all, "Draw Outlines");
                    ui.add_space(10.0);
                    ui.label("F10 - Hide Ui");
                    ui.label("Space - Pause");
                    ui.label("Left Arrow - 2x Speed");
                    ui.label("Right Arrow - 1/2 Speed");
                    
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        if ui.button("Back to Menu").clicked() {
                            let _ = state.set(SimState::ExitToMainMenu);
                        }
                    });
                });
    }    
}

fn body_tree<R>(
    ui: &mut Ui, 
    mut selected: &mut bool, 
    name: &Name, 
    default_open: bool,
    add_body: impl FnOnce(&mut Ui) -> R
) -> (
    Response,
    InnerResponse<()>,
    Option<InnerResponse<R>>,
) {
    let id = ui.make_persistent_id(name.as_str());
    egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, default_open)
        .show_header(ui, |ui| {
            ui.toggle_value(&mut selected, name.as_str());
        })
        .body(add_body)
}

fn body_ui(
    mut egui_context: EguiContexts,
    mut commands: Commands,
    mut query: Query<(&Name, Entity, &SimPosition, &Velocity, &mut OrbitSettings, &mut Mass, Option<&BodyChildren>)>,
    selected_entity: Res<SelectedEntity>,   
    ui_state: Res<UiState>
) {
    if !ui_state.visible {
        return;
    }
    if let Some(entity) = selected_entity.0 {
        let mut parent: Option<(&SimPosition, &Velocity, &Name)> = None;
        let mut selected: Option<(&Name, Entity, &SimPosition, &Velocity, Mut<OrbitSettings>, Mut<Mass>)> = None;
        for (name, b_entity, pos, velocity, orbit, mass, children) in query.iter_mut() {
            if let Some(children) = children {
                if children.0.contains(&entity) {
                    parent = Some((pos, velocity, name));
                }
            }
            if b_entity == entity {
                selected = Some((name, b_entity, pos, velocity, orbit, mass));
            }
        }
        if let Some((name, entity, pos, velocity, mut orbit, mut mass)) = selected {
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
                        RichText::new("Vector Position (km)")
                            .size(16.0)
                            .underline(),
                    );
                    ui.label(format!(
                        "X: {:.2} Y: {:.2} Z: {:.2}",
                        pos.0.x / 1000.0, pos.0.y / 1000.0, pos.0.z / 1000.0
                    ));
                    // Velocity Orbit Velocity around parent
                    let actual_velocity = match parent {
                        Some((_, vel, _)) => (vel.0 - velocity.0).length() / 1000.0,
                        None => velocity.0.length() / 1000.0,
                    };
                    ui.label(RichText::new("Orbital Velocity").size(16.0).underline());
                    ui.label(format!("{:.3} km/s", actual_velocity));
                    // Distance to parent
                    if let Some((parent_pos, _, p_name)) = parent {
                        ui.label(RichText::new(format!("Distance to {}", p_name)).size(16.0).underline());
                        let distance_in_km = parent_pos.0.distance(pos.0) / 10000.0;
                        ui.label(format!("{:.3} km", distance_in_km));
                        ui.label(format!("{:.3} au", distance_in_km * M_TO_UNIT));
                        
                        let old_draw_orbit = orbit.draw_lines;
                        ui.checkbox(&mut orbit.draw_lines, "Draw Orbit lines");
                        
                        if old_draw_orbit && !orbit.draw_lines {
                            orbit.lines.clear();
                        }
                                                
                        ui.horizontal(|ui| {
                            ui.label("Orbit Color");    
                            let mut rgb = [orbit.color.r(), orbit.color.g(), orbit.color.b()];    
                            ui.color_edit_button_rgb(&mut rgb);
                            orbit.color = Color::rgb(rgb[0], rgb[1], rgb[2]);
                        });

                        ui.label("Max Orbit Points");    
                        ui.add(egui::DragValue::new(&mut orbit.max_points).speed(1.0));
                        
                        if orbit.max_points < 1 {
                            orbit.max_points = 1;
                        }
                        
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                            if ui.button("Delete").clicked() {
                                commands.entity(entity).despawn_recursive()
                            }
                        });
                    }
                });
            }
    }
    
}
