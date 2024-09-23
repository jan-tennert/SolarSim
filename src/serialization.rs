use bevy::asset::io::Reader;
use bevy::asset::AsyncReadExt;
use bevy::prelude::{Asset, AssetApp};
use bevy::{
    asset::{AssetLoader, LoadContext},
    math::DVec3,
    prelude::Plugin, reflect::{TypePath}, utils::BoxedFuture,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, TypePath, Asset, Clone)]
pub struct SimulationData {
    pub bodies: Vec<SerializedBody>,
    pub starting_time_millis: i64,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize, TypePath, Clone)]
pub struct SerializedBody {
    pub children: Vec<SerializedBody>,
    pub data: SerializedBodyData
}

#[derive(Debug, Deserialize, TypePath, Clone, Copy)]
pub struct SerializedVec {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl From<SerializedVec> for DVec3 {
    
    fn from(value: SerializedVec) -> Self {
        DVec3::new(value.x, value.y, value.z)
    }
    
}

#[derive(Debug, Deserialize, TypePath, Clone)]
pub struct SerializedBodyData {
    pub mass: f64,
    pub starting_position: SerializedVec,
    pub starting_velocity: SerializedVec,
    pub name: String,
    pub model_path: String,
    pub diameter: f64,
    pub rotation_speed: f64,
    pub axial_tilt: f32,
    pub simulate: bool,
    pub light_source: Option<SerializedLightSource>
}

#[derive(Debug, Deserialize, TypePath, Clone)]
pub struct SerializedLightSource {
    pub intensity: f64,
    pub range: f64,
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