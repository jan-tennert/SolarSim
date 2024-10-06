use anise::structure::planetocentric::ellipsoid::Ellipsoid;
use bevy::asset::io::file::FileAssetReader;
use bevy::asset::io::{AssetSource, AssetSourceBuilder, AssetSourceId, Reader};
use bevy::asset::AsyncReadExt;
use bevy::prelude::{Asset, AssetApp, Mat3, Vec3};
use bevy::{
    asset::{AssetLoader, LoadContext},
    math::DVec3,
    prelude::Plugin, reflect::TypePath, utils::BoxedFuture,
};
use serde::{Deserialize, Serialize};
use crate::simulation::asset::default_values::*;

#[derive(Debug, Deserialize, Serialize, TypePath, Asset, Clone)]
pub struct SimulationData {
    pub bodies: Vec<SerializedBody>,
    #[serde(default = "default_spk")]
    pub data_sets: Vec<String>,
    pub starting_time_millis: i64,
    pub title: String,
    pub description: String,
    pub scale: f32,
    pub timestep: i32,
}

#[derive(Debug, Deserialize, Serialize, TypePath, Clone)]
pub struct SerializedBody {
    pub children: Vec<SerializedBody>,
    pub data: SerializedBodyData
}

#[derive(Debug, Deserialize, Serialize, TypePath, Clone, Copy)]
pub struct SerializedVec {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Debug, Deserialize, Serialize, TypePath, Clone, Copy)]
pub struct SerializedMat3 {
    pub x: SerializedVec,
    pub y: SerializedVec,
    pub z: SerializedVec
}

impl From<Mat3> for SerializedMat3 {

    fn from(value: Mat3) -> Self {
        SerializedMat3 {
            x: SerializedVec::from(value.x_axis.as_dvec3()),
            y: SerializedVec::from(value.y_axis.as_dvec3()),
            z: SerializedVec::from(value.z_axis.as_dvec3())
        }
    }

}

impl From<SerializedMat3> for Mat3 {

    fn from(value: SerializedMat3) -> Self {
        Mat3::from_cols(value.x.into(), value.y.into(), value.z.into())
    }

}

impl From<SerializedVec> for DVec3 {
    
    fn from(value: SerializedVec) -> Self {
        DVec3::new(value.x, value.y, value.z)
    }
    
}

impl From<SerializedVec> for Vec3 {

    fn from(value: SerializedVec) -> Self {
        DVec3::new(value.x, value.y, value.z).as_vec3()
    }

}


impl From<DVec3> for SerializedVec {

    fn from(value: DVec3) -> Self {
        SerializedVec {
            x: value.x,
            y: value.y,
            z: value.z
        }
    }

}

#[derive(Debug, Serialize, Deserialize, TypePath, Clone)]
pub struct SerializedBodyData {
    pub mass: f64,
    pub starting_position: SerializedVec,
    pub starting_velocity: SerializedVec,
    pub name: String,
    pub model_path: String,
    pub diameter: f64,
    pub rotation_speed: f64,
    pub simulate: bool,
    #[serde(default = "default_id")]
    pub naif_id: i32,
    #[serde(default = "default_frame")]
    pub fixed_body_frame: SerializedFixedBodyFrame,
    #[serde(default = "default_ellipsoid")]
    pub ellipsoid: Ellipsoid,
    pub light_source: Option<SerializedLightSource>,
    #[serde(default = "default_rot_matrix")]
    pub rotation_matrix: SerializedMat3
}

#[derive(Debug, Serialize, Deserialize, TypePath, Clone)]
pub struct SerializedFixedBodyFrame {
    pub target_id: i32,
    pub orientation_id: i32
}

#[derive(Debug, Serialize, Deserialize, TypePath, Clone)]
pub struct SerializedLightSource {
    pub intensity: f32,
    pub range: f32,
    pub color: String,
    pub enabled: bool
}

#[derive(Default)]
pub struct BodyAssetLoader;

impl AssetLoader for BodyAssetLoader {
    type Asset = SimulationData;
    type Settings = ();
    type Error = serde_json::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await.unwrap();
            let custom_asset = serde_json::from_str::<SimulationData>(std::str::from_utf8(&*bytes).unwrap())?;
            Ok((custom_asset))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["sim"]
    }
}

pub struct SerializationPlugin;

impl Plugin for SerializationPlugin {

    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_asset_loader::<BodyAssetLoader>()
            .init_asset::<SimulationData>();
    }

}