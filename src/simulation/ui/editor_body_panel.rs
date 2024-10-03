use anise::structure::planetocentric::ellipsoid::Ellipsoid;
use bevy::asset::{AssetServer, Assets};
use crate::simulation::components::apsis::ApsisBody;
use crate::simulation::components::body::{AxialTilt, BodyChildren, BodyParent, Diameter, LightSource, Mass, ModelPath, OrbitSettings, RotationSpeed, Scale, SceneEntity, SceneHandle, SimPosition, Velocity};
use crate::simulation::components::selection::SelectedEntity;
use bevy::core::Name;
use bevy::math::DVec3;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{default, BuildChildren, Camera, Color, Commands, DespawnRecursiveExt, Entity, Handle, Mesh, Mut, PointLight, PointLightBundle, Query, Res, ResMut, Resource, Scene, Srgba, Transform, Vec3, Visibility, With, Without};
use bevy_egui::egui::{Align, Context, Layout, RichText, ScrollArea};
use bevy_egui::{egui, EguiContexts};
use egui_toast::{Toast, ToastKind, ToastOptions};
use crate::simulation::scenario::setup::spawn_scene;
use crate::simulation::components::editor::{EditorSystemType, EditorSystems};
use crate::simulation::components::horizons::AniseMetadata;
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::render::star_billboard::StarBillboard;
use crate::simulation::ui::components::vector_field;
use crate::simulation::ui::toast::{success_toast, ToastContainer};
use crate::simulation::ui::UiState;
use crate::simulation::units::converter::{km_to_m, km_to_m_dvec, km_to_m_f64, m_to_km, m_to_km_dvec, m_to_km_f64, scale_lumen, unscale_lumen};

