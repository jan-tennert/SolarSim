use crate::simulation::components::anise::retrieve_starting_data;
use crate::simulation::components::body::{BodyBundle, BodyChildren, BodyParent, SimPosition};
use crate::simulation::components::rotation::initial_rotation;
use crate::simulation::components::scale::SimulationScale;
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::components::shape::apply_real_diameter;
use crate::simulation::render::star_billboard::SunImposterMaterial;
use crate::simulation::scenario::save_scenario::save_scenario;
use crate::simulation::scenario::setup::scenario::apply_body;
use crate::simulation::SimState;
use crate::utils::sim_state_type_editor;
use bevy::app::{App, Plugin};
use bevy::color::palettes::css::WHITE;
use bevy::ecs::system::SystemId;
use bevy::prelude::{AssetServer, Assets, Commands, Entity, FromWorld, IntoSystemConfigs, Local, Mesh, OnEnter, Query, Res, ResMut, Resource, Transform, Update, Vec3, Visibility, World};
use std::collections::HashMap;

#[non_exhaustive]
pub struct EditorSystemType;

impl EditorSystemType {
    pub const UPDATE_POSITIONS: &'static str = "update_positions";
    pub const UPDATE_DIAMETER: &'static str = "update_diameter";
    pub const UPDATE_TILT: &'static str = "update_tilt";
    pub const CREATE_BODY: &'static str = "create_body";
    pub const SAVE_SCENARIO: &'static str = "save_scenario";
    pub const RETRIEVE_DATA: &'static str = "retrieve_data";
}

#[derive(Resource)]
pub struct EditorSystems(pub HashMap<String, SystemId>);

#[derive(Resource, Default, PartialOrd, PartialEq, Eq, Clone, Debug)]
pub struct CreateBodyState {
    pub parent: Option<Entity>,
    pub body_type: CreateBodyType,
}

#[derive(Default, Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum CreateBodyType {
    #[default]
    Moon,
    Planet,
    Star
}

impl CreateBodyType {

    pub fn from_depth(depth: usize) -> Self {
        match depth {
            0 => CreateBodyType::Star,
            1 => CreateBodyType::Planet,
            _ => CreateBodyType::Moon
        }
    }

}

impl FromWorld for EditorSystems {
    fn from_world(world: &mut World) -> Self {
        let mut systems = EditorSystems(HashMap::new());

        systems.0.insert(
            EditorSystemType::UPDATE_POSITIONS.into(),
            world.register_system(update_body_positions)
        );

        systems.0.insert(
            EditorSystemType::UPDATE_DIAMETER.into(),
            world.register_system(apply_real_diameter)
        );

        systems.0.insert(
            EditorSystemType::UPDATE_TILT.into(),
            world.register_system(initial_rotation)
        );

        systems.0.insert(
            EditorSystemType::CREATE_BODY.into(),
            world.register_system(create_empty_body)
        );

        systems.0.insert(
            EditorSystemType::SAVE_SCENARIO.into(),
            world.register_system(save_scenario)
        );

        systems.0.insert(
            EditorSystemType::RETRIEVE_DATA.into(),
            world.register_system(retrieve_starting_data)
        );

        systems
    }
}

pub struct EditorPlugin;

impl Plugin for EditorPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<CreateBodyState>()
            .init_resource::<EditorSystems>()
            .add_systems(OnEnter(SimState::Loaded), update_body_positions.run_if(sim_state_type_editor))
            .add_systems(Update, selection_listener.run_if(sim_state_type_editor))
        //      .init_state::<crate::simulation::SimState>()
        ;
    }

}

fn create_empty_body(
    mut selected_entity: ResMut<SelectedEntity>,
    mut commands: Commands,
    mut create_body_state: ResMut<CreateBodyState>,
    mut parent_query: Query<&mut BodyChildren>,
    mut assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SunImposterMaterial>>,
    mut index: Local<i32>,
    scale: Res<SimulationScale>
) {
    let mut entity_commands = commands.spawn((Transform::default(), Visibility::default()));
    apply_body(
        BodyBundle::empty(*index),
        create_body_state.body_type.clone(),
        &mut assets,
        &mut entity_commands,
        &mut meshes,
        &mut materials,
        0.0,
        WHITE.into(),
        &scale
    );
    if create_body_state.body_type != CreateBodyType::Moon {
        entity_commands.insert(BodyChildren(Vec::new()));
    }
    if let Some(parent) = create_body_state.parent {
        entity_commands.insert(BodyParent(parent));
        parent_query.get_mut(parent).unwrap().0.push(entity_commands.id());
    }
    selected_entity.entity = Some(entity_commands.id());
    create_body_state.parent = None;
    *index += 1;
}

fn selection_listener(
    selected_entity: Res<SelectedEntity>,
    mut local_selected_entity: Local<SelectedEntity>,
    mut commands: Commands,
    systems: Res<EditorSystems>
) {
    if local_selected_entity.entity.is_none() {
        local_selected_entity.entity = selected_entity.entity;
    }
    match selected_entity.entity.zip(local_selected_entity.entity) {
        None => {}
        Some((e1, e2)) => {
            if e1 != e2 {
                local_selected_entity.entity = selected_entity.entity;
                commands.run_system(systems.0[EditorSystemType::UPDATE_POSITIONS]);
            }
        }
    }
}

pub fn update_body_positions(
    mut bodies: Query<(Entity, &SimPosition, &mut Transform)>,
    selected_entity: Res<SelectedEntity>,
    scale: Res<SimulationScale>
) {
    let offset = if let Some(entity) = selected_entity.entity {
        if let Err(_) = bodies.get(entity) {
            return;
        } else {
            let (_, position, mut transform) = bodies.get_mut(entity).unwrap();
            transform.translation = Vec3::ZERO;
            scale.m_to_unit_dvec(position.current).as_vec3()
        }
    } else {
        Vec3::ZERO
    };
    for (_, position, mut transform) in bodies.iter_mut() {
        transform.translation = scale.m_to_unit_dvec(position.current).as_vec3() - offset;
    }
}
