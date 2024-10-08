use crate::constants::DEF_M_TO_UNIT;
use crate::simulation::components::body::Star;
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::SimState;
use bevy::app::{App, Plugin, Update};
use bevy::asset::Asset;
use bevy::math::Vec3;
use bevy::prelude::{in_state, AlphaMode, Camera, Children, Component, Entity, Handle, Image, IntoSystemConfigs, LinearRgba, Material, MaterialPlugin, Parent, Query, Res, Transform, TypePath, Visibility, With, Without};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::scene::SceneInstance;

const STAR_IMPOSTER_THRESHOLD: f32 = 4_000.0;
pub const STAR_IMPOSTER_DIVIDER: f32 = 10000.0;

pub struct StarBillboardPlugin;

impl Plugin for StarBillboardPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<SunImposterMaterial>::default())
            .add_systems(Update, (change_sun_renderer/*.after(pan_orbit_camera)*/).run_if(in_state(SimState::Loaded)));
    }

}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SunImposterMaterial {
    #[uniform(0)]
    pub(crate) color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    pub(crate) color_texture: Option<Handle<Image>>,
    #[uniform(3)]
    pub(crate) glow_intensity: f32,
    #[uniform(4)]
    pub(crate) glow_radius: f32,
    pub(crate) alpha_mode: AlphaMode,
}

impl SunImposterMaterial {

    pub fn with(color: LinearRgba, radius: f32) -> Self {
        Self {
            color,
            color_texture: None,
            glow_intensity: 70.0,
            glow_radius: radius,
            alpha_mode: AlphaMode::Blend,
        }
    }

}

impl Material for SunImposterMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/star.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Component, Debug)]
pub struct StarBillboard(pub Entity);

fn change_sun_renderer(
    camera: Query<(&Transform, &Camera), (Without<Star>, Without<StarBillboard>)>,
    mut stars: Query<(&Transform, &Children), (Without<Camera>, Without<StarBillboard>)>,
    mut star_billboards: Query<(&mut Transform, &mut Visibility, &Parent), (With<StarBillboard>, Without<Camera>, Without<Star>)>,
    mut scenes: Query<(&SceneInstance, &mut Visibility), (Without<StarBillboard>, Without<Star>)>,
    scale: Res<SimulationScale>
) {
    let (c_transform, camera) = camera.single();
    let multiplier = scale.0 / DEF_M_TO_UNIT as f32;
    let multiplier_sq = multiplier.powi(2);
    for (transform, children) in &mut stars {
        let distance = c_transform.translation.distance(transform.translation);
        for child in children.iter() {
            if let Ok((_, mut visibility)) = scenes.get_mut(*child) {
                if distance > STAR_IMPOSTER_THRESHOLD * multiplier && camera.hdr {
                    *visibility = Visibility::Hidden;
                } else {
                    *visibility = Visibility::Visible;
                }
            }
            if let Ok((_, mut visibility, _)) = star_billboards.get_mut(*child) {
                if distance > STAR_IMPOSTER_THRESHOLD * multiplier && camera.hdr {
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }

    for (mut b_transform, _, parent) in &mut star_billboards {
        let (transform, _) = stars.get(**parent).unwrap();
        let distance = c_transform.translation.distance(transform.translation);
        b_transform.look_at(-c_transform.translation, Vec3::Y);
        b_transform.scale = Vec3::splat(distance / STAR_IMPOSTER_DIVIDER / multiplier_sq);
    }
}