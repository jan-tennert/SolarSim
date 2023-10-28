use bevy::prelude::{App, Camera, Entity, in_state, IntoSystemConfigs, Plugin, Query, Res, Resource, Transform, Update, Vec3, Without};

use crate::{body::{BodyChildren, Mass}, camera::pan_orbit_camera, selection::SelectedEntity, SimState};

pub struct LockOnPlugin;

impl Plugin for LockOnPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .init_resource::<LockOn>()
        .add_systems(Update, (lock_on.before(pan_orbit_camera)).run_if(in_state(SimState::Simulation)));
    }
    
}

#[derive(Resource, Default)]
pub struct LockOn {
    
    pub enabled: bool,
    
}

fn lock_on(
    lock_on: Res<LockOn>,
    mut query: Query<(Entity, &Transform, Option<&BodyChildren>, Without<Camera>)>,
    mut camera: Query<(&Camera, &mut Transform, Without<Mass>, Without<BodyChildren>)>,
    selected_entity: Res<SelectedEntity>
) {
    if !lock_on.enabled {
        return;
    }
    if let Some(s_entity) = selected_entity.0 {
        let mut parent: Option<&Transform> = None;
        for (_, transform, children, _) in query.iter_mut() {
            if let Some(children) = children {
                if children.0.contains(&s_entity) {
                    parent = Some(transform);
                }
            } 
        }
        if let Some(p_transform) = parent {
            let (_, mut c_transform, _, _) = camera.single_mut();   
            c_transform.look_at(p_transform.translation, Vec3::X);
        }
    }       
}