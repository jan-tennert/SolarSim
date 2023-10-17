use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::{TypePath, TypeUuid},
    utils::BoxedFuture, prelude::{Vec3, AddAsset, Plugin}, math::DVec3,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid, TypePath, Clone)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct SimulationData {
    pub bodies: Vec<SerializedBody>,
    pub starting_time_millis: i64
}

#[derive(Debug, Deserialize, TypeUuid, TypePath, Clone)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct SerializedBody {
    pub children: Vec<SerializedBody>,
    pub data: SerializedBodyData
}

#[derive(Debug, Deserialize, TypeUuid, TypePath, Clone)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
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

#[derive(Debug, Deserialize, TypeUuid, TypePath, Clone)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct SerializedBodyData {
    pub mass: f64,
    pub starting_position: SerializedVec,
    pub starting_velocity: SerializedVec,
    pub name: String,
    pub model_path: String,
    pub diameter: f64,
    pub rotation_speed: f64,
    pub axial_tilt: f32,
}

#[derive(Default)]
pub struct BodyAssetLoader;

impl AssetLoader for BodyAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = serde_json::from_str::<SimulationData>(std::str::from_utf8(bytes).unwrap())?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
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
            .add_asset::<SimulationData>();
    }

}