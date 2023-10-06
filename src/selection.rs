use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, Query, Transform, PostUpdate, Name};
use crate::body::Selectable;
use crate::SimState;
use crate::pan_orbit::lib::PanOrbitCamera;
use crate::physics::update_position;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (apply_camera_to_selection.after(update_position)).run_if(in_state(SimState::Simulation)));
    }

}

pub fn apply_camera_to_selection(
    mut camera: Query<&mut PanOrbitCamera>,
    selected_body: Query<(&Selectable, &Transform)>
) {
    let mut camera = camera.single_mut();
    for (selectable, transform) in selected_body.iter() {
        if selectable.0 {
            camera.target_focus = transform.translation;
            camera.force_update = true;  
        }
    }
}