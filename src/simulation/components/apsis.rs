use std::collections::HashMap;
use bevy::{math::DVec3, prelude::{in_state, App, Component, Entity, IntoSystemConfigs, Plugin, Query, Reflect, Res, Update, With, Without}};
use bevy::prelude::{Transform, Vec3};
use crate::simulation::SimState;
use crate::simulation::components::body::{BodyChildren, BodyParent, Moon, Planet, SimPosition, Star, Velocity};
use crate::simulation::components::horizons::AniseMetadata;
use crate::simulation::components::physics::apply_physics;
use crate::simulation::scenario::setup::ScenarioData;
use crate::simulation::ui::SimTime;

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
            let new_distance = p_pos.0.distance(position.0) as f32;
            //perihelion
            if apsis.perihelion.distance > new_distance || apsis.perihelion.distance == 0.0 {
                apsis.perihelion.distance = new_distance;
                apsis.perihelion.position = position.0;
            }
            if apsis.aphelion.distance < new_distance || apsis.perihelion.distance == 0.0 {
                apsis.aphelion.distance = new_distance;
                apsis.aphelion.position = position.0;
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
            let new_distance = p_pos.0.distance(position.0) as f32;
            //perihelion
            if apsis.perihelion.distance > new_distance || apsis.perihelion.distance == 0.0 {
                apsis.perihelion.distance = new_distance;
                apsis.perihelion.position = position.0;
            }
            if apsis.aphelion.distance < new_distance || apsis.perihelion.distance == 0.0 {
                apsis.aphelion.distance = new_distance;
                apsis.aphelion.position = position.0;
            }
        }
    }
}