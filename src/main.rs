use bevy::{input::system::exit_on_esc_system, prelude::*};

mod arrows;
mod audio;
mod consts;
mod score;
mod types;
mod ui;

use arrows::ArrowsPlugin;
use audio::AudioPlugin;
use score::ScoreResource;
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
        .init_resource::<ScoreResource>()
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .run();
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let config = types::load_config("test.toml", &asset_server);

    // Start songs five seconds after startup:
    let initial_arrow_spawn_time = config.arrows[0].get_click_time() + 5.0;

    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        .insert_resource(config)
        .insert_resource(arrows::InitialArrowSpawnTime(initial_arrow_spawn_time));
}
