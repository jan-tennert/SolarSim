use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::Skybox;
use bevy::ecs::system::EntityCommands;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec3;
use bevy::pbr::{PbrBundle, PointLight, PointLightBundle};
use bevy::prelude::{Assets, Bundle, Camera, Camera3dBundle, Color, Commands, default, Entity, Handle, in_state, IntoSystemConfigs, Mesh, OnEnter, PerspectiveProjection, Projection, Res, ResMut, Resource, SceneBundle, shape, SpatialBundle, StandardMaterial, Startup, Transform, Update, Visibility};
use bevy::scene::Scene;
use bevy::text::{TextAlignment, TextSection, TextStyle};
use bevy_mod_billboard::{BillboardLockAxisBundle, BillboardTextBundle};

use crate::body::{BodyBundle, BodyChildren, Moon, OrbitSettings, Planet, Star};
use crate::camera::PanOrbitCamera;
use crate::constants::M_TO_UNIT;
use crate::loading::LoadingState;
use crate::selection::SelectedEntity;
use crate::serialization::SimulationData;
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
    let bodies = bodies_asset.get(&bodies_handle.handle);
    if bodies.cloned().is_none() {
        return;
    }
    let data = bodies.unwrap();
    starting_time.0 = data.starting_time_millis;
    let stars = data.bodies.iter().count();  
    
    //iterate through the stars
    for (s_index, entry) in data.bodies.iter().enumerate() {
        let mut star = commands.spawn(SpatialBundle::default());
        if selected_entity.entity.is_none() {
            selected_entity.change_entity(star.id());
        }
        let mut planets: Vec<Entity> = vec![];
        star.insert(PointLightBundle {
            point_light: PointLight {
                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                intensity: 15000000000.0,
                shadows_enabled: false,
                range: 300000000000.0,
                radius: 100.0,
                ..default()
            },
            ..default()
        });
        apply_body(BodyBundle::from(entry.clone()), Star::default(), &assets, &mut star, &mut meshes, &mut materials,360.0 * ((s_index + 1) as f32 / stars as f32), true);
        let planet_count = entry.children.iter().count();
        
        //iterate through the planets
        for (p_index, planet_entry) in entry.children.iter().enumerate() {
            let mut planet = star.commands().spawn(SpatialBundle::default());
            let mut moons: Vec<Entity> = vec![];            
            apply_body(BodyBundle::from(planet_entry.clone()), Planet, &assets, &mut planet, &mut meshes, &mut materials,360.0 * ((p_index + 1) as f32 / planet_count as f32), false);
            
            //for the tree-based ui later
            planets.push(planet.id());
            let moon_count = planet_entry.children.iter().count();
            
            //iterate through the moons
            for (m_index, moon_entry) in planet_entry.children.iter().enumerate() {
                let mut moon = planet.commands().spawn(SpatialBundle::default());
                
                //for the tree-based ui later                
                moons.push(moon.id());
                apply_body(BodyBundle::from(moon_entry.clone()), Moon, &assets, &mut moon, &mut meshes, &mut materials, 360.0 * ((m_index + 1) as f32 / moon_count as f32), false);
            } 
            planet.insert(BodyChildren(moons));
        }  
        star.insert(BodyChildren(planets));
  
    }
    bodies_handle.spawned = true;
    loading_state.loaded_bodies = true;
}

fn apply_body(
    bundle: BodyBundle,
    body_type: impl Bundle,
    assets: &Res<AssetServer>,
    entity: &mut EntityCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    hue: f32,
    add_billboard: bool,
) {
    let asset_handle: Handle<Scene> = assets.load(bundle.model_path.clone().0);
    let color = Color::hsl(hue, 1.0, 0.5);
    entity.insert(bundle.clone());
    entity.insert(body_type);
    entity.insert(OrbitSettings {
        color,
       ..default() 
    });
    entity.with_children(|parent| {
        parent.spawn(SceneBundle {
            scene: asset_handle,
            transform: Transform::default(),
            ..Default::default()
        });
    });
    entity.with_children(|parent| {
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
                ]).with_alignment(TextAlignment::Center),
                ..default()
            },
            ..default()
        });

        if add_billboard {
            parent.spawn(PbrBundle {
                mesh: meshes.add(shape::Circle::new(((bundle.diameter.num * M_TO_UNIT) as f32) * 3.0).into()),
                material: materials.add(Color::rgb(100.0, 100.0, 0.0).into()),
        //        transform: Transform::from_scale(Vec3::splat(((bundle.diameter.num * M_TO_UNIT)) as f32)),
                visibility: Visibility::Visible,
                ..default()
            })
                .insert(StarBillboard);
        }
    });
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
                hdr: false,
                ..default()
            },
            ..default()
        },
        PanOrbitCamera::default(),
        Skybox(skybox_handle.clone()),
        BloomSettings {
            intensity: 0.4, // the default is 0.3,
            ..default()
        }
    ));
    
    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle,
        activated: true,
    });
}