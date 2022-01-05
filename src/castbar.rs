use std::f32::consts::PI;

use bevy::{
    log,
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{BlendFactor, BlendOperation, BlendState, PipelineDescriptor, RenderPipeline},
        render_graph::{base::node::MAIN_PASS, RenderGraph, RenderResourcesNode},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};

use crate::{
    player::{Casting, Player, PLAYER_SIZE},
    GameState,
};

pub struct Castbar;

#[derive(Bundle)]
struct CastbarBundle {
    percent: PercentComponent,
    blend_state: BlendState,
    castbar: Castbar,
    #[bundle]
    mesh_bundle: MeshBundle,
}

const CASTBAR_W: f32 = PLAYER_SIZE;
const CASTBAR_H: f32 = 10.;

impl CastbarBundle {
    fn new(pipeline: &CastbarPipeline, meshes: &mut Assets<Mesh>) -> Self {
        let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
        let x = -CASTBAR_W / 2.;
        let y = CASTBAR_H / 2.;
        let x2 = CASTBAR_W / 2.;
        let y2 = -CASTBAR_H / 2.;

        let v_pos = vec![[x, y, 0.], [x2, y, 0.], [x2, y2, 0.], [x, y2, 0.]];
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

        let v_color = vec![[0.0, 0.0, 0.0], [1., 0., 0.], [1., 0., 0.], [0., 0., 0.]];
        mesh.set_attribute("Vertex_Color", v_color);
        let indices = vec![0, 2, 1, 0, 3, 2];
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
        let uv = vec![1., 1., 1., 1.];
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);
        let mesh_bundle = MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline.0.clone(),
            )]),
            transform: Transform::from_xyz(0.0, PLAYER_SIZE / 2. + 10., 0.1),
            visible: Visible {
                is_visible: false,
                is_transparent: true,
            },
            ..Default::default()
        };

        CastbarBundle {
            blend_state: BlendState {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            percent: PercentComponent::default(),
            castbar: Castbar,
            mesh_bundle,
        }
    }
}

fn spawn_castbar(
    mut commands: Commands,
    player: Query<Entity, Added<Player>>,
    castbar_pipeline: Res<CastbarPipeline>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    player.for_each(|player| {
        commands.entity(player).with_children(|ec| {
            ec.spawn_bundle(CastbarBundle::new(&castbar_pipeline, &mut meshes));
        });
    });
}

fn update_castbar(
    casting: Res<Option<Casting>>,
    castbar: Query<(&mut PercentComponent, &mut Visible), With<Castbar>>,
) {
    if casting.is_changed() {
        castbar.for_each_mut(|(mut percent, mut visible)| {
            if let Some(casting) = casting.as_ref() {
                visible.is_visible = true;
                percent.value = 1. - casting.timer.percent_left();
        log::debug!(percent = percent.value,"updating castbar");
            } else {
                visible.is_visible = false;
            }
        });
    }
}

struct CastbarPipeline(Handle<PipelineDescriptor>);

impl FromWorld for CastbarPipeline {
    fn from_world(world: &mut World) -> Self {
        let mut render_graph: Mut<RenderGraph> = world.get_resource_mut().expect("render graph");

        render_graph.add_system_node(
            "percent_component",
            RenderResourcesNode::<PercentComponent>::new(true),
        );
        let mut shaders = world
            .get_resource_mut::<Assets<Shader>>()
            .expect("no shaders");
        let vertex = shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("../assets/castbar.vert"),
        ));
        let fragment = shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("../assets/castbar.frag"),
        ));
        let mut pipelines = world
            .get_resource_mut::<Assets<PipelineDescriptor>>()
            .expect("no pipelines");
        let handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
            vertex,
            fragment: Some(fragment),
        }));
        CastbarPipeline(handle)
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "14b139fb-3f2d-47e2-b75e-feff6e389927"]
pub struct PercentComponent {
    value: f32,
}

pub struct CastbarPlugin;
impl Plugin for CastbarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CastbarPipeline>().add_system_set(
            SystemSet::on_update(GameState::Level)
                .with_system(spawn_castbar.system())
                .with_system(update_castbar.system()),
        );
    }
}
