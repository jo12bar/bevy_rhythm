use crate::{
    consts::*,
    time::ControlledTime,
    types::{
        ArrowTimeToml,
        Directions::{self, *},
        Speed,
    },
};
use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use serde_derive::Serialize;
use std::fs::File;
use std::io::prelude::*;

/// A really simplistic map maker.
pub struct MapMakerPlugin;

impl Plugin for MapMakerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Presses>()
            .init_resource::<MapMakerAudio>()
            .on_state_enter(APP_STATE_STAGE, AppState::MakeMap, start_song.system())
            .on_state_enter(
                APP_STATE_STAGE,
                AppState::MakeMap,
                setup_map_maker_arrows.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::MakeMap,
                toggle_map_maker_arrows.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::MakeMap,
                save_key_presses.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::MakeMap,
                save_to_file_on_exit.system(),
            );
    }
}

/// Keeps track of when keys are pressed.
#[derive(Serialize, Debug, Default)]
struct Presses {
    arrows: Vec<ArrowTimeToml>,
}

/// Saves key presses to [`Presses`].
fn save_key_presses(
    time: Res<ControlledTime>,
    keyboard_input: Res<Input<KeyCode>>,
    mut presses: ResMut<Presses>,
) {
    const DIRECTIONS: [Directions; 4] = [Up, Down, Left, Right];
    for direction in DIRECTIONS.iter() {
        if direction.key_just_pressed(&keyboard_input) {
            presses.arrows.push(ArrowTimeToml {
                click_time: time.seconds_since_startup(),
                speed: Speed::Slow,
                direction: *direction,
            });
        }
    }
}

/// Saves our [`Presses`] to a file.
fn save_to_file_on_exit(
    mut event_reader: Local<EventReader<AppExit>>,
    events: Res<Events<AppExit>>,
    presses: Res<Presses>,
) {
    for _ in event_reader.iter(&events) {
        let text = toml::to_string(&*presses).expect("Couldn't convert keypresses to TOML!");

        let mut file = File::create("map.toml").expect("Couldn't create and open map.toml");
        file.write_all(text.as_bytes())
            .expect("Couldn't write to map.toml");
    }
}

/// An arrow that appears on the map maker screen when the user presses a key.
struct MapMakerArrow(Directions);

/// Sets up each map maker arrow.
fn setup_map_maker_arrows(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    let border_handle = materials.add(asset_server.load("images/arrow_border.png").into());

    const DIRECTIONS_AND_Y_VALS: [(Directions, f32); 4] =
        [(Up, 150.0), (Down, 50.0), (Left, -50.0), (Right, -150.0)];

    for (direction, y) in DIRECTIONS_AND_Y_VALS.iter() {
        let mut transform = Transform::from_translation(Vec3::new(0.0, *y, 1.0));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));

        commands
            .spawn(SpriteBundle {
                material: border_handle.clone(),
                sprite: Sprite::new(Vec2::new(140.0, 140.0)),
                transform,
                ..Default::default()
            })
            .with(MapMakerArrow(*direction));
    }
}

/// Toggles map maker arrow visibility according to if corresponding key is
/// being pressed.
fn toggle_map_maker_arrows(
    mut query: Query<(&mut Visible, &MapMakerArrow)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut visible, arrow) in query.iter_mut() {
        visible.is_visible = arrow.0.key_pressed(&keyboard_input);
    }
}

/// Just plays a single song by default. Hardcoded. The song select screen will
/// probably be built later.
struct MapMakerAudio(Handle<AudioSource>);

impl FromResources for MapMakerAudio {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get::<AssetServer>().unwrap();
        let audio = asset_server.load("Electronic Fantasy.mp3");
        Self(audio)
    }
}

/// Start playing some music!
fn start_song(audio: Res<Audio>, map_maker_audio: Res<MapMakerAudio>) {
    audio.play(map_maker_audio.0.clone());
}
