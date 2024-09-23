use std::collections::HashMap;
use bevy::app::{App, Plugin};
use bevy::color::palettes::css::WHITE;
use bevy::ecs::observer::TriggerTargets;
use bevy::ecs::system::SystemId;
use bevy::prelude::{AssetServer, Assets, Bundle, Commands, Entity, FromWorld, IntoSystemConfigs, Local, Mesh, OnEnter, Query, Res, ResMut, Resource, SpatialBundle, StandardMaterial, Transform, Update, Vec3, World};
use crate::constants::M_TO_UNIT;
use crate::setup::apply_body;
use crate::simulation::components::body::{BodyBundle, BodyChildren, BodyParent, Moon, Planet, SimPosition};
use crate::simulation::components::diameter::apply_real_diameter;
use crate::simulation::components::rotation::axial_tilt;
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::SimState;
use crate::utils::sim_state_type_editor;

#[non_exhaustive]
pub struct EditorSystemType;

impl EditorSystemType {
    pub const UPDATE_POSITIONS: &'static str = "update_positions";
    pub const UPDATE_DIAMETER: &'static str = "update_diameter";
    pub const UPDATE_TILT: &'static str = "update_tilt";
    pub const CREATE_BODY: &'static str = "create_body";
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

impl FromWorld for EditorSystems {
    fn from_world(world: &mut World) -> Self {
        let mut my_item_systems = EditorSystems(HashMap::new());

        my_item_systems.0.insert(
            EditorSystemType::UPDATE_POSITIONS.into(),
            world.register_system(update_body_positions)
        );

        my_item_systems.0.insert(
            EditorSystemType::UPDATE_DIAMETER.into(),
            world.register_system(apply_real_diameter)
        );

        my_item_systems.0.insert(
            EditorSystemType::UPDATE_TILT.into(),
            world.register_system(axial_tilt)
        );

        my_item_systems.0.insert(
            EditorSystemType::CREATE_BODY.into(),
            world.register_system(create_empty_body)
        );

        my_item_systems
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut index: Local<i32>
) {
    let mut entity_commands = commands.spawn(SpatialBundle::default());
    apply_body(
        BodyBundle::empty(*index),
        create_body_state.body_type.clone(),
        &mut assets,
        &mut entity_commands,
        &mut meshes,
        &mut materials,
        0.0,
        WHITE.into()
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
    selected_entity: Res<SelectedEntity>
) {
    let offset = if let Some(entity) = selected_entity.entity {
        if let Err(_) = bodies.get(entity) {
            return;
        } else {
            let (_, position, mut transform) = bodies.get_mut(entity).unwrap();
            transform.translation = Vec3::ZERO;
            (position.0 * M_TO_UNIT).as_vec3()
        }
    } else {
        Vec3::ZERO
    };
    println!("Offset: {:?}", offset);
    for (_, position, mut transform) in bodies.iter_mut() {
        transform.translation = (position.0 * M_TO_UNIT).as_vec3() - offset;
    }
}
