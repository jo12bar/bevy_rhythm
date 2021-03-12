use crate::{arrows::InitialArrowSpawnTime, types::SongConfig};
use bevy::prelude::*;

/// Handles all of the game's audio.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(start_song.system());
    }
}

fn start_song(
    audio: Res<Audio>,
    time: Res<Time>,
    config: Res<SongConfig>,
    initial_arrow_spawn_time: Res<InitialArrowSpawnTime>,
) {
    // Song starts whenever the first arrow spawns.
    let secs = time.seconds_since_startup();
    let secs_last = secs - time.delta_seconds_f64();
    let start_time = initial_arrow_spawn_time.0;

    if secs_last <= start_time && start_time <= secs {
        audio.play(config.song_audio.clone());
    }
}
