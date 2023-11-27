use bevy::{
    core_pipeline::Skybox,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::{
        App, Camera, Color, Commands, DespawnRecursiveExt, Entity, GizmoConfig,
        Input, IntoSystemConfigs, KeyCode, Mut, Name, NextState, Plugin, PointLight, Query, Res, ResMut, Resource, Transform, Vec3, Visibility, With, Without, default,
    },
    reflect::Reflect, time::Time, window::PresentMode, math::DVec3,
};
use bevy::app::Update;
use bevy::prelude::{in_state, Window};
use bevy_egui::{egui::{self, InnerResponse, Response, Ui}, EguiContexts};
use bevy_inspector_egui::egui::{RichText, TextEdit};
use chrono::{Days, NaiveDateTime};

//use crate::fps::Fps;
use crate::{body::{BodyChildren, Diameter, Mass, Moon, OrbitSettings, Planet, Scale, SimPosition, Star, Velocity, RotationSpeed}, constants::{DAY_IN_SECONDS, M_TO_UNIT, M_TO_AU}, egui_input_block::BlockInputPlugin, lock_on::LockOn, physics::{apply_physics, SubSteps}, selection::SelectedEntity, setup::StartingTime, skybox::Cubemap, apsis::ApsisBody, unit::format_length, rotation::RotationPlugin, orbit_lines::OrbitOffset, camera::PanOrbitCamera};
use crate::billboard::BillboardSettings;
use crate::body::BodyParent;
use crate::constants::G;
use crate::physics::Pause;
use crate::SimState;
use crate::speed::Speed;
use crate::unit::format_seconds;

#[derive(Resource, Reflect, Default)]
pub struct SimTime(pub f32);

#[derive(Resource, Reflect, Default)]
pub struct Light {
    pub shadows_enabled: bool,
}

#[derive(Reflect)]
pub enum StepType {
    SUBSTEPS,
    TIMESTEPS    
}

#[derive(Resource, Reflect)]
pub struct UiState {
    pub visible: bool,
    pub step_type: StepType,
    pub show_debug: bool,
}

impl Default for UiState {
    fn default() -> Self {
        UiState { visible: true, step_type: StepType::SUBSTEPS, show_debug: false, }
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiState>()
            .register_type::<SimTime>()
            .init_resource::<SimTime>()
            .add_plugins(BlockInputPlugin)
            .add_systems(
                Update,
                (system_ui.after(time_ui), body_ui.after(system_ui), time_ui.after(apply_physics)).run_if(in_state(SimState::Simulation)),
            );
    }
}

