use crate::simulation::render::skybox::Cubemap;
use bevy::asset::AssetServer;
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::Skybox;
use bevy::prelude::{default, Camera, Camera3d, Commands, PerspectiveProjection, Projection, Res};
use bevy::render::view::{GpuCulling, NoCpuCulling};
use bevy_panorbit_camera::PanOrbitCamera;

pub fn setup_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let skybox_handle = asset_server.load("textures/skybox.png");
    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            near: 0.00000001,
            ..default()
        }),
        PanOrbitCamera {
            zoom_lower_limit: 0.02,
            ..default()
        },
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
            ..default()
        },
        Bloom {
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