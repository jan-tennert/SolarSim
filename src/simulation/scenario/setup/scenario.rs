use crate::simulation::asset::serialization::{SerializedBody, SerializedLightSource, SimulationData};
use crate::simulation::components::apsis::ApsisBody;
use crate::simulation::components::body::{BodyBundle, BodyChildren, BodyParent, LightSource, Moon, OrbitSettings, Planet, SceneEntity, SceneHandle, Star};
use crate::simulation::components::editor::CreateBodyType;
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::components::selection::{SelectedEntity, SELECTION_MULTIPLIER};
use crate::simulation::render::star_billboard::{StarBillboard, SunImposterMaterial};
use crate::simulation::scenario::loading::LoadingState;
use crate::simulation::ui::scenario_selection::SelectedScenario;
use crate::simulation::units::converter::scale_lumen;
use crate::simulation::SimState;
use bevy::asset::AssetServer;
use bevy::color::palettes::css::WHITE;
use bevy::ecs::system::EntityCommands;
use bevy::math::{DVec3, Vec3};
use bevy::pbr::{MeshMaterial3d, PointLight};
use bevy::prelude::{default, Assets, Circle, Color, Commands, Entity, Handle, Hsva, JustifyText, Mesh, Mesh3d, NextState, Query, Res, ResMut, Resource, Srgba, TextFont, Transform, Visibility};
use bevy::prelude::{ChildSpawnerCommands, Name};
use bevy::scene::{Scene, SceneRoot};
use bevy::text::{TextColor, TextLayout};
use bevy_mod_billboard::BillboardText;
use bevy_panorbit_camera::PanOrbitCamera;
use std::collections::HashMap;

#[derive(Resource, Default, Clone, Debug)]
pub struct ScenarioData {

    pub starting_time_millis: i64,
    pub title: String,
    pub description: String,
    pub timestep: i32,
    pub scale: f32,
    pub spice_files: HashMap<String, bool>

}

impl From<SimulationData> for ScenarioData {

    fn from(value: SimulationData) -> Self {
        Self {
            starting_time_millis: value.starting_time_millis,
            title: value.title,
            description: value.description,
            timestep: value.timestep,
            scale: value.scale,
            spice_files: value.data_sets.iter().map(|d| (d.clone(), false)).collect()
        }
    }
}

pub fn setup_scenario(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut selected_scenario: ResMut<SelectedScenario>,
    bodies_asset: ResMut<Assets<SimulationData>>,
    mut scenario_data: ResMut<ScenarioData>,
    mut loading_state: ResMut<LoadingState>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sun_materials: ResMut<Assets<SunImposterMaterial>>,
    mut sim_state: ResMut<NextState<SimState>>,
    scale: Res<SimulationScale>,
    mut cam: Query<&mut PanOrbitCamera>
) {
    if selected_scenario.spawned {
        return;
    }
    let mut total_count = 0;
    let bodies = bodies_asset.get(&selected_scenario.handle);
    if bodies.cloned().is_none() {
        return;
    }
    let data = bodies.unwrap();
    *scenario_data = ScenarioData::from(data.clone());

    let mut stars = vec![];
    //iterate through the stars
    recursive_bodies(
        data.bodies.iter().collect(),
        &mut commands,
        &scale,
        &assets,
        &mut meshes,
        &mut sun_materials,
        0,
        &mut stars,
        None,
        &mut total_count
    );
    if selected_entity.entity.is_none() {
        if let Some(star) = stars.first() {
            selected_entity.change_entity(*star, false)
        }
    }
    selected_scenario.spawned = true;
    loading_state.spawned_bodies = true;
    loading_state.total_bodies = total_count;
    if total_count == 0 {
        sim_state.set(SimState::Loaded);
    } else {
        let mut cam = cam.single_mut().unwrap();
        let star = data.bodies.first().unwrap();
        cam.target_radius = scale.m_to_unit_32(star.data.ellipsoid.mean_equatorial_radius_km() as f32 * 2000. * SELECTION_MULTIPLIER);
    }
}

pub fn recursive_bodies(
    bodies: Vec<&SerializedBody>,
    commands: &mut Commands,
    scale: &SimulationScale,
    assets: &Res<AssetServer>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut sun_materials: &mut ResMut<Assets<SunImposterMaterial>>,
    current_depth: usize,
    parent_children: &mut Vec<Entity>,
    parent: Option<Entity>,
    count: &mut i32
) {
    let total_count = bodies.iter().count();
    *count += total_count as i32;
    for (index, serialized_body) in bodies.iter().enumerate() {
        if !serialized_body.data.simulate {
            continue;
        }
        let id = commands.spawn((Visibility::default(), Transform::default())).id();

        //planets vector for adding BodyChildren later
        let mut children: Vec<Entity> = vec![];

        parent_children.push(id);

        if !serialized_body.children.is_empty() {
            let mut sorted_children = serialized_body.children.iter().collect::<Vec<_>>();
            sort_bodies(&mut sorted_children, -DVec3::from(serialized_body.clone().data.starting_position));
            recursive_bodies(sorted_children, commands, &scale, &assets, &mut meshes, &mut sun_materials, current_depth + 1, &mut children, Some(id), count);
        }
        let mut body = commands.entity(id);
        //The initial star color will be for the actual light source, if it exists
        let mut star_color = WHITE.into();
        if let Some(source) = &serialized_body.data.light_source {
            add_light_source(&mut body, source, &mut star_color, serialized_body, &scale, id);
            //This star color is for the imposter billboard
            star_color = Srgba::hex(&source.imposter_color).unwrap().into();
        }
        apply_body(BodyBundle::from_serialized(serialized_body.clone()), CreateBodyType::from_depth(current_depth), &assets, &mut body, &mut meshes, &mut sun_materials, calculate_hue(index as f32, total_count as f32), star_color, &scale);

        body.insert(BodyChildren(children));
        if let Some(parent) = parent {
            body.insert(BodyParent(parent));
        }
    }
}

