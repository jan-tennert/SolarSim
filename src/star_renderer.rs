use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{Camera, Children, Component, in_state, IntoSystemConfigs, Parent, Query, Transform, Visibility, With, Without};
use bevy::scene::SceneInstance;

use crate::body::Star;
use crate::SimState;
use crate::camera::pan_orbit_camera;

const STAR_IMPOSTER_THRESHOLD: f32 = 4_000.0;
pub const STAR_IMPOSTER_DIVIDER: f32 = 10000.0;

pub struct StarRendererPlugin;

impl Plugin for StarRendererPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (change_sun_renderer.after(pan_orbit_camera)).run_if(in_state(SimState::Simulation)));
    }

}

#[derive(Component, Debug, Default)]
pub struct StarBillboard;

fn change_sun_renderer(
    camera: Query<(&Transform, &Camera, Without<Star>, Without<StarBillboard>)>,
    mut stars: Query<(&Transform, &Children, Without<Camera>, Without<StarBillboard>)>,
    mut star_billboards: Query<(&mut Transform, &mut Visibility, &Parent, With<StarBillboard>, Without<Camera>, Without<Star>)>,
    mut scenes: Query<(&SceneInstance, &mut Visibility, Without<StarBillboard>, Without<Star>)>,
) {
    let (c_transform, camera, _, _) = camera.single();
    for (transform, children, _, _) in &mut stars {
        let distance = c_transform.translation.distance(transform.translation);
        for child in children.iter() {
            if let Ok((_, mut visibility, _ , _)) = scenes.get_mut(*child) {
                if distance > STAR_IMPOSTER_THRESHOLD && camera.hdr {
                    *visibility = Visibility::Hidden;
                } else {
                    *visibility = Visibility::Visible;
                }
            }
            if let Ok((_, mut visibility, _, _, _, _)) = star_billboards.get_mut(*child) {
                if distance > STAR_IMPOSTER_THRESHOLD && camera.hdr {
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }

    for (mut b_transform, _, parent, _, _, _) in &mut star_billboards {
        let (transform, _, _, _) = stars.get(**parent).unwrap();
        let distance = c_transform.translation.distance(transform.translation);
        b_transform.look_at(-c_transform.translation, Vec3::Y);
        b_transform.scale = Vec3::splat(distance / STAR_IMPOSTER_DIVIDER);
    }
}