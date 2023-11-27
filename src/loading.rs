use bevy::{app::{App, Plugin}, prelude::{BuildChildren, Res, Color, Commands, default, Entity, in_state, IntoSystemConfigs, Label, NextState, NodeBundle, OnEnter, OnExit, Query, ResMut, Resource, TextBundle, Update, AssetServer, Component, With}, text::{TextStyle, Text}, ui::{AlignItems, JustifyContent, Node, Style, UiRect, Val, UiImage, FlexDirection}};

use crate::SimState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LoadingState>()
            .add_systems(OnEnter(SimState::Loading), spawn_loading)
            .add_systems(OnExit(SimState::Loading), despawn_loading)
            .add_systems(Update, (loading_system, update_progress.before(loading_system)).run_if(in_state(SimState::Loading)));
    }
}

#[derive(Component, Default)]
pub struct ProgressMarker;

#[derive(Resource, Default)]
pub struct LoadingState {
    
    pub loaded_bodies: bool,
    pub scaled_bodies: bool,
    pub scaled_bodies_count: i32,
    pub total_bodies: i32,
    pub tilted_bodies: bool,

}

impl LoadingState {
    
    pub fn reset(&mut self) {
        self.loaded_bodies = false;
        self.scaled_bodies = false;
        self.tilted_bodies = false;
        self.scaled_bodies_count = 0;
        self.total_bodies = 0;
    }
    
    pub fn is_done(&self) -> bool {
        return self.loaded_bodies && self.scaled_bodies && self.tilted_bodies;
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
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let mut parent = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        });
    parent.insert(UiImage::new(asset_server.load("images/background.png")));
    parent.with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Loading...",
                TextStyle {
                    //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            }),
            Label
        ));
        parent.spawn((
            TextBundle::from_section(
                "Spawning bodies",
                TextStyle {
                    //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            }),
            Label,
            ProgressMarker
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

fn update_progress(
    mut marker: Query<&mut Text, With<ProgressMarker>>,
    loading_state: Res<LoadingState>
) {
    let new_text = if loading_state.scaled_bodies_count > 0 && !loading_state.scaled_bodies {
        format!("Loading and scaling bodies: {}/{}", loading_state.scaled_bodies_count, loading_state.total_bodies)
    } else if !loading_state.loaded_bodies {
        "Spawning bodies".to_string()
    } else {
        "Rotating bodies".to_string()
    };
    if let Ok(mut text) = marker.get_single_mut() {
        let old_text = text.sections.first_mut().unwrap();
        if old_text.value != new_text {
            old_text.value = new_text;
        }
    }
}