pub fn time_ui(
    time: Res<Time>,
    mut sim_time: ResMut<SimTime>,
    mut egui_context: EguiContexts,
    mut speed: ResMut<Speed>,
    mut windows: Query<&mut Window>,
    mut lock_on_parent: ResMut<LockOn>,
    mut pause: ResMut<Pause>,
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<SimState>>,
    starting_time: Res<StartingTime>,
    mut sub_steps: ResMut<SubSteps>,
    mut ui_state: ResMut<UiState>,
    diagnostics: Res<DiagnosticsStore>,
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
                        if ui.small_button("<").clicked() || keys.just_pressed(KeyCode::Left) {
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
                        if ui.button(time_text).clicked() || keys.just_pressed(KeyCode::Space) {
                            pause.0 = !pause.0;
                        }
                        if ui.small_button(">").clicked() || keys.just_pressed(KeyCode::Right) {
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
                    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                        if let Some(value) = fps.smoothed() {
                            // Update the value of the second section
                            ui.label(format!("{:.0} FPS", value));
                        }
                    }
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
        &mut Visibility
    ), (
        With<Star>,
        Without<Planet>,
        Without<Planet>
    )>,
    mut planet_query: Query<(
        &Name,
        &BodyChildren,
        Entity,
        &mut Visibility
    ),(
        With<Planet>,
        Without<Star>,
        Without<Moon>
    )>,
    mut moon_query: Query<(
        &Name,
        Entity,
        &mut Visibility
    ),(
        With<Moon>,
        Without<Planet>,
        Without<Star>
    )>,
    //  mut camera: Query<&mut Camera>,
    mut light: Query<&mut PointLight>,
    mut state: ResMut<NextState<SimState>>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut config: ResMut<GizmoConfig>,
    mut camera: Query<(Entity, &mut Camera, &mut PanOrbitCamera, Option<&Skybox>)>,
    mut commands: Commands,
    mut cubemap: ResMut<Cubemap>,
    mut billboard: ResMut<BillboardSettings>,
    mut ui_state: ResMut<UiState>,
    mut orbit_offset: ResMut<OrbitOffset>
) {
    if !ui_state.visible {
        return;
    }
    if let Ok((entity, mut camera, mut pan, skybox)) = camera.get_single_mut() {
        egui::SidePanel::left("system_panel")
            .default_width(400.0)
            .resizable(true)
            .show(egui_context.ctx_mut(), |ui| {
                ui.heading("Bodies");
                for (s_name, s_children, s_entity, _) in &mut star_query {
                    let s_old_selected = selected_entity.entity == Some(s_entity);
                    let mut s_selected = s_old_selected;
                    body_tree(ui, &mut s_selected, s_name, true, |ui| {
                        for planet_child in &s_children.0 {
                            if let Ok((p_name, p_children, p_entity, _)) = planet_query.get_mut(*planet_child) {
                                let p_old_selected = selected_entity.entity == Some(p_entity);
                                let mut p_selected = p_old_selected;
                                body_tree(ui, &mut p_selected, p_name, false, |ui| {
                                    for moon_child in &p_children.0 {
                                        if let Ok((m_name, m_entity, _)) = moon_query.get_mut(*moon_child) {
                                            let m_old_selected = selected_entity.entity == Some(m_entity);
                                            let mut m_selected = m_old_selected;
                                            ui.horizontal(|ui| {
                                                ui.toggle_value(&mut m_selected, m_name.as_str());
                                            });
                                            if m_selected && !m_old_selected {
                                                selected_entity.change_entity(m_entity)
                                            }
                                        }
                                    }
                                });
                                if p_selected && !p_old_selected {
                                    selected_entity.change_entity(p_entity)
                                }
                            }
                        }
                    });
                    if s_selected && !s_old_selected {
                        selected_entity.change_entity(s_entity)
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
                ui.checkbox(&mut billboard.show, "Show Body Names");
                if ui.checkbox(&mut orbit_offset.enabled, "Offset body to zero").changed() {
                    if orbit_offset.enabled {
                        pan.focus = Vec3::ZERO;
                    }
                }
                if ui.button("Open Debug Window").clicked() {
                    ui_state.show_debug = true; 
                }
                ui.add_space(5.0);
                ui.label("F11 - Toggle Fullscreen");
                ui.label("F10 - Hide Ui");
                ui.label("Space - Pause");
                ui.label("Left Arrow - 2x Speed");
                ui.label("Right Arrow - 1/2 Speed");
                ui.label("C - Reset Camera");
                ui.label("Left Mouse - Rotate Camera");
                ui.label("Right Mouse - Move Camera");

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
    add_body: impl FnOnce(&mut Ui) -> R,
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
    mut query: Query<(&Name, Entity, &SimPosition, &Velocity, &RotationSpeed, &Diameter, &mut OrbitSettings, &mut Mass, &Scale, &mut Transform, Option<&mut ApsisBody>, Option<&BodyChildren>, Option<&BodyParent>)>,
    camera: Query<(&Camera, &Transform, Without<Velocity>)>,
    selected_entity: Res<SelectedEntity>,
    ui_state: Res<UiState>,
) {
    if !ui_state.visible {
        return;
    }
    if let Some(entity) = selected_entity.entity {
        let mut parent: Option<(&SimPosition, &Velocity, &Name, Mass)> = None;
        let mut selected: Option<(&Name, Entity, &SimPosition, &Velocity, &RotationSpeed, &Diameter, Mut<OrbitSettings>, Mut<Transform>, Mut<Mass>, Option<Mut<ApsisBody>>, &Scale, Option<&BodyChildren>)> = None;
        let mut s_children: Vec<(Entity, Mut<OrbitSettings>)> = vec![];
        for (name, b_entity, pos, velocity, rotation_speed, diameter, orbit, mass, scale, transform, apsis, children, maybe_parent) in query.iter_mut() {
            if let Some(children) = children { //check for the parent of the selected entity
                if children.0.contains(&entity) {
                    parent = Some((pos, velocity, name, mass.clone()));
                }
            }
            if b_entity == entity { //check for the selected entity
                selected = Some((name, b_entity, pos, velocity, rotation_speed, diameter, orbit, transform, mass, apsis, scale, children));
            } else if let Some(parent_id) = maybe_parent { //check for potential children of the entity
                if parent_id.0 == entity {
                    s_children.push((b_entity, orbit))
                }
            }
        }
        if let Some((name, entity, pos, velocity, rotation_speed, diameter, mut orbit, mut transform, mut mass, apsis, scale, _)) = selected {
            egui::SidePanel::right("body_panel")
                .max_width(250.0)
                .resizable(true)
                .show(egui_context.ctx_mut(), |ui| {
                    ui.heading(name.as_str());

                    //Mass block
                    ui.label(RichText::new("Mass").size(16.0).underline());
                    ui.horizontal(|ui| {
                        let f_mass = mass.0 * 10e-24;
                        ui.label(format!("{:.3} 10^24 kg", f_mass));
                    });
                    ui.horizontal(|ui| {
                        if ui.button(":5").clicked() {
                            mass.0 /= 5.0;
                        }
                        if ui.button(":2").clicked() {
                            mass.0 /= 2.0;
                        }
                        if ui.button("x5").clicked() {
                            mass.0 *= 5.0;
                        }
                        if ui.button("x2").clicked() {
                            mass.0 *= 2.0;
                        }
                    });
                    if scale.0 != 0.0 {
                        ui.label(
                            RichText::new("Body Scale")
                                .size(16.0)
                                .underline(),
                        );
                        let mut n_scale = transform.scale.x / scale.0;
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::Slider::new(&mut n_scale, 0.001..=100.0)
                                    .clamp_to_range(true)
                                    .logarithmic(true));
                        });
                        transform.scale = Vec3::splat(n_scale * scale.0);
                        ui.label(
                            RichText::new("Equator Diameter")
                                .size(16.0)
                                .underline(),
                        );
                        let scaled_diameter = (diameter.num as f32) * n_scale;
                        ui.label(format!("{} km", scaled_diameter / 1000.0));
                    }


                    // Velocity Orbit Velocity around parent
                    let actual_velocity = match &parent {
                        Some((_, vel, _, _)) => (vel.0 - velocity.0).length() / 1000.0,
                        None => velocity.0.length() / 1000.0,
                    };
                    ui.label(RichText::new("Orbital Velocity").size(16.0).underline());
                    ui.label(format!("{:.3} km/s", actual_velocity));
                    
                    let mut new_apsis = None;
                    if let Some((_, _, _, p_mass)) = parent {
                        if let Some(apsis) = apsis {
                            let distance = ((apsis.aphelion.distance + apsis.perihelion.distance) / 2.0) as f64;                           
                            orbit.period = 2.0 * std::f64::consts::PI * f64::sqrt(f64::powf(distance, 3.0) / (G * (p_mass.0 + mass.0)));
                            ui.label(RichText::new("Orbital Period").size(16.0).underline());
                            ui.label(format!("{}", format_seconds(orbit.period)));
                            new_apsis = Some(apsis);
                        }
                    }
                    
                    ui.label(RichText::new("Rotation Period").size(16.0).underline());
                    ui.label(format!("{}", format_seconds(rotation_speed.0 * 60.0)));

                    ui.label(RichText::new("Distance to Camera").size(16.0).underline());
                    let (_, camera_pos, _) = camera.single();
                    let c_distance_in_au = camera_pos.translation.distance(transform.translation);
                    ui.label(format!("{:.3} au", c_distance_in_au / 10000.0));

                    // Distance to parent
                    if let Some((parent_pos, _, p_name, _)) = parent {
                        ui.label(RichText::new(format!("Distance to {}", p_name)).size(16.0).underline());
                        let distance_in_m = parent_pos.0.distance(pos.0);
                        ui.label(format!("{}", format_length(distance_in_m as f32)));
                        ui.label(format!("{:.3} au", distance_in_m * (M_TO_AU as f64)));
                        
                        if let Some(mut apsis) = new_apsis {
                            //Apsis
                            ui.label(RichText::new(format!("Periapsis ({})", p_name)).size(16.0).underline());
                            ui.label(format!("{}", format_length(apsis.perihelion.distance)));
                            ui.label(format!("{:.3} au", apsis.perihelion.distance * M_TO_AU));                        
        
                            ui.label(RichText::new(format!("Apoapsis ({})", p_name)).size(16.0).underline());
                            ui.label(format!("{}", format_length(apsis.aphelion.distance)));
                            ui.label(format!("{:.3} au", apsis.aphelion.distance * M_TO_AU));
                            if ui.button("Reset Apsides").clicked() {
                               apsis.aphelion.distance = 0.0;
                               apsis.perihelion.distance = 0.0; 
                            }
                            
                            let mut new_draw_lines = orbit.draw_lines;
                            ui.checkbox(&mut new_draw_lines, "Draw Orbit lines");     
                            if new_draw_lines != orbit.draw_lines {
                                orbit.draw_lines = new_draw_lines;
                                if !new_draw_lines {
                                    orbit.lines.clear();
                                }   
                            }
                        }
                    }
                
                    if s_children.iter().count() > 0 {
                        let old_draw_children_orbits = s_children.iter().all(|(_, orbit)| {
                            orbit.draw_lines
                        });
                        let mut draw_children_orbits = old_draw_children_orbits;
                        ui.checkbox(&mut draw_children_orbits, "Draw Children Orbits");
                        if draw_children_orbits != old_draw_children_orbits {
                            for (_, orbit) in s_children.iter_mut() {
                                orbit.draw_lines = draw_children_orbits;
                                if !draw_children_orbits {
                                    orbit.lines.clear();   
                                }
                            }
                        }
                    }

                    ui.horizontal(|ui| {
                        ui.label("Orbit Color");
                        let mut rgb = [orbit.color.r(), orbit.color.g(), orbit.color.b()];
                        ui.color_edit_button_rgb(&mut rgb);
                        orbit.color = Color::rgb(rgb[0], rgb[1], rgb[2]);
                    });

               //     ui.label("Max Orbit Points");
              //      let mut old_length = orbit.lines.capacity();
                //    ui.add(egui::DragValue::new(&mut old_length).speed(1.0));

                 //   if old_length != orbit.lines.capacity() {
                //        orbit.lines.resize(old_length, Vec3::ZERO);
                //    }

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        if ui.button("Delete Children").clicked() {
                            for (entity, _) in s_children {
                                commands.entity(entity).despawn_recursive();
                            }
                        }
                        if ui.button("Delete").clicked() {
                            commands.entity(entity).despawn_recursive();
                        }
                    });
                });
        }
    }
}
