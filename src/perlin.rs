use bevy::{
    core::Bytes,
    prelude::{
        shape::{Plane, Quad},
        *,
    },
    reflect::TypeUuid,
    render::{
        pipeline::{BlendFactor, BlendOperation, BlendState, PipelineDescriptor, RenderPipeline},
        render_graph::{
            base::node::MAIN_PASS, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode,
        },
        renderer::{RenderResource, RenderResourceType, RenderResources},
        shader::{ShaderStage, ShaderStages},
    },
};

const VERTEX_SHADER2: &str = r"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in float Vertex_Uv;
layout(location = 0) out float uv;
layout(location = 1) out vec2 pos;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {

    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    uv = Vertex_Uv;
    pos = Vertex_Position.xy;
}
";

#[derive(Debug, Clone)]
pub struct PerlinPipelineHandle(Handle<PipelineDescriptor>);

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "af8c8bb6-bab2-48e9-9251-6b757d28afda"]
pub struct TimeComponent {
    value: f32,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "4218782a-4931-4983-8188-121dc3cf3be1"]
pub struct PerlinComponent {
    pub resolution: Vec2,
}

#[derive(Bundle)]
pub struct PerlinBundle {
    time: TimeComponent,
    noise: PerlinComponent,
    render_pipelines: RenderPipelines,
    blend_state: BlendState,
    visible: Visible,
}

impl PerlinBundle {
    pub fn new(handle: &PerlinPipelineHandle, resolution: Vec2) -> Self {
        PerlinBundle {
            time: TimeComponent::default(),
            noise: { PerlinComponent { resolution } },
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                handle.0.clone(),
            )]),
            blend_state: BlendState {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            visible: Visible {
                is_transparent: true,
                is_visible: true,
            },
        }
    }
}

fn animate_shader(time: Res<Time>, mut query: Query<&mut TimeComponent>) {
    for mut time_component in query.iter_mut() {
        time_component.value = time.seconds_since_startup() as f32;
    }
}

impl FromWorld for PerlinPipelineHandle {
    fn from_world(world: &mut World) -> Self {
        let mut render_graph: Mut<RenderGraph> = world.get_resource_mut().expect("render graph");

        render_graph.add_system_node(
            "time_component",
            RenderResourcesNode::<TimeComponent>::new(true),
        );
        render_graph
            .add_node_edge("time_component", MAIN_PASS)
            .unwrap();

        let mut shaders: Mut<Assets<Shader>> = world.get_resource_mut().expect("shaders");
        let vertex = shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            VERTEX_SHADER2, // include_str!("../assets/blend.vert"),
        ));
        let fragment = Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("../assets/camera_perlin.frag"),
        )));

        let mut pipelines: Mut<Assets<PipelineDescriptor>> =
            world.get_resource_mut().expect("pipelines");
        let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
            vertex,
            fragment,
        }));
        PerlinPipelineHandle(pipeline_handle)
    }
}

pub struct PerlinPlugin;

impl Plugin for PerlinPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PerlinPipelineHandle>()
            .add_system(animate_shader.system());
    }
}
