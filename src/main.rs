use bevy::{input::system::exit_on_esc_system, prelude::*, render::pass::ClearColor};

mod arrows;
mod audio;
mod consts;
mod menu;
mod score;
mod shaders;
mod time;
mod types;
mod ui;

use arrows::ArrowsPlugin;
use audio::AudioPlugin;
use consts::*;
use menu::MenuPlugin;
use score::ScoreResource;
use shaders::ShadersPlugin;
use time::TimePlugin;
use ui::UIPlugin;

fn main() {
    App::build()
        // Set antialiasing to 4xAA
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .add_resource(State::new(AppState::Menu))
        .add_stage_after(
            stage::UPDATE,
            APP_STATE_STAGE,
            StateStage::<AppState>::default(),
        )
        .init_resource::<ScoreResource>()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ShadersPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(TimePlugin)
        .run();
}

fn setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}
