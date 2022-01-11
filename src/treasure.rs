use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        shader::{ShaderStage, ShaderStages},
    },
    sprite::collide_aabb,
};

use crate::{
    perlin::TimeComponent,
    player::{LevelMarker, Player},
    GameState,
};

pub struct TreasureSpawn;

fn spawn_treasure(
    mut commands: Commands,
    query: Query<(Entity, &Transform), Added<TreasureSpawn>>,
    mut meshes: ResMut<Assets<Mesh>>,

    pipeline: Res<TreasurePipeline>,
) {
    query.for_each(|(entity, tr)| {
        let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);

        let mut v_pos = vec![[0.0, 0.0, 0.0]];
        let a1 = PI / 6.;
        let a2 = 5. * PI / 6.;
        let a3 = 3. * PI / 2.;
        let radius = 50.;
        let angles = [a1, a2, a3];
        for angle in angles {
            // orig (radius, 0)
            let x = angle.cos() * radius;
            let y = angle.sin() * radius;
            v_pos.push([x, y, 0.]);
        }
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

        let v_color = vec![[0.0, 0.0, 0.0], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
        mesh.set_attribute("Vertex_Color", v_color);
        let indices = vec![0, 1, 2, 0, 2, 3, 0, 3, 1];
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
        commands
            .entity(entity)
            .insert_bundle(MeshBundle {
                mesh: meshes.add(mesh),
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    pipeline.0.clone(),
                )]),
                transform: *tr,
                ..Default::default()
            })
            .insert(TimeComponent::default())
            .insert(LevelMarker);
    });
}

fn treasure_collide(
    player: Query<&Transform, With<Player>>,
    treasures: Query<&Transform, With<TreasureSpawn>>,
    mut state: ResMut<State<GameState>>,
) {
    let tr = player.single().expect("single player").translation;
    for tr_tr in treasures.iter() {
        if collide_aabb::collide(tr, Vec2::splat(100.), tr_tr.translation, Vec2::splat(100.))
            .is_some()
        {
            state
                .push(GameState::ChoosingTreasure)
                .expect("cant move to treasure choosing");
            return;
        }
    }
}

struct TreasurePipeline(Handle<PipelineDescriptor>);

impl FromWorld for TreasurePipeline {
    fn from_world(world: &mut World) -> Self {
        let mut shaders = world
            .get_resource_mut::<Assets<Shader>>()
            .expect("no shaders");
        let vertex = shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("../assets/treasure.vert"),
        ));
        let fragment = shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("../assets/treasure.frag"),
        ));
        let mut pipelines = world
            .get_resource_mut::<Assets<PipelineDescriptor>>()
            .expect("no pipelines");
        let handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
            vertex,
            fragment: Some(fragment),
        }));
        TreasurePipeline(handle)
    }
}

pub struct TreasurePlugin;
impl Plugin for TreasurePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TreasurePipeline>().add_system_set(
            SystemSet::on_update(GameState::Level)
                .with_system(spawn_treasure.system())
                .with_system(treasure_collide.system()),
        );
    }
}
