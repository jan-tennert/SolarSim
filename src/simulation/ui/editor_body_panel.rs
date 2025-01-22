use crate::simulation::components::body::{BodyRotation, BodyShape, LightSource, Mass, ModelPath, RotationSpeed, SceneEntity, SceneHandle, SimPosition, Velocity};
use crate::simulation::components::editor::{EditorSystemType, EditorSystems};
use crate::simulation::components::horizons::AniseMetadata;
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::render::star_billboard::{StarBillboard, SunImposterMaterial};
use crate::simulation::scenario::setup::scenario::spawn_scene;
use crate::simulation::ui::components::vector_field;
use crate::simulation::ui::toast::{success_toast, ToastContainer};
use crate::simulation::units::converter::{km_to_m_dvec, m_to_km_dvec, scale_lumen};
use anise::structure::planetocentric::ellipsoid::Ellipsoid;
use bevy::asset::{AssetServer, Assets};
use bevy::core::Name;
use bevy::math::DVec3;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{default, BuildChildren, ChildBuild, Color, Commands, DespawnRecursiveExt, Entity, Handle, Mat3, Mut, PointLight, Query, Res, ResMut, Resource, Scene, Srgba, Visibility, With};
use bevy_egui::egui::{Align, Context, Layout, ScrollArea};
use bevy_egui::{egui, EguiContexts};

#[derive(Debug, Clone, Resource)]
pub struct EditorPanelState {
    pub entity: Option<Entity>,
    pub new_name: String,
    pub new_position: DVec3,
    pub new_velocity: DVec3,
    pub ellipsoid: Ellipsoid,
    pub new_mass: f64,
    pub new_rotation_speed: f64,
    pub new_model_path: String,
    pub show_delete_confirm: bool,
    pub new_light_settings: Option<LightSettings>,
    pub ephemeris_id: i32,
    pub orientation_id: i32,
    pub target_id: i32,
    pub rotation_matrix: Mat3
}