fn add_light_source(
    entity: &mut EntityCommands,
    source: &SerializedLightSource,
    star_color: &mut Color,
    entry: &SerializedBody,
    scale: &SimulationScale,
    id: Entity
) {
    entity.with_children(|parent| {
        *star_color = Srgba::hex(&source.color).unwrap().into();
        parent.spawn(PointLight {
            color: *star_color,
            intensity: scale_lumen(source.intensity, &scale),
            shadows_enabled: true,
            range: scale.m_to_unit_32(source.range),
            radius: scale.m_to_unit_32(entry.data.ellipsoid.mean_equatorial_radius_km() as f32),
            ..default()
        })
            .insert(if source.enabled { Visibility::Visible } else { Visibility::Hidden })
            .insert(LightSource::new(id, source));
    });
}

pub fn calculate_hue(
    index: f32,
    total: f32
) -> f32 {
    360.0 * ((index + 1.) / total )
}

fn sort_bodies(
    bodies: &mut Vec<&SerializedBody>,
    offset: DVec3,
) {
    bodies.sort_by(|body1, body2| {
        let pos1 = DVec3::from(body1.data.starting_position) + offset;
        let pos2 = DVec3::from(body2.data.starting_position) + offset;
        pos1.length().partial_cmp(&pos2.length()).unwrap()
    });
}

pub fn apply_body(
    bundle: BodyBundle,
    body_type: CreateBodyType,
    assets: &Res<AssetServer>,
    entity: &mut EntityCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    sun_shader_materials: &mut ResMut<Assets<SunImposterMaterial>>,
    hue: f32,
    star_color: Color,
    scale: &SimulationScale
) {
    let asset_handle: Handle<Scene> = assets.load(bundle.model_path.clone().0);
    let color: Color = Hsva::new(hue, 1.0, 1.0, 1.0).into();
    entity.insert(bundle.clone());
    match body_type {
        CreateBodyType::Moon => {
            entity.insert(Moon);
        }
        CreateBodyType::Planet => {
            entity.insert(Planet);
        }
        CreateBodyType::Star => {
            entity.insert(Star {
                use_imposter: true,
            });
        }
    }
    entity.insert(OrbitSettings {
        color,
        ..default()
    });
    if body_type != CreateBodyType::Star {
        entity.insert(ApsisBody::default());
    }
    let mut scene_entity_id = None;
    let id = &entity.id();
    entity.with_children(|parent| {

        scene_entity_id = Some(spawn_scene(
            asset_handle.clone(),
            bundle.clone().name.as_str(),
            parent,
        ));

        spawn_billboard(
            bundle.clone(),
            color.into(),
            parent
        );

        if body_type == CreateBodyType::Star {
            spawn_imposter(
                bundle.clone(),
                parent,
                meshes,
                star_color.into(),
                *id,
                &scale,
                sun_shader_materials);
        }
    });
    entity.insert(SceneHandle(asset_handle.clone(), scene_entity_id.unwrap()));
}

fn spawn_imposter(
    bundle: BodyBundle,
    parent: &mut ChildSpawnerCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    color: Srgba,
    parent_id: Entity,
    scale: &SimulationScale,
    shader_material: &mut ResMut<Assets<SunImposterMaterial>>,
) {
    let color: Color = Srgba::rgb(color.red * 20., color.green * 20., color.blue * 20.).into();
    parent.spawn(Mesh3d(meshes.add(Circle::new(scale.m_to_unit_32(bundle.diameter.ellipsoid.mean_equatorial_radius_km() as f32 * 6000.)))))
        .insert(MeshMaterial3d(shader_material.add(SunImposterMaterial::with(color.into(), scale.m_to_unit_32(bundle.diameter.ellipsoid.mean_equatorial_radius_km() as f32)))))
        .insert(Visibility::Hidden)
        .insert(StarBillboard(parent_id))
        .insert(Name::new(format!("{} Imposter Billboard", bundle.name)));
}


fn spawn_billboard(
    bundle: BodyBundle,
    color: Color,
    parent: &mut ChildSpawnerCommands
) {
    parent.spawn(
        (
            BillboardText::from(bundle.name.as_str()),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Visibility::Visible,
            TextLayout::new_with_justify(JustifyText::Center),
            TextFont::from_font_size(60.0),
            TextColor(color)
        )

    )
        .insert(Name::new(format!("{} Text Billboard", bundle.name)));
}

pub fn spawn_scene(
    asset_handle: Handle<Scene>,
    name: &str,
    parent: &mut ChildSpawnerCommands,
) -> Entity {
    parent.spawn(SceneRoot::from(asset_handle))
        .insert(SceneEntity)
        .insert(Name::new(format!("{} Scene", name))).id()
}