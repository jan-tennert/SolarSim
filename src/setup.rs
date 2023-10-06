use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::core_pipeline::Skybox;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec3;
use bevy::pbr::{PointLight, PointLightBundle};
use bevy::prelude::{Camera3dBundle, Commands, default, OnEnter, Res, SceneBundle, SpatialBundle, Transform, Handle};
use bevy::scene::Scene;


use crate::bodies::Bodies;
use crate::SimState;
use crate::pan_orbit::lib::PanOrbitCamera;
use crate::skybox::Cubemap;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SimState::Simulation), (setup_planets, setup_camera));
    }
}

pub fn setup_planets(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let bodies = Bodies::all();
    for body in bodies {
        let mut entity = commands.spawn(SpatialBundle {
            ..default()
        });
        if body.light.enabled {
            entity.insert(PointLightBundle {
                point_light: PointLight {
                    intensity: body.light.intensity,
                    shadows_enabled: false,
                    range: body.light.range,
                    radius: body.light.radius,
                    ..default()
                },
                ..default()
            });
        }
        let asset_handle: Handle<Scene> = assets.load(body.model_path.clone().0);      
        
        entity.insert(body.clone());
        entity.insert(SceneBundle {
            scene: asset_handle,
            transform: Transform::from_scale(Vec3::splat(body.diameter.0)),
            ..Default::default()
        });
    }
}

pub fn setup_camera(    
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let skybox_handle = asset_server.load("textures/skybox.png");
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera {
            orbit_smoothness: 0.0,
            pan_smoothness: 0.0,
            zoom_smoothness: 0.0,
            ..default()
        },
        Skybox(skybox_handle.clone())
    ));
    
    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle,
    });
}