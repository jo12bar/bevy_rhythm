use crate::{consts::*, types::load_config};
use bevy::prelude::*;
use std::{fs::read_dir, iter};

/// A plugin for handling all the game's menus.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .on_state_enter(APP_STATE_STAGE, AppState::Menu, setup_menu.system())
            .on_state_update(
                APP_STATE_STAGE,
                AppState::Menu,
                button_color_system.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::Menu,
                button_press_system.system(),
            )
            .on_state_exit(APP_STATE_STAGE, AppState::Menu, despawn_menu.system());
    }
}

/// Hold the materials used for menu buttons.
struct ButtonMaterials {
    none: Handle<ColorMaterial>,
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
    font: Handle<Font>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get::<AssetServer>().unwrap();

        Self {
            none: materials.add(Color::NONE.into()),
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        }
    }
}

/// A marker struct for the Menu's UI.
struct MenuUI;

/// All the menu buttons
enum MenuButton {
    MakeMap,
    PlaySong(String),
}

impl MenuButton {
    fn name(&self) -> String {
        match self {
            Self::MakeMap => "Make Map".to_string(),
            Self::PlaySong(song) => format!("Play song: {}", song),
        }
    }
}

/// Create the menu UI.
fn setup_menu(commands: &mut Commands, button_materials: Res<ButtonMaterials>) {
    // A list of menu buttons:
    let buttons = get_songs()
        .into_iter()
        .map(|name| MenuButton::PlaySong(name))
        .chain(iter::once(MenuButton::MakeMap))
        .collect::<Vec<_>>();

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            material: button_materials.none.clone(),
            ..Default::default()
        })
        .with(MenuUI)
        .with_children(|parent| {
            // Add all of the buttons as children.
            for button in buttons {
                // Spawn a new button:
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                            margin: Rect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        material: button_materials.normal.clone(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                value: button.name(),
                                font: button_materials.font.clone(),
                                style: TextStyle {
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                    ..Default::default()
                                },
                            },
                            ..Default::default()
                        });
                    })
                    .with(button);
            }
        });
}

/// Remove the menu.
fn despawn_menu(commands: &mut Commands, query: Query<(Entity, &MenuUI)>) {
    for (entity, _) in query.iter() {
        commands.despawn_recursive(entity);
    }
}

/// Handles changing menu button backgrounds whenever their states change.
fn button_color_system(
    button_materials: Res<ButtonMaterials>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Mutated<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
            }

            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }

            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

/// Handles button clicks.
fn button_press_system(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Interaction, &MenuButton), (Mutated<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                // If the map maker button was clicked, change the state.
                MenuButton::MakeMap => state
                    .set_next(AppState::MakeMap)
                    .expect("Couldn't switch state to MakeMap."),

                // If a play song button was clicked, load the config, insert it
                // as a resource, and change state.
                MenuButton::PlaySong(song) => {
                    let config = load_config(&format!("{}.toml", song), &asset_server);

                    commands.insert_resource(config);

                    state
                        .set_next(AppState::Game)
                        .expect("Couldn't switch state to Game")
                }
            }
        }
    }
}

/// Gets all available songs.
pub fn get_songs() -> Vec<String> {
    read_dir("assets/songs")
        .unwrap()
        .map(|dir_entry| dir_entry.unwrap().path())
        .filter(|path| {
            path.as_path()
                .extension()
                .map_or(false, |ext| ext == "toml")
        })
        .map(|path| {
            path.as_path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect()
}