impl Default for EditorPanelState {
    fn default() -> Self {
        Self {
            entity: None,
            new_name: "".to_string(),
            new_position: DVec3::ZERO,
            new_velocity: DVec3::ZERO,
            ellipsoid: Ellipsoid::from_sphere(1.0),
            new_mass: 0.0,
            new_rotation_speed: 0.0,
            new_model_path: "".to_string(),
            show_delete_confirm: false,
            new_light_settings: None,
            ephemeris_id: -1,
            orientation_id: -1,
            target_id: -1,
            rotation_matrix: Mat3::IDENTITY
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LightSettings {
    pub color: Color,
    pub imposter_color: Color,
    pub intensity: f32,
    pub range: f32,
    pub enabled: bool,
}

pub fn editor_body_panel(
    mut egui_context: EguiContexts,
    selected_entity: Res<SelectedEntity>,
    mut query: Query<(Entity, &mut Name, &mut SimPosition, &mut Velocity, &mut Mass, &mut BodyShape, &mut RotationSpeed, &mut BodyRotation, &mut ModelPath, &mut SceneHandle, &mut AniseMetadata), With<Mass>>,
    scene_query: Query<Entity, With<SceneEntity>>,
    mut state: ResMut<EditorPanelState>,
    mut commands: Commands,
    systems: Res<EditorSystems>,
    assets: Res<AssetServer>,
    mut light_query: Query<(&mut PointLight, &mut LightSource, &mut Visibility)>,
    mut billboards: Query<(&StarBillboard, &mut MeshMaterial3d<SunImposterMaterial>)>,
    mut materials: ResMut<Assets<SunImposterMaterial>>,
    mut toast_container: ResMut<ToastContainer>,
    scale: Res<SimulationScale>
) {
    if egui_context.try_ctx_mut().is_none() {
        return;
    }
    let mut apply  = false;
    if let Some(s_entity) = selected_entity.entity {
        if let Ok((entity, mut name, mut pos, mut vel, mut mass, mut diameter, mut rotation_speed, mut rotation, mut model_path, mut scene, mut horizons_id)) = query.get_mut(s_entity) {
            let light = light_query.iter_mut().find(|(_, l, _)| l.parent == entity).map(|(a,b,c)| (a,b,c));
            let mut billboard_material = billboards.iter_mut().find(|(b, _)| b.0 == entity).map(|(_, m)| m.clone());
            if state.entity.is_none() || state.entity.unwrap() != s_entity {
                initialize_state(state.as_mut(), s_entity, &name, &pos, &vel, &mass, &diameter, &rotation_speed,&model_path, light.as_ref(), &horizons_id, &rotation);
            }
            display_body_panel(egui_context.ctx_mut(), state.as_mut(), &mut name, &mut pos, &mut vel, &mut mass, &mut diameter, &mut rotation_speed, &mut rotation, &mut model_path, &mut scene, &mut horizons_id, &mut commands, &systems, &assets, light, scene_query, billboard_material.as_mut(), &mut materials, &mut apply, &scale);
        }
    } else {
        state.entity = None;
    }
    if apply {
        toast_container.0.add(success_toast("Changes applied"));
    }
}

fn initialize_state(
    state: &mut EditorPanelState,
    s_entity: Entity,
    name: &Name,
    pos: &SimPosition,
    vel: &Velocity,
    mass: &Mass,
    diameter: &BodyShape,
    rotation_speed: &RotationSpeed,
    model_path: &ModelPath,
    light: Option<&(Mut<PointLight>, Mut<LightSource>, Mut<Visibility>)>,
    anise_metadata: &Mut<AniseMetadata>,
    rotation: &BodyRotation,
) {
    *state = EditorPanelState {
        entity: Some(s_entity),
        new_name: name.to_string(),
        new_position: m_to_km_dvec(pos.current),
        new_velocity: m_to_km_dvec(vel.0),
        new_mass: mass.0,
        new_rotation_speed: rotation_speed.0,
        new_model_path: model_path.cleaned(),
        show_delete_confirm: false,
        new_light_settings: light.map(|(_, source, _)| LightSettings {
            color: source.color,
            intensity: source.intensity,
            enabled: source.enabled,
            range: source.range,
            imposter_color: source.imposter_color
        }),
        ephemeris_id: anise_metadata.ephemeris_id,
        ellipsoid: diameter.ellipsoid,
        orientation_id: anise_metadata.orientation_id,
        rotation_matrix: rotation.matrix,
        target_id: anise_metadata.target_id
    };
}

fn display_body_panel(
    ctx: &mut Context,
    state: &mut EditorPanelState,
    name: &mut Name,
    pos: &mut SimPosition,
    vel: &mut Velocity,
    mass: &mut Mass,
    diameter: &mut BodyShape,
    rotation_speed: &mut RotationSpeed,
    tilt: &mut BodyRotation,
    model_path: &mut ModelPath,
    scene: &mut SceneHandle,
    horizons: &mut Mut<AniseMetadata>,
    commands: &mut Commands,
    systems: &Res<EditorSystems>,
    assets: &Res<AssetServer>,
    light: Option<(Mut<PointLight>, Mut<LightSource>, Mut<Visibility>)>,
    scene_query: Query<Entity, With<SceneEntity>>,
    billboard_material: Option<&mut MeshMaterial3d<SunImposterMaterial>>,
    materials: &mut ResMut<Assets<SunImposterMaterial>>,
    apply: &mut bool,
    scale: &SimulationScale,
) {
    egui::SidePanel::right("body_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ScrollArea::vertical()
                .auto_shrink(true)
                .show(ui, |ui| {
                    ui.heading("Body");
                    display_body_properties(ui, state);
                    display_light_source(ui, state);
                    display_bottom_buttons(ui, state, name, pos, vel, mass, diameter, rotation_speed, tilt, model_path, scene, horizons, commands, systems, assets, light, scene_query, billboard_material, materials, apply, scale);
                });
        });
}

fn display_body_properties(ui: &mut egui::Ui, state: &mut EditorPanelState) {
    ui.horizontal(|ui| {
        ui.label("Name");
        ui.text_edit_singleline(&mut state.new_name);
    });
    ui.horizontal(|ui| {
        ui.label("Ephemeris ID");
        ui.add(egui::DragValue::new(&mut state.ephemeris_id));
    });
    ui.horizontal(|ui| {
        ui.label("Fixed Frame IDs");
        ui.add(egui::DragValue::new(&mut state.target_id));
        ui.label("/");
        ui.add(egui::DragValue::new(&mut state.orientation_id));
    });
    ui.horizontal(|ui| {
        ui.label("Model Path");
        ui.text_edit_singleline(&mut state.new_model_path);
    });
    ui.horizontal(|ui| {
        ui.label("Mass (kg)");
        ui.add(egui::DragValue::new(&mut state.new_mass));
    });
    ui.horizontal(|ui| {
        ui.label("Rotation Speed (min/rotation)");
        ui.add(egui::DragValue::new(&mut state.new_rotation_speed));
    });
    vector_field(ui, "Position (km)", &mut state.new_position);
    vector_field(ui, "Velocity (km/s)", &mut state.new_velocity);
    rotation_matrix(ui, state);
    ellipsoid(ui, state);
}

fn ellipsoid(ui: &mut egui::Ui, state: &mut EditorPanelState) {
    ui.vertical(|ui| {
        ui.heading("Shape");
        ui.horizontal(|ui| {
            ui.label("Polar Radius (km)");
            ui.add(egui::DragValue::new(&mut state.ellipsoid.polar_radius_km));
        });
        ui.horizontal(|ui| {
            ui.label("Semi major equatorial radius (km)");
            ui.add(egui::DragValue::new(&mut state.ellipsoid.semi_major_equatorial_radius_km));
        });
        ui.horizontal(|ui| {
            ui.label("Semi minor equatorial radius (km)");
            ui.add(egui::DragValue::new(&mut state.ellipsoid.semi_minor_equatorial_radius_km));
        });
    });
}

fn rotation_matrix(ui: &mut egui::Ui, state: &mut EditorPanelState) {
    ui.vertical(|ui| {
        ui.heading("Rotation Matrix");
        ui.horizontal(|ui| {
            ui.label("Row 1");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut state.rotation_matrix.x_axis[0]));
                ui.add(egui::DragValue::new(&mut state.rotation_matrix.x_axis[1]));
                ui.add(egui::DragValue::new(&mut state.rotation_matrix.x_axis[2]));
            });
        });
        ui.horizontal(|ui| {
            ui.label("Row 2");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut state.rotation_matrix.y_axis[0]));
                ui.add(egui::DragValue::new(&mut state.rotation_matrix.y_axis[1]));
                ui.add(egui::DragValue::new(&mut state.rotation_matrix.y_axis[2]));
            });
        });
        ui.horizontal(|ui| {
            ui.label("Row 3");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut state.rotation_matrix.z_axis[0]));
                ui.add(egui::DragValue::new(&mut state.rotation_matrix.z_axis[1]));
                ui.add(egui::DragValue::new(&mut state.rotation_matrix.z_axis[2]));
            });
        });
    });
}

