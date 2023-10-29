use bevy::{app::{App, Plugin}, prelude::{BuildChildren, Color, Commands, default, Entity, in_state, IntoSystemConfigs, Label, NextState, NodeBundle, OnEnter, OnExit, Query, ResMut, Resource, TextBundle, Update}, text::TextStyle, ui::{AlignItems, JustifyContent, Node, Style, UiRect, Val}};

use crate::SimState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LoadingState>()
            .add_systems(OnEnter(SimState::Loading), spawn_loading)
            .add_systems(OnExit(SimState::Loading), despawn_loading)
            .add_systems(Update, (loading_system).run_if(in_state(SimState::Loading)));
    }
}

#[derive(Resource, Default)]
pub struct LoadingState {
    
    pub loaded_bodies: bool,
    pub scaled_bodies: bool

}

impl LoadingState {
    
    pub fn reset(&mut self) {
        self.loaded_bodies = false;
        self.scaled_bodies = false;
    }
    
    pub fn is_done(&self) -> bool {
        return self.loaded_bodies && self.scaled_bodies;
    }
    
}

fn despawn_loading(
    mut commands: Commands,
    nodes: Query<(Entity, &Node)>
) {
    for (entity, _) in &nodes {
        commands.entity(entity).despawn();
    }
}

fn spawn_loading(
    mut commands: Commands
) {
    let mut parent = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        });
    parent.with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Loading...",
                TextStyle {
                    //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::GRAY,
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            }),
            Label
        ));
    });
}

fn loading_system(
    loading_state: ResMut<LoadingState>,
    mut sim_state: ResMut<NextState<SimState>>,
) {
    if loading_state.is_done() {
        sim_state.set(SimState::Simulation)
    }
}