#[derive(Debug, Clone, Resource)]
pub struct EditorPanelState {
    pub entity: Option<Entity>,
    pub new_name: String,
    pub new_position: DVec3,
    pub new_velocity: DVec3,
    pub ellipsoid: Ellipsoid,
    pub new_mass: f64,
    pub new_diameter: f32,
    pub new_rotation_speed: f64,
    pub new_axial_tilt: f32,
    pub new_model_path: String,
    pub show_delete_confirm: bool,
    pub new_light_settings: Option<LightSettings>,
    pub naif_id: i32,
    pub orientation_id: i32,
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
            new_diameter: 0.0,
            new_rotation_speed: 0.0,
            new_axial_tilt: 0.0,
            new_model_path: "".to_string(),
            show_delete_confirm: false,
            new_light_settings: None,
            naif_id: -1,
            orientation_id: -1,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LightSettings {
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub enabled: bool,
}

pub fn editor_body_panel(
    mut egui_context: EguiContexts,
    selected_entity: Res<SelectedEntity>,
    mut query: Query<(Entity, &mut Name, &mut SimPosition, &mut Velocity, &mut Mass, &mut Diameter, &mut RotationSpeed, &mut AxialTilt, &mut ModelPath, &mut SceneHandle, &mut AniseMetadata), With<Mass>>,
    scene_query: Query<Entity, With<SceneEntity>>,
    mut state: ResMut<EditorPanelState>,
    mut commands: Commands,
    systems: Res<EditorSystems>,
    assets: Res<AssetServer>,
    mut light_query: Query<(&mut PointLight, &LightSource, &mut Visibility)>,
    mut billboards: Query<(&StarBillboard, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut toast_container: ResMut<ToastContainer>,
    scale: Res<SimulationScale>
) {
    if egui_context.try_ctx_mut().is_none() {
        return;
    }
    let mut apply  = false;
    if let Some(s_entity) = selected_entity.entity {
        if let Ok((entity, mut name, mut pos, mut vel, mut mass, mut diameter, mut rotation_speed, mut tilt, mut model_path, mut scene, mut horizons_id)) = query.get_mut(s_entity) {
            let light = light_query.iter_mut().find(|(_, l, _)| l.0 == entity).map(|(a,b,c)| (a,b,c));
            let mut billboard_material = billboards.iter_mut().find(|(b, _)| b.0 == entity).map(|(_, m)| m.clone());
            if state.entity.is_none() || state.entity.unwrap() != s_entity {
                initialize_state(state.as_mut(), s_entity, &name, &pos, &vel, &mass, &diameter, &rotation_speed, &tilt, &model_path, light.as_ref(), &scale, &horizons_id);
            }
            display_body_panel(egui_context.ctx_mut(), state.as_mut(), &mut name, &mut pos, &mut vel, &mut mass, &mut diameter, &mut rotation_speed, &mut tilt, &mut model_path, &mut scene, &mut horizons_id, &mut commands, &systems, &assets, light, scene_query, billboard_material.as_mut(), &mut materials, &mut apply, &scale);
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
    diameter: &Diameter,
    rotation_speed: &RotationSpeed,
    tilt: &AxialTilt,
    model_path: &ModelPath,
    light: Option<&(Mut<PointLight>, &LightSource, Mut<Visibility>)>,
    scale: &SimulationScale,
    anise_metadata: &Mut<AniseMetadata>,
) {
    *state = EditorPanelState {
        entity: Some(s_entity),
        new_name: name.to_string(),
        new_position: m_to_km_dvec(pos.0),
        new_velocity: m_to_km_dvec(vel.0),
        new_mass: mass.0,
        new_diameter: m_to_km(diameter.num),
        new_rotation_speed: rotation_speed.0,
        new_axial_tilt: tilt.num,
        new_model_path: model_path.cleaned(),
        show_delete_confirm: false,
        new_light_settings: light.map(|(light, _, visible)| LightSettings {
            color: (*light).color,
            intensity: unscale_lumen((*light).intensity, scale),
            enabled: **visible == Visibility::Visible,
            range: scale.unit_to_m_32(light.range),
        }),
        naif_id: anise_metadata.target_id,
        ellipsoid: diameter.ellipsoid,
        orientation_id: anise_metadata.orientation_id,
    };
}

fn display_body_panel(
    ctx: &mut Context,
    state: &mut EditorPanelState,
    name: &mut Name,
    pos: &mut SimPosition,
    vel: &mut Velocity,
    mass: &mut Mass,
    diameter: &mut Diameter,
    rotation_speed: &mut RotationSpeed,
    tilt: &mut AxialTilt,
    model_path: &mut ModelPath,
    scene: &mut SceneHandle,
    horizons: &mut Mut<AniseMetadata>,
    commands: &mut Commands,
    systems: &Res<EditorSystems>,
    assets: &Res<AssetServer>,
    light: Option<(Mut<PointLight>, &LightSource, Mut<Visibility>)>,
    scene_query: Query<Entity, With<SceneEntity>>,
    billboard_material: Option<&mut Handle<StandardMaterial>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    apply: &mut bool,
    scale: &SimulationScale
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
        ui.label("Naif ID");
        ui.add(egui::DragValue::new(&mut state.naif_id));
    });
    ui.horizontal(|ui| {
        ui.label("Orientation ID");
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
        ui.label("Diameter (km)");
        ui.add(egui::DragValue::new(&mut state.new_diameter));
    });
    ui.horizontal(|ui| {
        ui.label("Rotation Speed (min/rotation)");
        ui.add(egui::DragValue::new(&mut state.new_rotation_speed));
    });
    ui.horizontal(|ui| {
        ui.label("Axial Tilt (degrees)");
        ui.add(egui::DragValue::new(&mut state.new_axial_tilt));
    });
    vector_field(ui, "Position (km)", &mut state.new_position);
    vector_field(ui, "Velocity (km/s)", &mut state.new_velocity);
    ellipsoid(ui, state);
}

fn ellipsoid(ui: &mut egui::Ui, state: &mut EditorPanelState) {
    ui.vertical(|ui| {
        ui.heading("Ellipsoid");
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
        ui.checkbox(&mut light.enabled, "Enabled");
    } else {
        if ui.button("Add Light Source").on_hover_text("Add a light source to the body").clicked() {
            state.new_light_settings = Some(LightSettings {
                color: Color::WHITE,
                intensity: 100.0,
                range: 100.0,
                enabled: true,
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
    diameter: &mut Diameter,
    rotation_speed: &mut RotationSpeed,
    tilt: &mut AxialTilt,
    model_path: &mut ModelPath,
    scene: &mut SceneHandle,
    horizons: &mut Mut<AniseMetadata>,
    commands: &mut Commands,
    systems: &Res<EditorSystems>,
    assets: &Res<AssetServer>,
    light: Option<(Mut<PointLight>, &LightSource, Mut<Visibility>)>,
    scene_query: Query<Entity, With<SceneEntity>>,
    billboard_material: Option<&mut Handle<StandardMaterial>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    apply: &mut bool,
    scale: &SimulationScale
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
        if ui.button("Use starting data from ANISE").on_hover_text("Use starting data from ANISE").clicked() {
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
    diameter: &mut Diameter,
    rotation_speed: &mut RotationSpeed,
    tilt: &mut AxialTilt,
    model_path: &mut ModelPath,
    scene: &mut SceneHandle,
    anise_metadata: &mut AniseMetadata,
    commands: &mut Commands,
    systems: &Res<EditorSystems>,
    assets: &Res<AssetServer>,
    light: Option<(Mut<PointLight>, &LightSource, Mut<Visibility>)>,
    scene_query: Query<Entity, With<SceneEntity>>,
    billboard_material: Option<&mut Handle<StandardMaterial>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    scale: &SimulationScale
) {
    name.set(state.new_name.clone());
    pos.0 = km_to_m_dvec(state.new_position);
    vel.0 = km_to_m_dvec(state.new_velocity);
    mass.0 = state.new_mass;
    let new_diameter = km_to_m(state.new_diameter);
    diameter.applied = new_diameter == diameter.num;
    diameter.num = new_diameter;
    diameter.ellipsoid = state.ellipsoid;
    rotation_speed.0 = state.new_rotation_speed;
    let new_tilt = state.new_axial_tilt;
    tilt.applied = new_tilt == tilt.num;
    tilt.num = new_tilt;
    *anise_metadata = AniseMetadata {
        target_id: state.naif_id,
        orientation_id: state.orientation_id,
    };
    if let Some((mut light, _, mut visible)) = light {
        light.color = state.new_light_settings.as_ref().unwrap().color;
        if let Some(material) = billboard_material {
            let color = light.color.to_srgba();
            let mut rgb = [color.red, color.green, color.blue];
            let new_color: Color = Srgba::rgb(rgb[0] * 30.0, rgb[1] * 30.0, rgb[2] * 30.0).into();
            let mut material = materials.get_mut(material).unwrap();
            material.base_color = new_color;
        }
        light.intensity = scale_lumen(state.new_light_settings.as_ref().unwrap().intensity, scale);
        light.range = scale.m_to_unit_32(state.new_light_settings.as_ref().unwrap().range);
        *visible = if state.new_light_settings.as_ref().unwrap().enabled {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    } else if let Some(light) = state.new_light_settings.as_ref() {
        commands.entity(state.entity.unwrap()).with_children(|parent| {
            parent.spawn(LightSource(state.entity.unwrap()))
                .insert(PointLightBundle {
                    point_light: PointLight {
                        color: light.color,
                        intensity: scale_lumen(light.intensity, scale),
                        range: scale.m_to_unit_32(light.range),
                        radius: new_diameter / 2.0,
                        ..default()
                    },
                    visibility: if light.enabled {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    },
                    ..Default::default()
                });
        });
    }
    if model_path.cleaned() != state.new_model_path {
        *model_path = ModelPath::from_cleaned(state.new_model_path.as_str());
        diameter.path = model_path.0.clone();
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