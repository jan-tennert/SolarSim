use crate::simulation::asset::serialization::{SerializedBody, SimulationData};
use crate::simulation::components::apsis::ApsisBody;
use crate::simulation::components::body::{BodyBundle, BodyChildren, BodyParent, LightSource, Moon, OrbitSettings, Planet, SceneEntity, SceneHandle, Star};
use crate::simulation::components::editor::CreateBodyType;
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::components::selection::{SelectedEntity, SELECTION_MULTIPLIER};
use crate::simulation::render::skybox::Cubemap;
use crate::simulation::render::star_billboard::{StarBillboard, SunImposterMaterial};
use crate::simulation::scenario::loading::LoadingState;
use crate::simulation::ui::scenario_selection::SelectedScenario;
use crate::simulation::units::converter::scale_lumen;
use crate::simulation::SimState;
use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::color::palettes::css::WHITE;
use bevy::core::Name;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::Skybox;
use bevy::ecs::system::EntityCommands;
use bevy::hierarchy::BuildChildren;
use bevy::math::{DVec3, Vec3};
use bevy::pbr::{PointLight, PointLightBundle};
use bevy::prelude::{default, in_state, Assets, Camera, Camera3dBundle, ChildBuilder, Circle, Color, Commands, Entity, Handle, Hsva, IntoSystemConfigs, MaterialMeshBundle, Mesh, NextState, PerspectiveProjection, Projection, Query, Res, ResMut, Resource, SceneBundle, SpatialBundle, Srgba, StandardMaterial, Startup, Transform, Update, Visibility};
use bevy::render::view::{GpuCulling, NoCpuCulling};
use bevy::scene::Scene;
use bevy::text::{JustifyText, TextSection, TextStyle};
use bevy_mod_billboard::BillboardTextBundle;
use bevy_panorbit_camera::PanOrbitCamera;
use std::collections::HashMap;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ScenarioData>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, setup_scenario.run_if(in_state(SimState::Loading)));
    }
}

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
    mut standard_material: ResMut<Assets<StandardMaterial>>,
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
    let stars = data.bodies.iter().count();
    total_count += stars;

    //iterate through the stars
    for (s_index, entry) in data.bodies.iter().enumerate() {
        if !entry.data.simulate {
            continue;
        }
        let mut star = commands.spawn(SpatialBundle::default());
        let star_id = star.id();
        if selected_entity.entity.is_none() {
            selected_entity.change_entity(star_id, false);
        }
        
        //planets vector for adding BodyChildren later
        let mut planets: Vec<Entity> = vec![];
        let mut star_color = WHITE.into();
        if let Some(source) = &entry.data.light_source {
            star.with_children(|parent| {
                star_color = Srgba::hex(&source.color).unwrap().into();
                parent.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: star_color,
                        intensity: scale_lumen(source.intensity, &scale),
                        shadows_enabled: true,
                        range: scale.m_to_unit_32(source.range),
                        radius: scale.m_to_unit_32(entry.data.ellipsoid.mean_equatorial_radius_km() as f32),
                        ..default()
                    },
                    visibility: if source.enabled { Visibility::Visible } else { Visibility::Hidden },
                    ..default()
                }).insert(LightSource::new(star_id, source));
            });
        }
        star_color = entry.clone().data.light_source.map(|source| Srgba::hex(&source.imposter_color).unwrap().into()).unwrap_or(star_color);
        //add the star's components
        apply_body(BodyBundle::from(entry.clone()), CreateBodyType::Star, &assets, &mut star, &mut meshes, &mut sun_materials, calculate_hue(s_index as f32, stars as f32), star_color, &scale);
        
        //planet count in star system for coloring later
        let planet_count = entry.children.iter().filter(|p| p.data.simulate).count();
        total_count += planet_count;
        
        //collect the planets in a new vector and sort them by the length of the position
        let mut star_children = entry.children.iter().collect::<Vec<_>>();
        sort_bodies(&mut star_children, DVec3::ZERO);

        //iterate through the planets
        for (p_index, planet_entry) in star_children.iter().enumerate() {
            if !planet_entry.data.simulate {
                continue;
            }
            let mut star_commands = star.commands();
            let mut planet = star_commands.spawn(SpatialBundle::default());
            let planet_id = planet.id();
            
            //moon vector for adding BodyChildren later
            let mut moons: Vec<Entity> = vec![];
            
            //dereferenced planet entry (rust wants this in a new variable for some reason)
            let de_planet_entry = *planet_entry;
            
            //add the planet's components
            apply_body(BodyBundle::from(de_planet_entry.clone()), CreateBodyType::Planet, &assets, &mut planet, &mut meshes, &mut sun_materials, calculate_hue(p_index as f32, planet_count as f32), WHITE.into(), &scale);
            //for the tree-based ui later
            planets.push(planet_id);
            
            //moon count for coloring later
            let moon_count = planet_entry.children.iter().filter(|m| m.data.simulate).count();
            total_count += moon_count;
                
            //collect the moons in a new vector and sort them by the distance to the parent
            let mut planet_children = de_planet_entry.children.iter().collect::<Vec<_>>();
            sort_bodies(&mut planet_children, -DVec3::from(de_planet_entry.clone().data.starting_position));
            
            //iterate through the moons
            for (m_index, moon_entry) in planet_entry.children.iter().enumerate() {
                if !moon_entry.data.simulate {
                    continue;
                }
                let mut planet_commands = planet.commands();
                let mut moon = planet_commands.spawn(SpatialBundle::default());

                //for the tree-based ui later                
                moons.push(moon.id());
                
                //add the moon's components
                apply_body(BodyBundle::from(moon_entry.clone()), CreateBodyType::Moon, &assets, &mut moon, &mut meshes, &mut sun_materials, calculate_hue(m_index as f32, moon_count as f32), WHITE.into(), &scale);
                moon.insert(BodyParent(planet_id));
            }
            planet.insert(BodyParent(star_id));
            planet.insert(BodyChildren(moons));
        }  
        star.insert(BodyChildren(planets));
    }
    selected_scenario.spawned = true;
    loading_state.spawned_bodies = true;
    loading_state.total_bodies = total_count as i32;
    if stars == 0 {
        sim_state.set(SimState::Loaded);
    } else {
        let mut cam = cam.single_mut();
        let star = data.bodies.first().unwrap();
        cam.target_radius = scale.m_to_unit_32(star.data.ellipsoid.mean_equatorial_radius_km() as f32 * 2000. * SELECTION_MULTIPLIER);
    }
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
        let pos1 = DVec3::from(body1.data.starting_position) +offset;
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
                star_color,
                *id,
                &scale,
                sun_shader_materials,
                assets
            );
        }
    });
    entity.insert(SceneHandle(asset_handle.clone(), scene_entity_id.unwrap()));
}

