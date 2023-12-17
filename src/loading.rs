use bevy::{app::{App, Plugin}, prelude::{AssetServer, BuildChildren, Color, Commands, Component, default, Entity, in_state, IntoSystemConfigs, Label, NextState, NodeBundle, OnEnter, OnExit, Query, Res, ResMut, Resource, TextBundle, Update, With, Visibility, Has}, text::{Text, TextStyle}, ui::{AlignItems, FlexDirection, JustifyContent, Node, Style, UiImage, UiRect, Val}};

use crate::{SimState, menu::BackgroundImage};

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
    mut nodes: Query<(Entity, &mut Visibility, Has<BackgroundImage>), With<Node>>
) {
    for (entity, mut visibilty, is_background) in &mut nodes {
        if !is_background {
            commands.entity(entity).despawn();
        } else {
            *visibilty = Visibility::Hidden;
        }
    }
}

fn spawn_loading(
    mut commands: Commands,
    mut parent: Query<(Entity, &mut Visibility), With<BackgroundImage>>
) {
    let (background, mut b_visibility) = parent.get_single_mut().unwrap();
    let mut parent = commands.entity(background);
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
    *b_visibility = Visibility::Visible;
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
    } else if loading_state.loaded_bodies && loading_state.scaled_bodies {
        "Rotating bodies".to_string()
    } else {
        "Spawning bodies".to_string()
    };
    if let Ok(mut text) = marker.get_single_mut() {
        let old_text = text.sections.first_mut().unwrap();
        if old_text.value != new_text {
            old_text.value = new_text;
        }
    }
}