use crate::simulation::components::body::{BodyChildren, BodyShape, Moon, OrbitSettings, Planet, Velocity};
use crate::simulation::components::scale::SimulationScale;
use crate::utils::sim_state_type_simulation;
use bevy::color::palettes::css;
use bevy::math::DVec3;
use bevy::prelude::{Camera, PostUpdate, Res};
use bevy::{app::{App, Plugin}, prelude::{Entity, Gizmos, IntoScheduleConfigs, Query, Transform, With}};
use bevy_panorbit_camera::PanOrbitCameraSystemSet;

pub struct DirectionPlugin;

impl Plugin for DirectionPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(PostUpdate, (display_force_and_velocity.after(PanOrbitCameraSystemSet)).run_if(sim_state_type_simulation));
    }
    
}

fn display_force_and_velocity(
    planet_query: Query<(&Transform, &BodyChildren, &OrbitSettings, &BodyShape, &Velocity), With<Planet>>,
    moon_query: Query<(Entity, &Transform, &OrbitSettings, &BodyShape, &Velocity), With<Moon>>,
    mut gizmos: Gizmos,
    scale: Res<SimulationScale>,
    camera: Query<&Transform, With<Camera>>
) {
    let cam = camera.single().unwrap();
    for (transform, _, orbit, shape, velocity) in &planet_query {
        let multiplier = multiplier(orbit, cam, transform, shape, &scale);
        if orbit.display_force {
            force_arrow(transform, orbit.force_direction, multiplier, &mut gizmos, &css::BLUE.into());
        }
        if orbit.display_velocity {
            gizmos.arrow(transform.translation, transform.translation +(velocity.0.normalize() * multiplier * orbit.arrow_scale as f64).as_vec3(), css::RED);
        }
    }
    for (entity, transform, orbit, shape, velocity) in &moon_query {
        let multiplier = multiplier(orbit, cam, transform, shape, &scale);
        if orbit.display_force {
            force_arrow(transform, orbit.force_direction, multiplier, &mut gizmos, &css::BLUE.into());
        }
        if orbit.display_velocity {
            if let Some((_, _, _, _, vel)) = planet_query.iter().find(|(_, ch, _, _, _)| ch.0.contains(&entity)) {
                velocity_arrow(transform, (velocity.0 - vel.0).normalize(), multiplier, &mut gizmos, &css::RED.into());
            }
        }
    }
}

fn multiplier(
    orbit: &OrbitSettings,
    cam: &Transform,
    transform: &Transform,
    shape: &BodyShape,
    scale: &SimulationScale
) -> f64 {
    let d = scale.m_to_unit(shape.ellipsoid.mean_equatorial_radius_km()) * 3.0;
    if orbit.auto_scale_arrows {
        f64::max(cam.translation.distance(transform.translation) as f64 / 6.0, d)
    } else {
        d * orbit.arrow_scale as f64
    }
}

fn force_arrow(
    transform: &Transform,
    direction: DVec3,
    scale: f64,
    gizmos: &mut Gizmos,
    color: &bevy::color::Color,
) {
    gizmos.arrow(transform.translation, transform.translation + (direction * scale).as_vec3(), *color);
}

fn velocity_arrow(
    transform: &Transform,
    direction: DVec3,
    scale: f64,
    gizmos: &mut Gizmos,
    color: &bevy::color::Color,
) {
    gizmos.arrow(transform.translation, transform.translation + (direction * scale).as_vec3(), *color);
}