use bevy::prelude::{in_state, App, Camera, Entity, IntoSystemConfigs, Plugin, PreUpdate, Query, Res, Without};
use crate::simulation::components::body::BodyChildren;
use crate::simulation::components::body::BodyShape;
use crate::simulation::components::body::Mass;

use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::SimState;

use bevy_panorbit_camera::PanOrbitCamera;


pub struct MinZoomPlugin;

impl Plugin for MinZoomPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .add_systems(PreUpdate, (min_zoom).run_if(in_state(SimState::Loaded)));
    }
    
}

fn min_zoom(
    query: Query<(Entity, &BodyShape), Without<Camera>>,
    mut camera_query: Query<(&Camera, &mut PanOrbitCamera), (Without<Mass>, Without<BodyChildren>)>,
    selected_entity: Res<SelectedEntity>,
) {
    if let Some(s_entity) = selected_entity.entity {
        if let Ok((_, body_shape)) = query.get(s_entity) { // Attempt to fetch BodyShape for selected entity
            let (_,  mut pan_orbit) = camera_query.single_mut();
            
            pan_orbit.zoom_lower_limit = (body_shape.ellipsoid.polar_radius_km/10000.0) as f32;
        }
    }
}