fn display_light_source(ui: &mut egui::Ui, state: &mut EditorPanelState) {
    ui.heading("Light Source");
    if let Some(mut light) = state.new_light_settings.as_mut() {
        ui.horizontal(|ui| {
            ui.label("Intensity (Lm)");
            ui.add(egui::DragValue::new(&mut light.intensity));
        });
        ui.horizontal(|ui| {
            ui.label("Range (m)");
            ui.add(egui::DragValue::new(&mut light.range));
        });
        ui.horizontal(|ui| {
            ui.label("Color");
            let color = light.color.to_srgba();
            let mut rgb = [color.red, color.green, color.blue];
            ui.color_edit_button_rgb(&mut rgb);
            light.color = Srgba::rgb(rgb[0], rgb[1], rgb[2]).into();
        });
        ui.horizontal(|ui| {
            ui.label("Imposter Color");
            let color = light.imposter_color.to_srgba();
            let mut rgb = [color.red, color.green, color.blue];
            ui.color_edit_button_rgb(&mut rgb);
            light.imposter_color = Srgba::rgb(rgb[0], rgb[1], rgb[2]).into();
        });
        ui.checkbox(&mut light.enabled, "Enabled");
    } else {
        if ui.button("Add Light Source").on_hover_text("Add a light source to the body").clicked() {
            state.new_light_settings = Some(LightSettings {
                color: Color::WHITE,
                intensity: 100.0,
                range: 100.0,
                enabled: true,
                imposter_color: Color::WHITE
            });
        }
    }
}

fn display_bottom_buttons(
    ui: &mut egui::Ui,
    state: &mut EditorPanelState,
    name: &mut Name,
    pos: &mut SimPosition,
    vel: &mut Velocity,
    mass: &mut Mass,
    diameter: &mut BodyShape,
    rotation_speed: &mut RotationSpeed,
    tilt: &mut BodyRotation,
    model_path: &mut ModelPath,
    scene: &mut SceneHandle,
    horizons: &mut Mut<AniseMetadata>,
    commands: &mut Commands,
    systems: &Res<EditorSystems>,
    assets: &Res<AssetServer>,
    light: Option<(Mut<PointLight>, Mut<LightSource>, Mut<Visibility>)>,
    scene_query: Query<Entity, With<SceneEntity>>,
    billboard_material: Option<&mut MeshMaterial3d<SunImposterMaterial>>,
    materials: &mut ResMut<Assets<SunImposterMaterial>>,
    apply: &mut bool,
    scale: &SimulationScale,
) {
    ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Apply").on_hover_text("Apply changes").clicked() {
                apply_changes(state, name, pos, vel, mass, diameter, rotation_speed, tilt, model_path, scene, horizons, commands, systems, assets, light, scene_query, billboard_material, materials, scale);
                *apply = true;
            }
            if ui.button("Reset").on_hover_text("Reset to original values").clicked() {
                // Reset logic here
            }
            if state.show_delete_confirm {
                ui.label("Are you sure?");
                if ui.button("Yes").on_hover_text("Delete selected body").clicked() {
                    commands.entity(state.entity.unwrap()).despawn_recursive();
                    state.show_delete_confirm = false;
                }
                if ui.button("No").on_hover_text("Cancel deletion").clicked() {
                    state.show_delete_confirm = false;
                }
            } else {
                if ui.button("Delete").on_hover_text("Delete selected body").clicked() {
                    state.show_delete_confirm = true;
                }
            }
        });
        ui.separator();
        if ui.button("Load starting data from included SPK kernels").on_hover_text("Use starting data from ANISE").clicked() {
            commands.run_system(systems.0[EditorSystemType::RETRIEVE_DATA]);
        }
    });
}

