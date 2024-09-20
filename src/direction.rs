use bevy::{app::{App, Plugin}, prelude::{Color, Entity, Gizmos, in_state, IntoSystemConfigs, Query, Transform, Update, With}};
use bevy::color::palettes::css;
use crate::{body::{BodyChildren, Diameter, Moon, OrbitSettings, Planet, Velocity}, camera::pan_orbit_camera, SimState};

pub struct DirectionPlugin;

impl Plugin for DirectionPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (display_force_and_velocity.after(pan_orbit_camera)).run_if(in_state(SimState::Simulation)));
    }
    
}

fn display_force_and_velocity(
    planet_query: Query<(&Transform, &BodyChildren, &OrbitSettings, &Diameter, &Velocity), With<Planet>>,
    moon_query: Query<(Entity, &Transform, &OrbitSettings, &Diameter, &Velocity), With<Moon>>,
    mut gizmos: Gizmos
) {
    for (transform, _, orbit, diameter, velocity) in &planet_query {
        if orbit.display_force {
            gizmos.arrow(transform.translation, transform.translation + (orbit.force_direction * diameter.num as f64 * orbit.arrow_scale as f64).as_vec3(), css::BLUE);
        }
        if orbit.display_velocity {
            gizmos.arrow(transform.translation, transform.translation +(velocity.0.normalize() * diameter.num as f64 * orbit.arrow_scale as f64).as_vec3(), css::RED);
        }
    }
    for (entity, transform, orbit, diameter, velocity) in &moon_query {
        if orbit.display_force {
            gizmos.arrow(transform.translation, transform.translation +(orbit.force_direction * diameter.num as f64 * orbit.arrow_scale as f64).as_vec3(), css::BLUE);
        }
        if orbit.display_velocity {
            if let Some((_, _, _, _, vel)) = planet_query.iter().find(|(_, ch, _, _, _)| ch.0.contains(&entity)) {
                gizmos.arrow(transform.translation, transform.translation +((velocity.0 - vel.0).normalize() * diameter.num as f64 * orbit.arrow_scale as f64).as_vec3(), css::RED);
            }
        }
    }
}