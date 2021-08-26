mod map;
mod movement;
mod player;

use std::f32::consts::PI;

use bevy::{
    core::Bytes,
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base::node::MAIN_PASS, AssetRenderResourcesNode, RenderGraph},
        renderer::{RenderResource, RenderResourceType, RenderResources},
        shader::{ShaderStage, ShaderStages},
    },
};
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
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_state(GameState::LoadingLevel)
        .add_plugin(PlayerPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(TiledMapPlugin)
        .add_plugin(MapPlugin)
        .add_asset::<BlendColors>()
        .add_startup_system(setup.system())
        .run();
}

pub struct MainCamera;

fn setup(
    mut commands: Commands,
    mut colors: ResMut<Assets<ColorMaterial>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BlendColors>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    let material = colors.add(ColorMaterial {
        color: Color::rgba(0., 0., 0., 0.9),
        texture: None,
    });

    // render_graph.add_system_node(
    //     "blend_colors",
    //     AssetRenderResourcesNode::<BlendColors>::new(true),
    // );
    // render_graph
    //     .add_node_edge("blend_colors", MAIN_PASS)
    //     .unwrap();
    let material = materials.add(BlendColors {
        color_a: Color::rgb(0.0, 0.0, 1.0),
        color_b: Color::rgb(1.0, 0.0, 0.0),
        start_lerp: 0.25,
        end_lerp: 0.75,
    });

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            VERTEX_SHADER
            // include_str!("../assets/blend.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            FRAGMENT_SHADER
            // include_str!("../assets/blend.frag"),
        ))),
    }));
    // let mut ui_bundle = commands.spawn_bundle(UiCameraBundle::default());
    // let commands = ui_bundle // root node
    //     .commands();

    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);

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
    let mut v_color = vec![[0.0, 0.0, 0.0], [1., 0., 0.], [0., 1., 0.], [0., 0.,1.]];
    mesh.set_attribute("Vertex_Color", v_color);
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
        // .insert(material);
        ;
}
#[derive(RenderResources, Default, TypeUuid)]
#[render_resources(from_self)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
#[repr(C)]
struct BlendColors {
    pub color_a: Color,
    pub color_b: Color,
    pub start_lerp: f32,
    pub end_lerp: f32,
}

impl RenderResource for BlendColors {
    fn resource_type(&self) -> Option<RenderResourceType> {
        Some(RenderResourceType::Buffer)
    }

    fn buffer_byte_len(&self) -> Option<usize> {
        Some(40)
    }

    fn write_buffer_bytes(&self, buffer: &mut [u8]) {
        let (color_a_buf, rest) = buffer.split_at_mut(16);
        self.color_a.write_bytes(color_a_buf);

        let (color_b_buf, rest) = rest.split_at_mut(16);
        self.color_b.write_bytes(color_b_buf);

        let (start_lerp_buf, rest) = rest.split_at_mut(4);
        self.start_lerp.write_bytes(start_lerp_buf);

        self.end_lerp.write_bytes(rest);
    }

    fn texture(&self) -> Option<&Handle<Texture>> {
        None
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
}
";

const FRAGMENT_SHADER: &str = r"
#version 450
layout(location = 1) in vec3 v_Color;
layout(location = 0) out vec4 o_Target;
void main() {
    o_Target = vec4(v_Color, 1.0);
}
";