fn apply_changes(
    state: &mut EditorPanelState,
    name: &mut Name,
    pos: &mut SimPosition,
    vel: &mut Velocity,
    mass: &mut Mass,
    shape: &mut BodyShape,
    rotation_speed: &mut RotationSpeed,
    rotation: &mut BodyRotation,
    model_path: &mut ModelPath,
    scene: &mut SceneHandle,
    anise_metadata: &mut AniseMetadata,
    commands: &mut Commands,
    systems: &Res<EditorSystems>,
    assets: &Res<AssetServer>,
    light: Option<(Mut<PointLight>, Mut<LightSource>, Mut<Visibility>)>,
    scene_query: Query<Entity, With<SceneEntity>>,
    billboard_material: Option<&mut MeshMaterial3d<SunImposterMaterial>>,
    materials: &mut ResMut<Assets<SunImposterMaterial>>,
    scale: &SimulationScale,
) {
    name.set(state.new_name.clone());
    pos.current = km_to_m_dvec(state.new_position);
    vel.0 = km_to_m_dvec(state.new_velocity);
    mass.0 = state.new_mass;
    shape.applied = state.ellipsoid == shape.ellipsoid;
    shape.ellipsoid = state.ellipsoid;
    rotation_speed.0 = state.new_rotation_speed;
    rotation.applied = state.rotation_matrix == rotation.matrix;
    rotation.matrix = state.rotation_matrix;
    *anise_metadata = AniseMetadata {
        ephemeris_id: state.ephemeris_id,
        orientation_id: state.orientation_id,
        target_id: state.target_id,
    };
    if let Some((mut light, mut source, mut visible)) = light {
        light.color = state.new_light_settings.as_ref().unwrap().color;
        if let Some(material) = billboard_material {
            let color = state.new_light_settings.unwrap().imposter_color.to_srgba();
            let mut rgb = [color.red, color.green, color.blue];
            let new_color: Color = Srgba::rgb(rgb[0] * 20.0, rgb[1] * 20.0, rgb[2] * 20.0).into();
            let mut material = materials.get_mut(material.clone()).unwrap();
            material.color = new_color.into();
        }
        light.intensity = scale_lumen(state.new_light_settings.as_ref().unwrap().intensity, scale);
        light.range = scale.m_to_unit_32(state.new_light_settings.as_ref().unwrap().range);
        LightSource::apply(&mut source, state.new_light_settings.as_ref().unwrap());
        *visible = if state.new_light_settings.as_ref().unwrap().enabled {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    } else if let Some(light) = state.new_light_settings.as_ref() {
        commands.entity(state.entity.unwrap()).with_children(|parent| {
            parent.spawn(LightSource::new_settings(state.entity.unwrap(), light))
                .insert(PointLight {
                    color: light.color,
                    intensity: scale_lumen(light.intensity, scale),
                    range: scale.m_to_unit_32(light.range),
                    radius: shape.ellipsoid.mean_equatorial_radius_km() as f32,
                    ..default()
                })
                .insert(if light.enabled {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                });
        });
    }
    if model_path.cleaned() != state.new_model_path {
        *model_path = ModelPath::from_cleaned(state.new_model_path.as_str());
        shape.path = model_path.0.clone();
        let asset_handle: Handle<Scene> = assets.load(model_path.clone().0);
        commands.entity(scene_query.get(scene.1).unwrap()).despawn_recursive();
        scene.0 = asset_handle.clone();
        commands.entity(state.entity.unwrap()).with_children(|parent| {
            scene.1 = spawn_scene(
                asset_handle,
                state.new_name.as_str(),
                parent
            );
        });
    }
    commands.run_system(systems.0[EditorSystemType::UPDATE_POSITIONS]);
    commands.run_system(systems.0[EditorSystemType::UPDATE_DIAMETER]);
    commands.run_system(systems.0[EditorSystemType::UPDATE_TILT]);
}