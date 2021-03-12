use crate::types::*;
use crate::{consts::*, score::ScoreResource};
use bevy::prelude::*;

/// Spawns and controls arrows.
pub struct ArrowsPlugin;

impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Init resources
            .init_resource::<ArrowMaterialResource>()
            // Add systems
            .add_startup_system(setup_target_arrows.system())
            .add_system(spawn_arrows.system())
            .add_system(move_arrows.system())
            .add_system(despawn_arrows.system());
    }
}

/// Keeps the textures and materials for Arrows.
struct ArrowMaterialResource {
    red_texture: Handle<ColorMaterial>,
    blue_texture: Handle<ColorMaterial>,
    green_texture: Handle<ColorMaterial>,
    border_texture: Handle<ColorMaterial>,
}

impl FromResources for ArrowMaterialResource {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get::<AssetServer>().unwrap();

        let red_handle = asset_server.load("images/arrow_red.png");
        let green_handle = asset_server.load("images/arrow_green.png");
        let blue_handle = asset_server.load("images/arrow_blue.png");
        let border_handle = asset_server.load("images/arrow_border.png");

        ArrowMaterialResource {
            red_texture: materials.add(red_handle.into()),
            blue_texture: materials.add(blue_handle.into()),
            green_texture: materials.add(green_handle.into()),
            border_texture: materials.add(border_handle.into()),
        }
    }
}

/// Arrow component.
struct Arrow {
    speed: Speed,
    direction: Directions,
}

/// The time that the initial arrow should be spawned at.
pub struct InitialArrowSpawnTime(pub f64);

/// Spawns arrows.
fn spawn_arrows(
    commands: &mut Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<ArrowMaterialResource>,
    time: Res<Time>,
    initial_arrow_spawn_time: Res<InitialArrowSpawnTime>,
) {
    // We get the current time since startup (secs) and the time since the last
    // iteration (secs_last). This lets us check if there are any arrows that
    // should spawn in the current window.

    // Song starts whenever we configure it to start, so subtract that value.
    let secs = time.seconds_since_startup() - initial_arrow_spawn_time.0;
    let secs_last = secs - time.delta_seconds_f64();

    // Counts the number of arrows that need to be spawned and removed from the
    // list.
    let mut spawn_counter = 0;

    for arrow in &song_config.arrows {
        // List is sorted by spawn time, so we can just check until an item fails.

        // Check if an arrow should be spawned at any point between the last frame
        // and this frame:
        if secs_last < arrow.spawn_time && arrow.spawn_time < secs {
            spawn_counter += 1;

            // Get the correct material according to speed:
            let material = match arrow.speed {
                Speed::Slow => materials.red_texture.clone(),
                Speed::Medium => materials.blue_texture.clone(),
                Speed::Fast => materials.green_texture.clone(),
            };

            let mut transform =
                Transform::from_translation(Vec3::new(SPAWN_POSITION, arrow.direction.y(), 1.0));

            // Rotate the arrow according to its direction:
            transform.rotate(Quat::from_rotation_z(arrow.direction.rotation()));

            commands
                .spawn(SpriteBundle {
                    material,
                    sprite: Sprite::new(Vec2::new(140.0, 140.0)),
                    transform,
                    ..Default::default()
                })
                .with(Arrow {
                    speed: arrow.speed,
                    direction: arrow.direction,
                });
        } else {
            break;
        }
    }

    // Remove any arrows that were spawned from the list:
    for _ in 0..spawn_counter {
        song_config.arrows.remove(0);
    }
}

/// Moves the arrows forwards.
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();

        let distance_after_target = transform.translation.x - (TARGET_POSITION + THRESHOLD);
        if distance_after_target >= 0.02 {
            // Move the arrow down if it's past its target
            transform.translation.y -= time.delta_seconds() * distance_after_target * 2.0;

            // Change the scale according to how far away the arrow is
            let scale = ((100.0 - distance_after_target / 3.0) / 100.0).max(0.2);
            transform.scale = Vec3::splat(scale);

            // Rotate the arrow according to distance and speed
            transform.rotate(Quat::from_rotation_z(
                -distance_after_target * arrow.speed.multiplier() / 2500.0,
            ));
        }
    }
}

/// The locations that the players have to hit the arrows at.
struct TargetArrow;

/// Setup the target arrows.
fn setup_target_arrows(commands: &mut Commands, materials: Res<ArrowMaterialResource>) {
    use Directions::*;
    const DIRECTIONS: [Directions; 4] = [Up, Down, Left, Right];

    for direction in DIRECTIONS.iter() {
        let mut transform =
            Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), 1.0));

        transform.rotate(Quat::from_rotation_z(direction.rotation()));

        commands
            .spawn(SpriteBundle {
                material: materials.border_texture.clone(),
                sprite: Sprite::new(Vec2::new(140.0, 140.0)),
                transform,
                ..Default::default()
            })
            .with(TargetArrow);
    }
}

/// Despawns the arrows when they reach the end if the correct button is clicked.
fn despawn_arrows(
    commands: &mut Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut score: ResMut<ScoreResource>,
) {
    for (entity, transform, arrow) in query.iter() {
        let pos = transform.translation.x;

        // Check if the arrow is inside clicking threshold
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && arrow.direction.key_just_pressed(&keyboard_input)
        {
            commands.despawn(entity);

            let _points = score.increase_correct(TARGET_POSITION - pos);
        }

        // Despawn arrows after they leave the screen
        if pos >= 2.0 * TARGET_POSITION {
            commands.despawn(entity);

            score.increase_fails();
        }
    }
}
