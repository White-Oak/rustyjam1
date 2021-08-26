use bevy::{
    math::{Mat2, Vec2, Vec3Swizzles},
    prelude::*,
};

use crate::{GameState, perlin::{NoiseColorComponent, PerlinBundle, PerlinPipelineHandle}, player::{Dashing, Player}};

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
    for spawn in spawns.iter() {
    let uv = vec![1., 0.5, 0.5];
    let indices = vec![0, 1, 2];
        let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
        let mut v_pos = vec![[0., 0.]];
        let angles = [spawn.start_angle, spawn.end_angle];
        let radius = spawn.radius;
        let origin = Vec2::new(radius, 0.);
        for angle in angles {
            let rotation_mat = Mat2::from_angle(angle);
            let res = rotation_mat * origin;
            v_pos.push(res.into());
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
            .insert(Camera {
                start_angle: spawn.start_angle,
                end_angle: spawn.end_angle,
                radius: spawn.radius,
            })
            .insert_bundle(PerlinBundle::new(&pp_handle, Vec2::splat(NOISE_RESOLUTION), base_color()));
    }
}

fn base_color() -> Vec3 {
    Vec3::new(0.8, 0.8, 0.)
}

fn detected_color() -> Vec3 {
    Vec3::new(0.9, 0.1, 0.)
}

fn detect_player(
    cameras: Query<(&Camera, &Transform, &mut NoiseColorComponent)>,
    player: Query<&Transform, (With<Player>, Without<Dashing>)>,
) {
    let player_tr = if let Ok(x) = player.single(){
        x.translation.xy()
    } else {
        return
    };
    let origin = Vec2::new(1., 0.);
    let base_color = base_color();
    let detected_color = detected_color();
    cameras.for_each_mut(|(cam, tr, mut color)| {
        let mut is_base_color = true;
        let tr = tr.translation.xy();
        let player_tr = player_tr - tr;
        let angle = origin.angle_between(player_tr);
        if angle > cam.start_angle && angle < cam.end_angle {
            let dist = player_tr.length_squared();
            if dist < cam.radius * cam.radius {
                is_base_color = false;
            }
        }
        if is_base_color {
            color.value = base_color;
        } else {
            color.value = detected_color;
        }
    })
}

pub struct EnemyCameraPlugin;
impl Plugin for EnemyCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Vec<CameraSpawn>>()
            .add_system_set(
                SystemSet::on_enter(GameState::Level).with_system(spawn_camera.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Level).with_system(detect_player.system()),
                // SystemSet::new()
                //     .with_run_criteria(FixedTimestep::steps_per_second(1.))
                //     .with_system(detect_player.system()),
            );
    }
}
