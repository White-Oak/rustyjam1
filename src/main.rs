mod map;
mod movement;
mod player;
mod camera_enemy;

use std::f32::consts::PI;

use bevy::{core::Bytes, prelude::{*, shape::{Plane, Quad}}, reflect::TypeUuid, render::{pipeline::{PipelineDescriptor, RenderPipeline}, render_graph::{AssetRenderResourcesNode, RenderGraph, RenderResourcesNode, base::node::MAIN_PASS}, renderer::{RenderResource, RenderResourceType, RenderResources}, shader::{ShaderStage, ShaderStages}}};
use bevy_ecs_tilemap::prelude::*;
use map::MapPlugin;

use crate::{movement::MovementPlugin, player::PlayerPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    LoadingLevel,
    Level,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 800.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_state(GameState::LoadingLevel)
        .add_plugin(PlayerPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(TiledMapPlugin)
        .add_plugin(MapPlugin)
        .add_system(animate_shader.system())
        .add_startup_system(setup.system())
        .run();
}

pub struct MainCamera;
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "af8c8bb6-bab2-48e9-9251-6b757d28afda"]
struct TimeComponent {
    value: f32,
}

fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    render_graph.add_system_node(
        "time_component",
        RenderResourcesNode::<TimeComponent>::new(true),
    );
    render_graph
        .add_node_edge("time_component", MAIN_PASS)
        .unwrap();

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            VERTEX_SHADER2
            // include_str!("../assets/blend.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            // FRAGMENT_SHADER2
            include_str!("../assets/classic_perlin_animated.frag"),
        ))),
    }));
    // let mut ui_bundle = commands.spawn_bundle(UiCameraBundle::default());
    // let commands = ui_bundle // root node
    //     .commands();

    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    // let mut mesh: Mesh = Plane{ size:  2000.}.into();

    let mut v_pos = vec![[0.0, 0.0, 0.0]];
    let a1 = PI / 6.;
    let a2 = 5. * PI / 6.;
    let a3 = 3. * PI / 2.;
    let radius = 100.;
    let angles = [a1, a2, a3];
    for angle in angles {
        // orig (radius, 0)
        let x = angle.cos()*radius;
        let y = angle.sin()*radius;
        v_pos.push([x, y, 0.]);
    }
    dbg!(&v_pos);
    // Set the position attribute
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

    // And a RGB color attribute as well
    let v_color = vec![[0.0, 0.0, 0.0], [1., 0., 0.], [0., 1., 0.], [0., 0.,1.]];
    mesh.set_attribute("Vertex_Color", v_color);

    let uv = vec![[0.1, 0.0], [0., 0.], [0., 0.], [0., 0.]];
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    // Now, we specify the indices of the vertex that are going to compose the
    // triangles in our star. Vertices in triangles have to be specified in CCW
    // winding (that will be the front face, colored). Since we are using
    // triangle list, we will specify each triangle as 3 vertices
    let mut indices = vec![0, 1, 2, 0, 2, 3, 0, 3, 1];
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    commands
        .spawn_bundle(MeshBundle {
            // commands.spawn_bundle(SpriteBundle {
            //     sprite: Sprite::new(Vec2::splat(1000.)),
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle.clone(),
            )]),
            transform: Transform::from_xyz(0., 0., 0.4),
            ..Default::default()
        })
        .insert(TimeComponent { value: 0.})
        // .insert(material);
        ;
}

fn animate_shader(time: Res<Time>, mut query: Query<&mut TimeComponent>) {
    for mut time_component in query.iter_mut() {
        time_component.value = time.seconds_since_startup() as f32;
    }
}

const VERTEX_SHADER: &str = r"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Color;
layout(location = 1) out vec3 v_Color;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {

    v_Color = Vertex_Color;
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    uv = Vertex_Uv;
}
";

const VERTEX_SHADER2: &str = r"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;
layout(location = 0) out vec2 uv;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {

    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    uv = Vertex_Uv;
}
";

const FRAGMENT_SHADER2: &str = r"
#version 450
layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 o_Target;
layout(set = 2, binding = 0) uniform TimeComponent_value {
    float time;
};
void main() {
    vec3 white = vec3(1.,1.,1.);
    vec3 red = vec3(1.,0.,0.);
    float speed = 2.;
    float translation = (sin(time * speed) + 1.) / 2;
    float percentage_extent = 0.1;
    float threshold = uv.x + translation * percentage_extent;
    vec3 mixed = mix(red, white, threshold);
    o_Target = vec4(mixed, 1.0);
}
";



const FRAGMENT_SHADER: &str = r"
#version 450
layout(location = 1) in vec3 v_Color;
layout(location = 0) out vec4 o_Target;
layout(set = 2, binding = 0) uniform TimeComponent_value {
    float time;
};
void main() {
    vec3 white = vec3(1.,1.,1.);
    vec3 red = vec3(1.,0.,0.);
    float speed = 2.;
    float translation = (sin(time * speed) + 1.) / 2;
    float percentage_extent = 0.1;
    float threshold = translation * percentage_extent;
    vec3 mixed = mix(red, white, threshold);
    o_Target = vec4(mixed, 1.0);
}
";

const FRAG_SHADER_PERLIN: &str = r"
";
