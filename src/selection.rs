use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Entity, in_state, IntoSystemConfigs, Query, ResMut, Resource, Transform, With};

use crate::body::{Mass, Star};
use crate::SimState;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedEntity>()
            .add_systems(Update, (apply_camera_to_selection).run_if(in_state(SimState::Simulation)));
    }

}

#[derive(Debug, Resource, Default)]
pub struct SelectedEntity(pub Option<Entity>);

pub fn apply_camera_to_selection(
    bodies: Query<(Entity, &Transform, With<Mass>, Option<&Star>)>,
    mut selected_entity: ResMut<SelectedEntity>
) {
    if let Some(entity) = selected_entity.0 {
        if let Err(_) = bodies.get(entity) {
             selected_entity.0 = None;
        }
    } else {
        if let Some((entity, _, _, _)) = bodies.iter().find(|(_, _, _, maybe_star)| {
            maybe_star.is_some()
        }) {
            selected_entity.0 = Some(entity);
        }
    }
}