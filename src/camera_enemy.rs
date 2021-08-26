use bevy::{math::Vec2, prelude::*};

use crate::{
    perlin::{PerlinBundle, PerlinPipelineHandle},
    GameState,
};

/// (position, start_a, radius)
#[derive(Debug, Clone, Copy)]
pub struct CameraSpawn {
    pub x: f32,
    pub y: f32,
    pub start_angle: f32,
    pub end_angle: f32,
    pub radius: f32,
}

const NOISE_RESOLUTION: f32 = 3000.;

#[derive(Debug)]
struct Camera {
    // in radians
    start_angle: f32,
    // in radians
    end_angle: f32,
    radius: f32,
}

fn spawn_camera(
    mut commands: Commands,
    spawns: Res<Vec<CameraSpawn>>,
    pp_handle: Res<PerlinPipelineHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    dbg!(&spawns as &Vec<_>);
    // x is used for transparency going further from start
    // let uv = vec![[1.0, 0.0], [0., 0.], [0., 0.]];
    let uv = vec![1., 0.5, 0.5];
    let indices = vec![0, 1, 2];
    for spawn in spawns.iter() {
        let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
        let mut v_pos = vec![[0., 0., 0.]];
        let angles = [spawn.start_angle, spawn.end_angle];
        let radius = spawn.radius;
        for angle in angles {
            // rotating (radius, 0)
            let x = angle.cos() * radius;
            let y = angle.sin() * radius;
            v_pos.push([x, y, 0.]);
        }
        dbg!(&v_pos);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices.clone())));
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv.clone());

        commands
            .spawn_bundle(MeshBundle {
                mesh: meshes.add(mesh),
                transform: Transform::from_xyz(spawn.x, spawn.y, 0.2),
                ..Default::default()
            })
            .insert_bundle(PerlinBundle::new(&pp_handle, Vec2::splat(NOISE_RESOLUTION)));
    }
}

pub struct EnemyCameraPlugin;
impl Plugin for EnemyCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Vec<CameraSpawn>>().add_system_set(
            SystemSet::on_enter(GameState::Level).with_system(spawn_camera.system()),
        );
    }
}
