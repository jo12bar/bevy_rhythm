/// Speed at which a slow arrow moves.
pub const BASE_SPEED: f32 = 200.0;

/// X coordinate value at which arrows should spawn. Should be off-screen.
pub const SPAWN_POSITION: f32 = -400.0;

/// X coordinate value where the arrows should be clicked.
pub const TARGET_POSITION: f32 = 200.0;

/// Margin of error for clicking an arrow.
pub const THRESHOLD: f32 = 20.0;

/// Total distance travelled by an arrow, from spawn to target.
pub const DISTANCE: f32 = TARGET_POSITION - SPAWN_POSITION;

/// Stage for our systems
pub const APP_STATE_STAGE: &str = "app_state_stage";

/// Possible app states
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    Menu,
    Game,
    MakeMap,
}

/// How long to wait before starting a song after entering a game, in seconds.
pub const SONG_START_DELAY: f64 = 5.0;
