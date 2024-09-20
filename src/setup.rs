use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::color::Color::Hsla;
use bevy::core::Name;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::Skybox;
use bevy::ecs::system::EntityCommands;
use bevy::hierarchy::BuildChildren;
use bevy::math::{DVec3, Vec3};
use bevy::pbr::{PbrBundle, PointLight, PointLightBundle};
use bevy::prelude::{Assets, Bundle, Camera, Camera3dBundle, ChildBuilder, Color, Commands, default, Entity, Handle, in_state, IntoSystemConfigs, Mesh, OnEnter, PerspectiveProjection, Projection, Res, ResMut, Resource, SceneBundle, SpatialBundle, StandardMaterial, Startup, Transform, Update, Visibility, Circle, Srgba, Hsva};
use bevy::render::view::RenderLayers;
use bevy::scene::Scene;
use bevy::text::{JustifyText, TextSection, TextStyle};
use bevy_mod_billboard::{BillboardLockAxisBundle, BillboardTextBundle};

use crate::apsis::ApsisBody;
use crate::body::{BodyBundle, BodyChildren, BodyParent, Moon, OrbitSettings, Planet, SceneHandle, Star};
use crate::camera::PanOrbitCamera;
use crate::constants::M_TO_UNIT;
use crate::loading::LoadingState;
use crate::selection::SelectedEntity;
use crate::serialization::{SerializedBody, SerializedVec, SimulationData};
use crate::SimState;
use crate::skybox::Cubemap;
use crate::star_renderer::StarBillboard;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BodiesHandle>()
            .init_resource::<StartingTime>()
            .add_systems(Startup, setup_camera)
            .add_systems(OnEnter(SimState::Loading), load_bodies)
            .add_systems(Update, setup_planets.run_if(in_state(SimState::Loading)));
    }
}

#[derive(Resource, Default)]
pub struct BodiesHandle {
    
    handle: Handle<SimulationData>,
    pub spawned: bool
    
}

#[derive(Resource, Default)]
pub struct StartingTime(pub i64);

pub fn load_bodies(
    assets: Res<AssetServer>,
    mut bodies_handle: ResMut<BodiesHandle>
) {
  //  let bodies = Bodies::all();
    bodies_handle.handle = assets.load("bodies.sim");
}

pub fn setup_planets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut bodies_handle: ResMut<BodiesHandle>,
    bodies_asset: ResMut<Assets<SimulationData>>,
    mut starting_time: ResMut<StartingTime>,
    mut loading_state: ResMut<LoadingState>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if bodies_handle.spawned {
        return;
    }
    let mut total_count = 0;
    let bodies = bodies_asset.get(&bodies_handle.handle);
    if bodies.cloned().is_none() {
        return;
    }
    let data = bodies.unwrap();
    starting_time.0 = data.starting_time_millis;
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
       star.insert(PointLightBundle {
            point_light: PointLight {
                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                intensity: 15000000000000.0,
                shadows_enabled: true,
                range: 30000000000000000.0,
                radius: 1000000000000000.0,
                ..default()
            },
            ..default()
        });

        //add the star's components
        apply_body(BodyBundle::from(entry.clone()), Star::default(), &assets, &mut star, &mut meshes, &mut materials,360.0 * ((s_index + 1) as f32 / stars as f32), true);
        
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
            apply_body(BodyBundle::from(de_planet_entry.clone()), Planet, &assets, &mut planet, &mut meshes, &mut materials,360.0 * ((p_index + 1) as f32 / planet_count as f32), false);
            //for the tree-based ui later
            planets.push(planet_id);
            
            //moon count for coloring later
            let moon_count = planet_entry.children.iter().filter(|m| m.data.simulate).count();
            total_count += moon_count;
                
            //collect the moons in a new vector and sort them by the distance to the parent
            let mut planet_children = de_planet_entry.children.iter().collect::<Vec<_>>();
            sort_bodies(&mut planet_children, -serialized_vec_to_vec(de_planet_entry.clone().data.starting_position));
            
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
                apply_body(BodyBundle::from(moon_entry.clone()), Moon, &assets, &mut moon, &mut meshes, &mut materials, 360.0 * ((m_index + 1) as f32 / moon_count as f32), false);
                moon.insert(BodyParent(planet_id));
            }
            planet.insert(BodyParent(star_id));
            planet.insert(BodyChildren(moons));
        }  
        star.insert(BodyChildren(planets));
    }
    bodies_handle.spawned = true;
    loading_state.loaded_bodies = true;
    loading_state.total_bodies = total_count as i32;
}

fn sort_bodies(
    bodies: &mut Vec<&SerializedBody>,
    offset: DVec3,
) {
    bodies.sort_by(|body1, body2| {
        let pos1 = serialized_vec_to_vec(body1.data.starting_position) + offset;
        let pos2 = serialized_vec_to_vec(body2.data.starting_position) + offset;
        pos1.length().partial_cmp(&pos2.length()).unwrap()
    });
}

fn serialized_vec_to_vec(
    serialized_vec: SerializedVec
) -> DVec3 {
    DVec3::new(serialized_vec.x, serialized_vec.y, serialized_vec.z)
}

fn apply_body(
    bundle: BodyBundle,
    body_type: impl Bundle,
    assets: &Res<AssetServer>,
    entity: &mut EntityCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    hue: f32,
    is_star: bool,
) {
    let asset_handle: Handle<Scene> = assets.load(bundle.model_path.clone().0);
    let color: Color = Hsva::new(hue, 1.0, 0.5, 1.0).into();
    entity.insert(bundle.clone());
    entity.insert(body_type);
    entity.insert(OrbitSettings {
        color,
       ..default() 
    });
    if !is_star {
        entity.insert(ApsisBody::default());
    }
    entity.insert(SceneHandle(asset_handle.clone()));
    entity.with_children(|parent| {

        spawn_scene(
            asset_handle.clone(),
            bundle.clone(),
            parent
        );

        spawn_billboard(
            bundle.clone(),
            color.into(),
            parent
        );
        
        if is_star {
            spawn_imposter(
                bundle.clone(),
                parent,
                meshes,
                materials
            );
        }
    });
}

fn spawn_imposter(
    bundle: BodyBundle,
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    parent.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(bundle.diameter.num  * 3.0)),
        material: materials.add(Color::rgb(30.0, 30.0, 0.0)),
        visibility: Visibility::Hidden,
        ..default()
    })
        .insert(StarBillboard)
        .insert(Name::new(format!("{} Imposter Billboard", bundle.name)));
}

fn spawn_scene(
    asset_handle: Handle<Scene>,
    bundle: BodyBundle,
    parent: &mut ChildBuilder
) {
    parent.spawn(SceneBundle {
        scene: asset_handle,
        transform: Transform::default(),
        ..Default::default()
    })
        .insert(Name::new(format!("{} Scene", bundle.name)));
}

fn spawn_billboard(
    bundle: BodyBundle,
    color: Color,
    parent: &mut ChildBuilder
) {
    parent.spawn(BillboardLockAxisBundle {
        billboard_bundle: BillboardTextBundle {
            transform: Transform::from_translation(Vec3::new(0., 2000., 0.))
                .with_scale(Vec3::splat(8.5)),
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
        },
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
        PanOrbitCamera::default(),
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
        },
        BloomSettings {
            intensity: 0.3, // the default is 0.3,
            ..default()
        }
    ));
    
    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle,
        activated: true,
    });
}