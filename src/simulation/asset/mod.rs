use bevy::asset::io::AssetSourceId;
use bevy::asset::AssetPath;
use std::path::Path;

pub mod serialization;
mod default_values;

pub const SCENARIO_ASSET_SOURCE: &str = "scenarios";

pub fn from_scenario_source(path: &str) -> AssetPath {
    let path = Path::new(path);
    let source = AssetSourceId::from(SCENARIO_ASSET_SOURCE);
    AssetPath::from_path(path).with_source(source)
}