fn spawn_imposter(
    bundle: BodyBundle,
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    color: Color,
    parent_id: Entity,
    scale: &SimulationScale,
    shader_material: &mut ResMut<Assets<SunImposterMaterial>>,
    assets: &Res<AssetServer>,
) {
    let srgba = Srgba::from(color);
    let color: Color = Srgba::rgb(srgba.red * 20., srgba.green * 20., srgba.blue * 20.).into();
    parent.spawn(MaterialMeshBundle {
        mesh: meshes.add(Circle::new(scale.m_to_unit_32(bundle.diameter.ellipsoid.mean_equatorial_radius_km() as f32 * 6000.))),
        material: shader_material.add(SunImposterMaterial::with(color.into(), scale.m_to_unit_32(bundle.diameter.ellipsoid.mean_equatorial_radius_km() as f32))),
        visibility: Visibility::Hidden,
        ..default()
    })
        .insert(StarBillboard(parent_id))
        .insert(Name::new(format!("{} Imposter Billboard", bundle.name)));
}

pub fn spawn_scene(
    asset_handle: Handle<Scene>,
    name: &str,
    parent: &mut ChildBuilder,
) -> Entity {
    parent.spawn(SceneBundle {
        scene: asset_handle,
        transform: Transform::default(),
        ..Default::default()
    })
        .insert(SceneEntity)
        .insert(Name::new(format!("{} Scene", name))).id()
}

fn spawn_billboard(
    bundle: BodyBundle,
    color: Color,
    parent: &mut ChildBuilder
) {
    parent.spawn(BillboardTextBundle {
        text: bevy::text::Text::from_sections([
            TextSection {
                value: bundle.name.to_string(),
                style: TextStyle {
                    font_size: 60.0,
                    // font: fira_sans_regular_handle.clone(),
                    color,
                    ..default()
                }
            }
        ]).with_justify(JustifyText::Center),
        ..default()
    })
        .insert(Name::new(format!("{} Text Billboard", bundle.name)));
}

pub fn setup_camera(    
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let skybox_handle = asset_server.load("textures/skybox.png");
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            projection: Projection::Perspective(PerspectiveProjection {
                near: 0.00000001,
                ..default()
            }),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        PanOrbitCamera {
            zoom_lower_limit: Some(0.02),
            ..default()
        },
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
        },
        BloomSettings {
            intensity: 0.3, // the default is 0.3,
            ..default()
        },
        GpuCulling,
        NoCpuCulling
    ));
    
    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle,
        activated: true,
    });
}