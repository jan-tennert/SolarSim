use crate::simulation::render::skybox::SkyboxPlugin;
use crate::simulation::render::star_billboard::StarBillboardPlugin;
use bevy::app::Plugin;

pub mod star_billboard;
pub mod skybox;

pub struct SimRenderPlugin;

impl Plugin for SimRenderPlugin {

    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(StarBillboardPlugin)
            .add_plugins(SkyboxPlugin);
    }

}