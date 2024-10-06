use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use bevy::prelude::{Children, in_state, IntoSystemConfigs, Query, Res, Resource, Transform, Update, Visibility, With, Without, Has, Name, Window, Camera, GlobalTransform, Vec2, Projection, PerspectiveProjection, Entity, Gizmos, Srgba};
use bevy::render::camera::CameraProjection;
use bevy::text::Text;
use bevy::utils::HashMap;
use bevy_mod_billboard::text::BillboardTextBounds;

use crate::simulation::components::apsis::ApsisBody;
use crate::simulation::components::body::{Diameter, Moon, Planet, Star, BillboardVisible, BodyParent};
use crate::simulation::components::camera::{pan_orbit_camera, PanOrbitCamera};
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::SimState;
use crate::simulation::render::star_billboard::STAR_IMPOSTER_DIVIDER;

const STAR_VISIBILITY_THRESHOLD: f32 = 40_000_000.0; //if the camera's radius is less than this, stars' names will be hidden
const PLANET_VISIBILITY_THRESHOLD: f32 = 1700.0; //if the camera's radius is less than this, planets' names will be hidden
//const MOON_VISIBILITY_THRESHOLD: f32 = 0.001; //if the camera's radius is less than this, moons' names will be hidden
const RADIUS_DIVIDER: f32 = 3700.0;
const TRANSLATION_MULTIPLIER: f32 = 2000.0;
const VISIBILITY_THRESHOLD: f32 = 20.;

pub struct BodyBillboardPlugin;

impl Plugin for BodyBillboardPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<BillboardSettings>()
            .add_systems(Update, (auto_scale_billboards.after(pan_orbit_camera)).run_if(in_state(SimState::Loaded)));
    }

}

#[derive(Resource)]
pub struct BillboardSettings {
    pub show: bool,
    pub dynamic_hide: bool,
}

impl Default for BillboardSettings {
    fn default() -> Self {
        Self { show: true, dynamic_hide: true }
    }
}

fn auto_scale_billboards(
    mut bodies: Query<(Entity, &Name, &Children, &Transform, &Diameter, &mut BillboardVisible, Option<&ApsisBody>, Has<Planet>, Has<Star>, Option<&BodyParent>), Without<Text>>,
    mut billboards: Query<(&Text, &mut Transform, &mut Visibility), With<BillboardTextBounds>>,
    camera: Query<(&Transform, &GlobalTransform, &Camera), (Without<BillboardTextBounds>, Without<Planet>, Without<Moon>, Without<Star>)>,
    settings: Res<BillboardSettings>,
    scale: Res<SimulationScale>,
    mut gizmos: Gizmos,
) {
    if !settings.show {
        for (_, _, mut visible) in billboards.iter_mut() {
            *visible = Visibility::Hidden;
        }
        return;
    }
    let (c_transform, global_trans, cam) = camera.single();
    let mut parent_pos = HashMap::default();
    for (entity, n, _, transform, _, _, _, _, _, p) in &mut bodies {
        parent_pos.insert(entity, transform.translation.clone());
    }
    for (_, name, children, p_transform, diameter, mut billboard_visible, apsis, planet, star, p) in bodies.iter_mut() {
        let mut predicate = true;
        if p.is_some() {
            let parent_transform = parent_pos.get(&p.unwrap().0).unwrap_or(&Vec3::ZERO);
            let distance_to_parent = calculate_screen_distance(&p_transform.translation, &parent_transform, &cam, &global_trans);
            if distance_to_parent < VISIBILITY_THRESHOLD {
                predicate = false;
            }
        }
        let distance_to_cam = c_transform.translation.distance(p_transform.translation) / STAR_IMPOSTER_DIVIDER;
        let offset = if star {
            distance_to_cam
        } else {
            diameter.num * scale.0
        };
        billboard_visible.0 = !settings.dynamic_hide || predicate;
        billboard(
            &mut billboards,
            c_transform,
            p_transform,
            offset,
            children,
            !settings.dynamic_hide || predicate,
            &mut gizmos
        )
    }
}

fn billboard(
    billboards: &mut Query<(&Text, &mut Transform, &mut Visibility), With<BillboardTextBounds>>,
    c_transform: &Transform,
    p_transform: &Transform,
    offset: f32,
    children: &Children,
    predicate: bool,
    x: &mut Gizmos
) {
    for child in children.iter() {
        if let Ok((_, mut transform, mut visible)) = billboards.get_mut(*child) {
            if predicate {
                apply_billboard(*c_transform, *p_transform, &mut transform, offset, x);
                *visible = Visibility::Visible;
            } else {
                *visible = Visibility::Hidden;
            }
        }
    }
}

fn apply_billboard(
    camera: Transform, //camera transform
    body: Transform, //body transform
    b_transform: &mut Transform, //billboard transform
    multiplier: f32, //This is the diameter of the body
    x: &mut Gizmos,
) {
    let direction = (body.translation - camera.translation).normalize();
    let cam_up = camera.rotation * Vec3::Y;
    let cam_right = cam_up.cross(direction).normalize();
    let orthogonal = direction.cross(cam_right).normalize();
    let cam_distance = camera.translation.distance(body.translation);
    b_transform.scale = body.scale.recip() * (cam_distance / RADIUS_DIVIDER);
    b_transform.translation = orthogonal * multiplier; //just extend the orthogonal vector by a constant
  //  x.line(body.translation, body.translation+orthogonal * multiplier, Srgba::RED);
}

fn calculate_screen_distance(
    object1: &Vec3,
    object2: &Vec3,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> f32 {
    // Convert 3D positions to 2D screen coordinates
    let screen_pos1 = camera.world_to_viewport(camera_transform, *object1).unwrap_or(Vec2::ZERO);
    let screen_pos2 = camera.world_to_viewport(camera_transform, *object2).unwrap_or(Vec2::ZERO);

    // Calculate the distance between the two points in 2D screen space
    (screen_pos1 - screen_pos2).length()
}