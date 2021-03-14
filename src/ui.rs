use crate::{consts::*, score::ScoreResource, time::ControlledTime};
use bevy::prelude::*;

/// All the UI!
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(APP_STATE_STAGE, AppState::Game, setup_ui.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, update_time_text.system())
            .on_state_update(APP_STATE_STAGE, AppState::Game, update_score_text.system());
    }
}

fn setup_ui(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let material = color_materials.add(Color::NONE.into());

    commands
        // Time text node
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: material.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        value: "Time: 0.0".to_string(),
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(TimeText);
        })
        // Score text node
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        value: format!("Score: 0  |  Corrects: 0  |  Fails: 0"),
                        font,
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(ScoreText);
        });
}

struct TimeText;

fn update_time_text(time: Res<ControlledTime>, mut query: Query<(&mut Text, &TimeText)>) {
    let secs = time.seconds_since_startup() - SONG_START_DELAY;

    // Don't do anything before the song starts!
    if secs < 0.0 {
        return;
    }

    for (mut text, _marker) in query.iter_mut() {
        text.value = format!("Time: {:.2}", secs);
    }
}

/// Displays the score.
struct ScoreText;

fn update_score_text(score: ChangedRes<ScoreResource>, mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, _marker) in query.iter_mut() {
        text.value = format!(
            "Score: {}  |  Hits: {}  |  Misses: {}",
            score.score(),
            score.corrects(),
            score.fails(),
        )
    }
}
