use crate::consts::*;
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use core::f32::consts::PI;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl Directions {
    /// Checks if a key corresponding to this direction has been pressed.
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Self::Up => [KeyCode::Up, KeyCode::W],
            Self::Down => [KeyCode::Down, KeyCode::S],
            Self::Left => [KeyCode::Left, KeyCode::A],
            Self::Right => [KeyCode::Right, KeyCode::D],
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }

    /// Returns the correct rotation for an arrow with this direction
    pub fn rotation(&self) -> f32 {
        match self {
            Self::Up => PI / 2.0,
            Self::Down => -PI / 2.0,
            Self::Left => PI,
            Self::Right => 0.0,
        }
    }

    /// Returns the correct y coordinate for an arrow with this direction
    pub fn y(&self) -> f32 {
        match self {
            Self::Up => 150.0,
            Self::Down => 50.0,
            Self::Left => -50.0,
            Self::Right => -150.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}

impl Speed {
    /// Returns the actual speed at which arrows should move.
    pub fn value(&self) -> f32 {
        BASE_SPEED * self.multiplier()
    }

    /// Speed multiplier.
    pub fn multiplier(&self) -> f32 {
        match self {
            Self::Slow => 1.0,
            Self::Medium => 1.2,
            Self::Fast => 1.5,
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// Keeps track of when each arrow should spawn, its speed, and its direction.
pub struct ArrowTime {
    pub spawn_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}

impl ArrowTime {
    fn new(arrow: &ArrowTimeToml) -> Self {
        let speed_value = arrow.speed.value();
        Self {
            spawn_time: arrow.click_time - (DISTANCE / speed_value) as f64,
            speed: arrow.speed,
            direction: arrow.direction,
        }
    }

    /// Gets the time that the arrow will be clicked at.
    pub fn get_click_time(&self) -> f64 {
        self.spawn_time + (DISTANCE / self.speed.value()) as f64
    }
}

/// Keeps track of a list of all the arrows in a song, including their speeds
/// and their directions and when they should be clicked or spawned.
#[derive(Debug)]
pub struct SongConfig {
    pub name: String,
    pub song_audio: Handle<AudioSource>,
    pub arrows: Vec<ArrowTime>,
}

/// An `ArrowTime` that stores the click time instead of the spawn time. Used
/// for TOML songfiles.
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct ArrowTimeToml {
    pub click_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}

/// A `SongConfig` that stores the click times instead of the spawn times. Used
/// for TOML songfiles.
#[derive(Deserialize, Debug)]
struct SongConfigToml {
    pub name: String,
    pub filename: String,
    pub arrows: Vec<ArrowTimeToml>,
}

pub fn load_config(path: &str, asset_server: &AssetServer) -> SongConfig {
    // Open file and read contents
    let mut file = File::open(format!("assets/songs/{}", path)).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file into a UTF-8 string");

    // Parse using toml and Serde
    let parsed: SongConfigToml = toml::from_str(&contents).expect("Couldn't parse file as TOML");

    // Process arrows
    let mut arrows = parsed
        .arrows
        .iter()
        .map(|arr| ArrowTime::new(arr))
        .collect::<Vec<_>>();

    // Sort arrows by spawn time
    arrows.sort_unstable_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());

    // Load song audio and get the handle
    let song_audio = asset_server.load(&*format!("songs/{}", parsed.filename));

    SongConfig {
        name: parsed.name,
        song_audio,
        arrows,
    }
}
