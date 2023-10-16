use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::{TypePath, TypeUuid},
    utils::BoxedFuture, prelude::{Vec3, AddAsset, Plugin},
};
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid, TypePath)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct SimulationBodies {
    pub bodies: Vec<SerializedBody>,
}

#[derive(Debug, Deserialize, TypeUuid, TypePath)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct SerializedBody {
    pub children: Vec<SerializedBody>,
    pub data: SerializedBodyData
}

#[derive(Debug, Deserialize, TypeUuid, TypePath)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct SerializedBodyData {
    pub mass: f64,
    pub starting_position: Vec3,
    pub starting_velocity: Vec3,
    pub name: String,
    pub model_path: String
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
            let custom_asset = ron::de::from_bytes::<SimulationBodies>(bytes)?;
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
            .init_asset_loader::<BodyAssetLoader>();
    }

}