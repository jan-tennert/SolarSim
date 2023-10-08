use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, Query, Transform, Resource, Entity, With, ResMut, IntoSystemConfigs, Vec3};
use crate::SimState;
use crate::body::Mass;
use crate::pan_orbit::lib::PanOrbitCamera;
use crate::physics::update_position;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedEntity>();
      //      .add_systems(Update, (apply_camera_to_selection.after(update_position)).run_if(in_state(SimState::Simulation)));
    }

}

#[derive(Debug, Resource, Default)]
pub struct SelectedEntity(pub Option<Entity>);

pub fn apply_camera_to_selection(
    mut camera: Query<&mut PanOrbitCamera>,
    bodies: Query<(&Transform, With<Mass>)>,
    mut selected_entity: ResMut<SelectedEntity>
) {
    let mut camera = camera.single_mut();
    if let Some(entity) = selected_entity.0 {
        if let Ok((transform, _)) = bodies.get(entity) {
     //       camera.target_focus = Vec3::ZERO;
     //       camera.force_update = true;  
        } else {
            selected_entity.0 = None;
        }
    }
}