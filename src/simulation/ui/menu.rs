use crate::simulation::{SimState, SimStateType};
use bevy::{app::AppExit, prelude::*};
use bevy_egui::EguiContexts;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SimState::Menu), spawn_menu)
            .add_systems(OnExit(SimState::Menu), despawn_menu)
            .add_systems(Update, setup_background.run_if(in_state(SimState::Setup)))
            .add_systems(Update, (button_system).run_if(in_state(SimState::Menu)));
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

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
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<SimState>>,
    egui_context: EguiContexts
) {
    commands
        .spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
        })
        .insert(BackgroundColor(Color::WHITE.into()))
        .insert(ImageNode::new(asset_server.load("images/background.png")))
        .insert(BackgroundImage);
    state.set(SimState::Menu);
}

fn spawn_menu(
    mut commands: Commands, 
    mut parent: Query<(Entity, &mut Visibility), With<BackgroundImage>>
) {
    let (background, mut visibility) = parent.get_single_mut().unwrap();
    let mut parent = commands.entity(background);

    parent.with_children(|parent| {
            parent.spawn((
                Text::from("Solar System Simulation"),
                TextFont::from_font_size(70.0),
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                Label
            ));
            parent.spawn((
                Text::from("by Jan Tennert"),
                TextFont::from_font_size(20.0),
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(40.)),
                    ..default()
                },
                Label
            ));
            button("Start", MenuButtonType::START, parent);
            button("Exit", MenuButtonType::EXIT, parent);
        });
    *visibility = Visibility::Visible;
}

fn button(text: &str, button_type: MenuButtonType, builder: &mut ChildBuilder) {
    builder
        .spawn(Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        })
        .insert(BackgroundColor(NORMAL_BUTTON.into()))
        .insert(MenuButton(button_type))
        .insert(Button)
        .with_children(|parent| {
            parent.spawn((
                Text::from(text),
                TextColor(Color::srgb(0.9, 0.9, 0.9).into()),
                TextFont::from_font_size(40.0),
            ));
        });
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
    sim_type: ResMut<SimStateType>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, mut color, border_color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match button.0 {
                    MenuButtonType::START => {
                        let _ = state.set(SimState::ScenarioSelection);
                    }
                    MenuButtonType::EXIT => {
                        exit.send(AppExit::Success);
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