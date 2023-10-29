use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use bevy::prelude::{Children, in_state, IntoSystemConfigs, Query, Res, Resource, Transform, Update, Visibility, With, Without};
use bevy::text::Text;
use bevy_mod_billboard::text::BillboardTextBounds;

use crate::body::{Moon, Planet, Star};
use crate::camera::{pan_orbit_camera, PanOrbitCamera};
use crate::SimState;
use crate::star_renderer::STAR_IMPOSTER_DIVIDER;

const STAR_VISIBILITY_THRESHOLD: f32 = 40_000_000.0; //if the camera's radius is less than this, stars' names will be hidden
const PLANET_VISIBILITY_THRESHOLD: f32 = 1000.0; //if the camera's radius is less than this, planets' names will be hidden
const MOON_VISIBILITY_THRESHOLD: f32 = 10.0; //if the camera's radius is less than this, moons' names will be hidden
const RADIUS_DIVIDER: f32 = 3200.0;
const TRANSLATION_MULTIPLIER: f32 = 2000.0;

pub struct BodyBillboardPlugin;

impl Plugin for BodyBillboardPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<BillboardSettings>()
            .add_systems(Update, ((auto_scale_billboards).chain().after(pan_orbit_camera)).run_if(in_state(SimState::Simulation)));
    }

}

#[derive(Resource)]
pub struct BillboardSettings {
    pub show: bool,
}

impl Default for BillboardSettings {
    fn default() -> Self {
        Self { show: true }
    }
}

fn auto_scale_billboards(
    planets: Query<(&Children, &Transform, With<Planet>, Without<Moon>, Without<Text>)>,
    moons: Query<(&Children, &Transform, With<Moon>, Without<Planet>, Without<Text>)>,
    stars: Query<(&Children, &Transform, With<Star>, Without<Text>)>,
    mut billboards: Query<(&Text, &mut Transform, &mut Visibility, With<BillboardTextBounds>)>,
    camera: Query<(&PanOrbitCamera, &Transform, Without<BillboardTextBounds>, Without<Planet>, Without<Moon>, Without<Star>)>,
    settings: Res<BillboardSettings>
) {
    if !settings.show {
        for (_, _, mut visible, _) in billboards.iter_mut() {
            *visible = Visibility::Hidden;
        }
        return;
    }
    let (cam, c_transform, _, _, _, _) = camera.single();
    let radius = cam.radius;
    for (children, p_transform, _, _, _) in planets.iter() {
        for child in children.iter() {
            if let Ok((_, mut transform, mut visible, _)) = billboards.get_mut(*child) {
                if radius > PLANET_VISIBILITY_THRESHOLD && radius < STAR_VISIBILITY_THRESHOLD {
                    apply_billboard(*c_transform, radius, *p_transform, &mut transform, 1.0);
                    *visible = Visibility::Visible;
                } else {
                    *visible = Visibility::Hidden;
                }
            }
        }
    }

    for (children, p_transform, _, _, _) in moons.iter() {
        for child in children.iter() {
            if let Ok((_, mut transform, mut visible, _)) = billboards.get_mut(*child) {
                if radius < PLANET_VISIBILITY_THRESHOLD && radius > MOON_VISIBILITY_THRESHOLD {
                    apply_billboard(*c_transform, radius, *p_transform, &mut transform, 1.0);
                    *visible = Visibility::Visible;
                } else {
                    *visible = Visibility::Hidden;
                }
            }
        }
    }

    for (children, p_transform, _, _) in stars.iter() {
        for child in children.iter() {
            if let Ok((_, mut transform, mut visible, _)) = billboards.get_mut(*child) {
                if radius > STAR_VISIBILITY_THRESHOLD {
                    let distance_to_cam = c_transform.translation.distance(p_transform.translation);
                    let offset = distance_to_cam / STAR_IMPOSTER_DIVIDER;
                    apply_billboard(*c_transform, radius, *p_transform, &mut transform, offset);
                    *visible = Visibility::Visible;
                } else {
                    *visible = Visibility::Hidden;
                }
            }
        }
    }
}

fn apply_billboard(
    camera: Transform,
    cam_radius: f32,
    body: Transform,
    b_transform: &mut Transform,
    multiplier: f32,
) {
    let direction = (body.translation - camera.translation).normalize();
    let cam_up = camera.rotation * Vec3::Y;
    let cam_right = cam_up.cross(direction).normalize();
    let orthogonal = direction.cross(cam_right).normalize();
    b_transform.scale = body.scale.recip() * (cam_radius / RADIUS_DIVIDER);
    b_transform.translation = orthogonal * TRANSLATION_MULTIPLIER * multiplier; //just extend the orthogonal vector by a constant
}