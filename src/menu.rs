use bevy::{app::AppExit, prelude::*};

use crate::SimState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SimState::Menu), spawn_menu)
            .add_systems(OnExit(SimState::Menu), despawn_menu)  
            .add_systems(Startup, setup_background)
            .add_systems(Update, (button_system).run_if(in_state(SimState::Menu)));
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

#[derive(Component)]
pub struct BackgroundImage;

fn despawn_menu(
    mut commands: Commands,
    background: Query<&Children, (With<Node>, With<BackgroundImage>)>
) {
    let children = background.single();
    for entity in children.iter() {
        commands.entity(*entity).despawn_recursive();   
    }
}

enum MenuButtonType {
    START,
    EXIT
}


#[derive(Component)]
struct MenuButton(pub MenuButtonType);

fn setup_background(  
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands
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
        })
        .insert(UiImage::new(asset_server.load("images/background.png")))
        .insert(BackgroundImage);
}

fn spawn_menu(
    mut commands: Commands, 
    mut parent: Query<(Entity, &mut Visibility), With<BackgroundImage>>
) {
    let (background, mut visibility) = parent.get_single_mut().unwrap();
    let mut parent = commands.entity(background);

    parent.with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Solar System Simulation",
                    TextStyle {
                        //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 70.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(20.)),
                    ..default()
                }),
                Label
            ));
            parent.spawn((
                TextBundle::from_section(
                    "by Jan Tennert",
                    TextStyle {
                        //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(40.)),
                    ..default()
                }),
                Label
            ));
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(30.)),
                        ..default()
                    },
        //            border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButton(MenuButtonType::START))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
               //     border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButton(MenuButtonType::EXIT))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
    *visibility = Visibility::Visible;
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &MenuButton
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<NextState<SimState>>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, mut color, mut border_color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match button.0 {
                    MenuButtonType::START => {
                        let _ = state.set(SimState::Loading);
                    }
                    MenuButtonType::EXIT => {
                        exit.send(AppExit);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
           //     border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
          //      border_color.0 = Color::BLACK;
            }
        }
    }
}