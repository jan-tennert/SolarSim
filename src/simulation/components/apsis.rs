use bevy::{math::DVec3, prelude::{in_state, App, Component, Entity, IntoSystemConfigs, Plugin, Query, Reflect, Res, Update, With, Without}};
use crate::simulation::SimState;
use crate::simulation::components::body::{BodyChildren, Moon, Planet, SimPosition, Star};
use crate::simulation::components::physics::apply_physics;

pub struct ApsisPlugin;

impl Plugin for ApsisPlugin {

    fn build(&self, app: &mut App) {
        app
            .register_type::<Apsis>()
            .add_systems(Update, (update_apsis.after(apply_physics)).run_if(in_state(SimState::Loaded)));
    }

}

#[derive(Debug, Clone, Copy, Reflect, Default)]
pub struct Apsis {
    
    pub position: DVec3,
    pub distance: f32
    
}

#[derive(Component, Debug, Clone, Copy, Reflect, Default)]
pub struct ApsisBody {
    
    pub aphelion: Apsis,
    pub perihelion: Apsis,
    
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum ApsisType {
    Aphelion,
    Perihelion
}

fn update_apsis(
    stars: Query<(&SimPosition, &BodyChildren), (With<Star>, Without<Moon>, Without<Planet>)>,
    mut planets: Query<(Entity, &SimPosition, &mut ApsisBody, &BodyChildren), (With<Planet>, Without<Star>, Without<Moon>)>,
    mut moons: Query<(Entity, &SimPosition, &mut ApsisBody), (With<Moon>, Without<Star>, Without<Planet>)>,
) {
    for (entity, position, mut apsis, _) in &mut planets {
        let mut parent = None;
        for (s_pos, s_child) in &stars {
            if s_child.0.contains(&entity) {
                parent = Some(s_pos);
                break;
            }
        }
        if let Some(p_pos) = parent {
            let new_distance = p_pos.0.distance(position.0) as f32;
            //perihelion
            if apsis.perihelion.distance > new_distance || apsis.perihelion.distance == 0.0 {
                apsis.perihelion.distance = new_distance;
            }
            if apsis.aphelion.distance < new_distance || apsis.perihelion.distance == 0.0 {
                apsis.aphelion.distance = new_distance;
            } 
        }
    }
    for (entity, position, mut apsis) in &mut moons {
        let mut parent = None;
        for (_, s_pos, _, s_child) in &planets {
            if s_child.0.contains(&entity) {
                parent = Some(s_pos);
                break;
            }
        }
        if let Some(p_pos) = parent {
            let new_distance = p_pos.0.distance(position.0) as f32;
            //perihelion
            if apsis.perihelion.distance > new_distance || apsis.perihelion.distance == 0.0 {
                apsis.perihelion.distance = new_distance;
            }
            if apsis.aphelion.distance < new_distance || apsis.perihelion.distance == 0.0 {
                apsis.aphelion.distance = new_distance;
            } 
        }
    }
}