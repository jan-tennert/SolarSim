use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::core_pipeline::Skybox;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasPlugin;
use bevy::ecs::system::EntityCommands;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec3;
use bevy::pbr::{PointLight, PointLightBundle};
use bevy::prelude::{Camera3dBundle, Commands, default, OnEnter, Res, SceneBundle, SpatialBundle, Transform, Handle, Entity, Bundle, Projection, PerspectiveProjection, Startup, GizmoConfig, ResMut, Color, Msaa, Camera, StandardMaterial, Mesh, Assets, Material};
use bevy::scene::{Scene, SceneInstance};


use crate::bodies::Bodies;
use crate::SimState;
use crate::body::{BodyBundle, Star, Planet, Moon, BodyChildren};
use crate::pan_orbit::lib::PanOrbitCamera;
use crate::skybox::Cubemap;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .add_systems(OnEnter(SimState::Simulation), setup_planets);
    }
}

pub fn setup_planets(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let bodies = Bodies::all();
    for entry in bodies {
        let mut star = commands.spawn(SpatialBundle::default());
        let mut planets: Vec<Entity> = vec![];
        star.insert(PointLightBundle {
            point_light: PointLight {
                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                intensity: entry.bundle.light.intensity,
                shadows_enabled: false,
                range: entry.bundle.light.range,
                radius: entry.bundle.light.radius,
                ..default()
            },
            ..default()
        });
        apply_body(entry.bundle, Star, &assets, &mut star);
        for planet_entry in entry.children {
            let mut planet = star.commands().spawn(SpatialBundle::default());
            let mut moons: Vec<Entity> = vec![];            
            apply_body(planet_entry.bundle, Planet, &assets, &mut planet);
            planets.push(planet.id());
            for moon_entry in planet_entry.children {
                let mut moon = planet.commands().spawn(SpatialBundle::default());
                moons.push(moon.id());
                apply_body(moon_entry.bundle, Moon, &assets, &mut moon);
            } 
            planet.insert(BodyChildren(moons));
        }  
        star.insert(BodyChildren(planets));
  
    }
}

fn apply_body(
    bundle: BodyBundle,
    body_type: impl Bundle,
    assets: &Res<AssetServer>,
    entity: &mut EntityCommands 
) {
    let asset_handle: Handle<Scene> = assets.load(bundle.model_path.clone().0);      
    entity.insert(bundle.clone());
    entity.insert(body_type);
    entity.with_children(|child| {
        child.spawn(SceneBundle {
            scene: asset_handle,
            transform: Transform::from_scale(Vec3::splat(bundle.scale.0)).with_rotation(bundle.starting_rotation.0),
            ..Default::default()
        });
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
                near: 0.00001,
                ..default()
            }),
            camera: Camera {
                hdr: false,
                ..default()
            },
            ..default()
        },
        PanOrbitCamera {
            orbit_smoothness: 0.0,
            pan_smoothness: 0.0,
            zoom_smoothness: 0.0,
            ..default()
        },
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