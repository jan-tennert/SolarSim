use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Entity, in_state, IntoSystemConfigs, Query, Res, ResMut, Resource, Transform, With, Vec3};

use crate::simulation::components::body::{Diameter, Mass, Star};
use crate::simulation::components::camera::{pan_orbit_camera, PanOrbitCamera};
use crate::constants::M_TO_UNIT;
use crate::simulation::components::orbit_lines::OrbitOffset;
use crate::simulation::components::physics::apply_physics;
use crate::simulation::SimState;

const SELECTION_MULTIPLIER: f32 = 3.0;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedEntity>()
            .add_systems(Update, (apply_camera_to_selection.after(apply_physics).before(pan_orbit_camera)).run_if(in_state(SimState::Loaded)));
    }

}

#[derive(Debug, Resource, Default)]
pub struct SelectedEntity {

    pub entity: Option<Entity>,
    pub changed_focus: bool,

}

impl SelectedEntity {

    pub fn change_entity(&mut self, entity: Entity, ignore_focus: bool) {
        self.entity = Some(entity);
        self.changed_focus = ignore_focus;
    }

}

pub fn apply_camera_to_selection(
    bodies: Query<(Entity, &Transform, &Diameter, Option<&Star>), With<Mass>>,
    mut camera: Query<&mut PanOrbitCamera>,
    mut selected_entity: ResMut<SelectedEntity>,
    orbit_offset: Res<OrbitOffset>
) {
    if let Some(entity) = selected_entity.entity {
        if let Err(_) = bodies.get(entity) {
             selected_entity.entity = None;
        } else if !selected_entity.changed_focus {
            let (_, _, diameter, _) = bodies.get(entity).unwrap();
            let mut cam = camera.single_mut();            
            cam.radius = (diameter.num * SELECTION_MULTIPLIER * M_TO_UNIT as f32);
            if orbit_offset.enabled {
                cam.focus = Vec3::ZERO;        
            }
            selected_entity.changed_focus = true;
        }
        if !orbit_offset.enabled {
            let mut cam = camera.single_mut();
            cam.focus = bodies.get(entity).unwrap().1.translation;         
        }
    } else {
        if let Some((entity, _, _, _)) = bodies.iter().find(|(_, _, _, maybe_star)| {
            maybe_star.is_some()
        }) {
            selected_entity.change_entity(entity, false);
        }
    }
}