use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use bevy::prelude::{Children, in_state, IntoSystemConfigs, Query, Res, Resource, Transform, Update, Visibility, With, Without, Name};
use bevy::text::Text;
use bevy_mod_billboard::text::BillboardTextBounds;

use crate::body::{Diameter, Moon, Planet, Star};
use crate::camera::{pan_orbit_camera, PanOrbitCamera};
use crate::constants::M_TO_UNIT;
use crate::SimState;
use crate::star_renderer::STAR_IMPOSTER_DIVIDER;

const STAR_VISIBILITY_THRESHOLD: f32 = 40_000_000.0; //if the camera's radius is less than this, stars' names will be hidden
const PLANET_VISIBILITY_THRESHOLD: f32 = 1000.0; //if the camera's radius is less than this, planets' names will be hidden
const MOON_VISIBILITY_THRESHOLD: f32 = 10.0; //if the camera's radius is less than this, moons' names will be hidden
const RADIUS_DIVIDER: f32 = 3700.0;
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
    bodies: Query<(&Children, &Transform, &Diameter, Option<&Planet>, Option<&Star>, Without<Text>)>,
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
    for (children, p_transform, diameter, planet, star, _) in bodies.iter() {
        let distance_to_cam = c_transform.translation.distance(p_transform.translation) / STAR_IMPOSTER_DIVIDER;
        let is_planet = planet.is_some();
        let is_star = star.is_some();
        let predicate = if is_planet {
            radius > PLANET_VISIBILITY_THRESHOLD && radius < STAR_VISIBILITY_THRESHOLD
        } else if is_star {
            radius > STAR_VISIBILITY_THRESHOLD
        } else {
            radius < PLANET_VISIBILITY_THRESHOLD && radius > MOON_VISIBILITY_THRESHOLD
        };
        let offset = if is_star {
            distance_to_cam
        } else {
            distance_to_cam * 100.0 / (diameter.num * M_TO_UNIT) as f32
        };
        billboard(
            &mut billboards,
            c_transform,
            p_transform,
            radius,
            offset,
            children,
            predicate
        )
    }
}

fn billboard(
    billboards: &mut Query<(&Text, &mut Transform, &mut Visibility, With<BillboardTextBounds>)>,
    c_transform: &Transform,
    p_transform: &Transform,
    radius: f32,
    offset: f32,
    children: &Children,
    predicate: bool
) {
    for child in children.iter() {
        if let Ok((_, mut transform, mut visible, _)) = billboards.get_mut(*child) {
            if predicate {
                apply_billboard(*c_transform, radius, *p_transform, &mut transform, offset);
                *visible = Visibility::Visible;
            } else {
                *visible = Visibility::Hidden;
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