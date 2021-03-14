use super::ShaderInputs;
use crate::{
    arrows::CorrectArrowEvent,
    consts::*,
    types::Directions::{self, *},
};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, RenderGraph, RenderResourcesNode},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};

/// A component for the sparkles that appear around a target arrow when successfully
/// hitting an arrow. Keeps track of the direction of the target.
#[derive(Debug)]
pub struct TargetArrowSparkle {
    direction: Directions,
}

/// A struct to be passed to a shader as a parameter which keeps track of when
/// how long ago the player last hit an arrow, and how many points it was worth.
#[derive(RenderResources, TypeUuid)]
#[uuid = "c9400817-b3a3-4baa-8bfa-0320b9b87b17"]
pub struct TimeSinceLastCorrect {
    last_time: f32,
    points: f32,
}

/// Setup the fancy target arrows.
pub fn setup_target_arrows(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    window: Res<WindowDescriptor>,
) {
    // Create a new shader pipeline.
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("target_arrows.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("target_arrows.frag"),
        ))),
    }));

    // Add `TimeSinceLastCorrect` to the render graph.
    render_graph.add_system_node(
        "last_time",
        RenderResourcesNode::<TimeSinceLastCorrect>::new(true),
    );
    render_graph
        .add_node_edge("last_time", base::node::MAIN_PASS)
        .unwrap();

    // Different z values so they don't overlap
    const DIRECTIONS_AND_Z_VALUES: [(Directions, f32); 4] =
        [(Up, 0.3), (Down, 0.4), (Left, 0.5), (Right, 0.6)];

    for (direction, z) in DIRECTIONS_AND_Z_VALUES.iter() {
        let mut transform =
            Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), *z));
        transform.scale = Vec3::new(300.0, 300.0, 1.0);

        commands
            .spawn(SpriteBundle {
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    pipeline_handle.clone(),
                )]),
                transform,
                visible: Visible {
                    is_transparent: true,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(TargetArrowSparkle {
                direction: *direction,
            })
            .with(TimeSinceLastCorrect {
                last_time: -10.0,
                points: 0.0,
            })
            .with(ShaderInputs {
                time: 0.0,
                resolution: Vec2::new(window.width / window.height, 1.0),
            });
    }
}

pub fn correct_arrow_event_listener(
    time: Res<Time>,
    mut correct_event_reader: Local<EventReader<CorrectArrowEvent>>,
    correct_events: Res<Events<CorrectArrowEvent>>,
    mut query: Query<(&TargetArrowSparkle, &mut TimeSinceLastCorrect)>,
) {
    for event in correct_event_reader.iter(&correct_events) {
        for (arrow, mut last_correct) in query.iter_mut() {
            if arrow.direction == event.direction {
                last_correct.last_time = time.seconds_since_startup() as f32;
                last_correct.points = event.points as f32 / 100.0;
            }
        }
    }
}
