use crate::constants::DEFAULT_SUB_STEPS;
use crate::simulation::components::body::{Acceleration, Mass, OrbitSettings, SimPosition, Velocity};
use crate::simulation::components::motion_line::OrbitOffset;
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::integration::euler::EulerIntegrationPlugin;
use crate::simulation::integration::verlet::VerletIntegrationPlugin;
use crate::utils::sim_state_type_simulation;
use bevy::app::App;
use bevy::diagnostic::{Diagnostic, DiagnosticPath, RegisterDiagnostic};
use bevy::math::{DVec3, Vec3};
use bevy::prelude::{not, AppExtStates, Entity, IntoSystemConfigs, Plugin, Query, Res, ResMut, Resource, States, SystemSet, Transform, Update};

mod euler;
mod verlet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimulationStep;

pub const NBODY_STEP_TIME: DiagnosticPath = DiagnosticPath::const_new("nbody_step_time");
pub const NBODY_TOTAL_TIME: DiagnosticPath = DiagnosticPath::const_new("nbody_total_time");
pub const NBODY_STEPS: DiagnosticPath = DiagnosticPath::const_new("nbody_steps");

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntegrationType {
    #[default]
    Verlet,
    Euler
}

impl IntegrationType {

    pub fn as_str(&self) -> String {
        match self {
            IntegrationType::Verlet => "Verlet".to_string(),
            IntegrationType::Euler => "Euler".to_string()
        }
    }

    pub fn all() -> Vec<IntegrationType> {
        vec![IntegrationType::Verlet, IntegrationType::Euler]
    }

}

#[derive(Resource, Default)]
pub struct Pause(pub bool);

#[derive(Resource)]
pub struct SubSteps(pub i32);

impl Default for SubSteps {
    fn default() -> Self {
        SubSteps(DEFAULT_SUB_STEPS)
    }
}

impl SubSteps {

    pub fn small_step_up(&mut self) {
        self.0 *= 2;
    }

    pub fn big_step_up(&mut self) {
        self.0 *= 10;
    }

    pub fn small_step_down(&mut self) {
        self.0 = std::cmp::max(self.0 / 2, 1);
    }

    pub fn big_step_down(&mut self) {
        self.0 = std::cmp::max(self.0 / 10, 1);
    }

}

pub struct IntegrationPlugin;

impl Plugin for IntegrationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<IntegrationType>()
            .init_resource::<Pause>()
            .init_resource::<SubSteps>()
            .register_type::<Velocity>()
            .register_type::<Acceleration>()
            .register_type::<Mass>()
            .register_type::<OrbitSettings>()
            .add_plugins(EulerIntegrationPlugin)
            .add_plugins(VerletIntegrationPlugin)
            .register_diagnostic(Diagnostic::new(NBODY_STEP_TIME).with_max_history_length(50))
            .register_diagnostic(Diagnostic::new(NBODY_TOTAL_TIME).with_max_history_length(50))
            .register_diagnostic(Diagnostic::new(NBODY_STEPS).with_max_history_length(50))
            .add_systems(Update, (change_selection_without_update).in_set(SimulationStep).run_if(sim_state_type_simulation).run_if(paused))
            .add_systems(Update, (update_positions_after_pos_update).in_set(SimulationStep).run_if(sim_state_type_simulation).run_if(not(paused)));
    }
}

fn change_selection_without_update(
    mut query: Query<(Entity, &mut SimPosition, &mut Transform)>,
    selected_entity: Res<SelectedEntity>,
    mut orbit_offset: ResMut<OrbitOffset>,
    scale: Res<SimulationScale>,
) {
    let offset = match selected_entity.entity { //if orbit_offset.enabled is true, we calculate the new position of the selected entity first and then move it to 0,0,0 and add the actual position to all other bodies
        Some(selected) => {
            if let Ok((_, sim_pos, mut transform)) = query.get_mut(selected) {
                let raw_translation = scale.m_to_unit_dvec(sim_pos.current);
                transform.translation = Vec3::ZERO; //the selected entity will always be at 0,0,0
                -raw_translation
            } else {
                DVec3::ZERO
            }
        }
        None => DVec3::ZERO,
    };
    if offset.as_vec3() == orbit_offset.value {
        return;
    }
    for (entity, sim_pos, mut transform) in query.iter_mut() {
        if let Some(s_entity) = selected_entity.entity {
            if s_entity == entity {
                continue;
            }
        }
        let pos_without_offset = scale.m_to_unit_dvec(sim_pos.current);
        transform.translation = (pos_without_offset + offset).as_vec3();
    }
    orbit_offset.value = offset.as_vec3();
}

fn update_positions_after_pos_update(
    mut query: Query<(Entity, &Mass, &mut Acceleration, &mut OrbitSettings, &mut Velocity, &mut SimPosition, &mut Transform)>,
    mut orbit_offset: ResMut<OrbitOffset>,
    selected_entity: Res<SelectedEntity>,
    scale: Res<SimulationScale>
) {
    let offset = match selected_entity.entity { //if orbit_offset.enabled is true, we calculate the new position of the selected entity first and then move it to 0,0,0 and add the actual position to all other bodies
        Some(selected) => {
            if let Ok((_, mass, acc, mut orbit_s, vel, sim_pos, mut transform)) = query.get_mut(selected) {
                if orbit_s.display_force {
                    orbit_s.force_direction = acc.0.normalize();
                }
                let raw_translation = scale.m_to_unit_dvec(sim_pos.current);
                transform.translation = Vec3::ZERO; //the selected entity will always be at 0,0,0
                -raw_translation
            } else {
                DVec3::ZERO
            }
        }
        None => DVec3::ZERO
    };
    for (entity, _, acc, mut orbit_s, vel, sim_pos, mut transform) in query.iter_mut() {
        if let Some(s_entity) = selected_entity.entity {
            if s_entity == entity {
                continue;
            }
        }
        if orbit_s.display_force {
            orbit_s.force_direction = acc.0.normalize();
        }
        let pos_without_offset = scale.m_to_unit_dvec(sim_pos.current);
        transform.translation = (pos_without_offset + offset).as_vec3(); //apply offset
    }
    orbit_offset.value = offset.as_vec3();
}

pub fn paused(
    res: Res<Pause>
) -> bool {
    res.0
}

