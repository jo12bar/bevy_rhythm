use crate::consts::*;
use bevy::{
    prelude::*,
    utils::{Duration, Instant},
};

/// A specialized plugin that wraps around Bevy's [`Time`], resetting it when
/// entering the map editor or a game.
pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ControlledTime>()
            .on_state_update(APP_STATE_STAGE, AppState::Game, update_time.system())
            .on_state_update(APP_STATE_STAGE, AppState::MakeMap, update_time.system())
            .on_state_enter(APP_STATE_STAGE, AppState::Game, reset_time.system())
            .on_state_enter(APP_STATE_STAGE, AppState::MakeMap, reset_time.system());
    }
}

/// A wrapper around Bevy's [`Time`] that resets itself whenever entering
/// the [`AppState::Game`] or [`AppState::MakeMap`] states.
pub struct ControlledTime {
    delta: Duration,
    last_update: Option<Instant>,
    delta_seconds_f64: f64,
    delta_seconds: f32,
    seconds_since_startup: f64,
    startup: Instant,
}

impl ControlledTime {
    /// Reset tracked time to 0.
    pub fn reset_time(&mut self) {
        self.startup = Instant::now();
        self.seconds_since_startup = 0.0;
    }

    /// Update the tracked time to be the current time.
    pub fn update(&mut self) {
        let now = Instant::now();
        self.update_with_instant(now);
    }

    /// Update the tracked time with some instant in time.
    pub fn update_with_instant(&mut self, instant: Instant) {
        if let Some(last_update) = self.last_update {
            self.delta = instant - last_update;
            self.delta_seconds_f64 = self.delta.as_secs_f64();
            self.delta_seconds = self.delta.as_secs_f32();
        }

        let duration_since_startup = instant - self.startup;
        self.seconds_since_startup = duration_since_startup.as_secs_f64();
        self.last_update = Some(instant);
    }

    /// The delta between the current and last tick as [`f32`] seconds.
    #[inline]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    /// The delta between the current and last tick as [`f64`] seconds
    #[inline]
    pub fn delta_seconds_f64(&self) -> f64 {
        self.delta_seconds_f64
    }

    /// The time since startup in seconds
    #[inline]
    pub fn seconds_since_startup(&self) -> f64 {
        self.seconds_since_startup
    }
}

impl Default for ControlledTime {
    fn default() -> Self {
        Self {
            delta: Duration::from_secs(0),
            last_update: None,
            startup: Instant::now(),
            delta_seconds_f64: 0.0,
            delta_seconds: 0.0,
            seconds_since_startup: 0.0,
        }
    }
}

/// Updates the current time.
pub fn update_time(mut time: ResMut<ControlledTime>) {
    time.update();
}

/// Resets the current time. To be used when entering a game, or the map editor.
pub fn reset_time(mut time: ResMut<ControlledTime>) {
    time.reset_time();
}
