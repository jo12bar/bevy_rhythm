use crate::{consts::*, time::ControlledTime, types::SongConfig};
use bevy::prelude::*;

/// Handles all of the game's audio.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_update(APP_STATE_STAGE, AppState::Game, start_song.system());
    }
}

fn start_song(audio: Res<Audio>, time: Res<ControlledTime>, config: Res<SongConfig>) {
    // Song starts whenever the first arrow spawns.
    let secs = time.seconds_since_startup();
    let secs_last = secs - time.delta_seconds_f64();

    if secs_last <= SONG_START_DELAY && SONG_START_DELAY <= secs {
        audio.play(config.song_audio.clone());
    }
}
