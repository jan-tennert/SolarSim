use bevy::prelude::{App, Entity, Plugin, Resource, Res, Update, IntoSystemConfigs, in_state, Transform, Query, Camera, Vec3, Without};

use crate::{selection::{SelectedEntity, apply_camera_to_selection}, SimState, body::{BodyChildren, Mass}, pan_orbit::lib::pan_orbit_camera, physics::{update_position, apply_physics}};

pub struct LockOnPlugin;

impl Plugin for LockOnPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .init_resource::<LockOn>()
        .add_systems(Update, (lock_on.after(apply_physics)).run_if(in_state(SimState::Simulation)));
    }
    
}

#[derive(Resource, Default)]
pub struct LockOn(pub bool);

fn lock_on(
    lock_on: Res<LockOn>,
    mut query: Query<(Entity, &Transform, Option<&BodyChildren>, Without<Camera>)>,
    mut camera: Query<(&Camera, &mut Transform, Without<Mass>, Without<BodyChildren>)>,
    selected_entity: Res<SelectedEntity>
) {
    if !lock_on.0 {
        return;
    }
    if let Some(s_entity) = selected_entity.0 {
        let mut parent: Option<&Transform> = None;
        let mut selected: Option<&Transform> = None;
        for (entity, transform, children, _) in query.iter_mut() {
            if let Some(children) = children {
                if children.0.contains(&s_entity) {
                    parent = Some(transform);
                }
            } 
            if s_entity == entity {                
                selected = Some(transform);   
            }
        }
        if let Some(p_transform) = parent {
            if let Some(s_transform) = selected {
                let (_, mut c_transform, _, _) = camera.single_mut();     
                c_transform.look_at(p_transform.translation, Vec3::Y);
            }
        }
    }       
}