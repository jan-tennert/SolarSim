use bevy::{app::{App, Plugin}, prelude::{AssetServer, BuildChildren, Color, Commands, Component, default, Entity, in_state, IntoSystemConfigs, Label, NextState, NodeBundle, OnEnter, OnExit, Query, Res, ResMut, Resource, TextBundle, Update, With, Visibility, Has, Children, DespawnRecursiveExt}, text::{Text, TextStyle}, ui::{AlignItems, FlexDirection, JustifyContent, Node, Style, UiImage, UiRect, Val}};

use crate::{menu::BackgroundImage};
use crate::simulation::{SimState, SimStateType};

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
    
    pub scaled_bodies: bool,
    pub scaled_bodies_count: i32,
    pub total_bodies: i32,
    pub tilted_bodies: bool,
    pub spice_loaded: i32,
    pub spice_total: i32,
    pub loaded_spice_files: bool,
    pub spawned_bodies: bool

}

impl LoadingState {
    
    pub fn reset(&mut self) {
        self.scaled_bodies = false;
        self.tilted_bodies = false;
        self.spice_loaded = 0;
        self.spice_total = 0;
        self.spawned_bodies = false;
        self.scaled_bodies_count = 0;
        self.loaded_spice_files = false;
        self.total_bodies = 0;
    }
    
    pub fn is_done(&self) -> bool {
        self.scaled_bodies && self.tilted_bodies && self.loaded_spice_files && self.spawned_bodies
    }
    
}

fn despawn_loading(
    mut commands: Commands,
    mut background: Query<(&Children, &mut Visibility), (With<Node>, With<BackgroundImage>)>
) {
    let (children, mut visibility) = background.single_mut();
    for entity in children.iter() {
        commands.entity(*entity).despawn_recursive();   
    }
    *visibility = Visibility::Hidden;
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
        sim_state.set(SimState::Loaded)
    }
}

fn update_progress(
    mut marker: Query<&mut Text, With<ProgressMarker>>,
    loading_state: Res<LoadingState>,
    sim_type: Res<SimStateType>
) {
    let text0 = loading_text("Spawning bodies", loading_state.spawned_bodies, false);
    let text1 = loading_text(format!("Scaling bodies: {}/{}", loading_state.scaled_bodies_count, loading_state.total_bodies).as_str(), loading_state.scaled_bodies, false);
    let text2 = loading_text("Rotating bodies", loading_state.tilted_bodies, false);
    let text3 = loading_text(format!("Loading SPK files: {}/{}", loading_state.spice_loaded, loading_state.spice_total).as_str(), loading_state.loaded_spice_files, false);
    let new_text = format!("{}\n{}\n{}\n{}", text0, text1, text2, text3);
    if let Ok(mut text) = marker.get_single_mut() {
        let old_text = text.sections.first_mut().unwrap();
        if old_text.value != new_text {
            old_text.value = new_text;
        }
    }
}

fn loading_text(text: &str, predicate: bool, skip: bool) -> String {
    if predicate {
        format!("Done - {}", text)
    } else if skip {
        format!("Skipped - {}", text)
    } else {
        format!("In Progress - {}", text)
    }
}