use super::*;

/// A shader-based background.
pub struct Background;

/// Sets up a shader-based backgound.
pub fn setup_background(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    window: Res<WindowDescriptor>,
) {
    let color_mat = color_materials.add(Color::rgb(0.0, 0.0, 0.0).into());

    // Create a new shader pipeline.
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("background.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("background.frag"),
        ))),
    }));

    commands
        .spawn(SpriteBundle {
            material: color_mat,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_scale(Vec3::new(
                window.width + 10.0,
                window.height + 10.0,
                1.0,
            )),
            ..Default::default()
        })
        .with(Background)
        .with(ShaderInputs {
            time: 0.0,
            resolution: Vec2::new(window.width / window.height, 1.0),
        });
}

/// Resizes the background when the window is resized.
pub fn update_background_size(
    mut event_reader: Local<EventReader<WindowResized>>,
    events: Res<Events<WindowResized>>,
    mut background: Query<(&mut Transform, &Background)>,
) {
    for event in event_reader.iter(&events) {
        for (mut transform, _) in background.iter_mut() {
            transform.scale = Vec3::new(event.width, event.height, 1.0);
        }
    }
}
