use crate::simulation::components::body::{BodyChildren, Moon, Planet, SimPosition, Star};
use crate::simulation::integration::{paused, SimulationStep};
use crate::utils::sim_state_type_simulation;
use bevy::prelude::{not, Transform};
use bevy::{math::DVec3, prelude::{App, Component, Entity, IntoSystemConfigs, Plugin, Query, Reflect, Update, With, Without}};

pub struct ApsisPlugin;

impl Plugin for ApsisPlugin {

    fn build(&self, app: &mut App) {
        app
            .register_type::<Apsis>()
            .add_systems(Update, (update_apsis.after(SimulationStep)).run_if(sim_state_type_simulation).run_if(not(paused)));
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
    mut planets: Query<(Entity, &SimPosition, &Transform, &mut ApsisBody, &BodyChildren), (With<Planet>, Without<Star>, Without<Moon>)>,
    mut moons: Query<(Entity, &SimPosition, &Transform, &mut ApsisBody), (With<Moon>, Without<Star>, Without<Planet>)>,
) {
    for (entity, position, tra, mut apsis, _) in &mut planets {
        let mut parent = None;
        for (s_pos, s_child) in &stars {
            if s_child.0.contains(&entity) {
                parent = Some(s_pos);
                break;
            }
        }
        if let Some(p_pos) = parent {
            let new_distance = p_pos.current.distance(position.current) as f32;
            //perihelion
            if apsis.perihelion.distance > new_distance || apsis.perihelion.distance == 0.0 {
                apsis.perihelion.distance = new_distance;
                apsis.perihelion.position = position.current;
            }
            if apsis.aphelion.distance < new_distance || apsis.perihelion.distance == 0.0 {
                apsis.aphelion.distance = new_distance;
                apsis.aphelion.position = position.current;
            }
        }
    }
    for (entity, position, tra, mut apsis) in &mut moons {
        let mut parent = None;
        for (_, s_pos, _, _, s_child) in &planets {
            if s_child.0.contains(&entity) {
                parent = Some(s_pos);
                break;
            }
        }
        if let Some(p_pos) = parent {
            let new_distance = p_pos.current.distance(position.current) as f32;
            //perihelion
            if apsis.perihelion.distance > new_distance || apsis.perihelion.distance == 0.0 {
                apsis.perihelion.distance = new_distance;
                apsis.perihelion.position = position.current;
            }
            if apsis.aphelion.distance < new_distance || apsis.perihelion.distance == 0.0 {
                apsis.aphelion.distance = new_distance;
                apsis.aphelion.position = position.current;
            }
        }
    }
}