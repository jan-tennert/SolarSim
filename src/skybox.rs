use bevy::{
    asset::LoadState,
    prelude::*,
    render::
    render_resource::{
        TextureViewDescriptor, TextureViewDimension,
    },
};
use bevy::core_pipeline::Skybox;
use bevy::prelude::Plugin;

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {

    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Update, asset_loaded);
    }

}

#[derive(Resource)]
pub struct Cubemap {
    pub activated: bool,
    pub is_loaded: bool,
    pub image_handle: Handle<Image>,
}   

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if cubemap.activated && !cubemap.is_loaded
        && asset_server.get_load_state(cubemap.image_handle.clone_weak()) == Some(LoadState::Loaded)
    {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(
                image.texture_descriptor.size.height / image.texture_descriptor.size.width,
            );
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.0 = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}