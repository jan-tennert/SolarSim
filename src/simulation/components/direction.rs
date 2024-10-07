use crate::simulation::components::body::{BodyChildren, BodyShape, Moon, OrbitSettings, Planet, Velocity};
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::SimState;
use bevy::color::palettes::css;
use bevy::prelude::Res;
use bevy::{app::{App, Plugin}, prelude::{in_state, Entity, Gizmos, IntoSystemConfigs, Query, Transform, Update, With}};
use bevy_panorbit_camera::PanOrbitCameraSystemSet;

pub struct DirectionPlugin;

impl Plugin for DirectionPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (display_force_and_velocity.after(PanOrbitCameraSystemSet)).run_if(in_state(SimState::Loaded)));
    }
    
}

fn display_force_and_velocity(
    planet_query: Query<(&Transform, &BodyChildren, &OrbitSettings, &BodyShape, &Velocity), With<Planet>>,
    moon_query: Query<(Entity, &Transform, &OrbitSettings, &BodyShape, &Velocity), With<Moon>>,
    mut gizmos: Gizmos,
    scale: Res<SimulationScale>
) {
    for (transform, _, orbit, shape, velocity) in &planet_query {
        let d = scale.m_to_unit(shape.ellipsoid.mean_equatorial_radius_km());
        if orbit.display_force {
            gizmos.arrow(transform.translation, transform.translation + (orbit.force_direction * d * orbit.arrow_scale as f64).as_vec3(), css::BLUE);
        }
        if orbit.display_velocity {
            gizmos.arrow(transform.translation, transform.translation +(velocity.0.normalize() * d* orbit.arrow_scale as f64).as_vec3(), css::RED);
        }
    }
    for (entity, transform, orbit, diameter, velocity) in &moon_query {
        let d = scale.m_to_unit(diameter.ellipsoid.mean_equatorial_radius_km());
        if orbit.display_force {
            gizmos.arrow(transform.translation, transform.translation +(orbit.force_direction * d * orbit.arrow_scale as f64).as_vec3(), css::BLUE);
        }
        if orbit.display_velocity {
            if let Some((_, _, _, _, vel)) = planet_query.iter().find(|(_, ch, _, _, _)| ch.0.contains(&entity)) {
                gizmos.arrow(transform.translation, transform.translation +((velocity.0 - vel.0).normalize() * d * orbit.arrow_scale as f64).as_vec3(), css::RED);
            }
        }
    }
}