use bevy::{
    math::{Mat2, Vec2, Vec3Swizzles},
    prelude::*,
};

use crate::{
    perlin::{NoiseColorComponent, PerlinBundle, PerlinPipelineHandle},
    player::{Dashing, Player},
    smoke_bomb::SmokeBomb,
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

const NOISE_RESOLUTION: f32 = 2000.;
const NOISE_OCTAVE: f32 = 0.15;
const TRANSPARENCY_BASES: [f32; 3] = [0.8, 0.2, 0.2];
const CAMERA_INDICES: [u32; 3] = [0, 1, 2];

#[derive(Debug)]
struct Camera {
    // in radians
    start_angle: f32,
    // in radians
    end_angle: f32,
    radius: f32,
    // TODO: I don't really need any of the above fields
    points: [Vec2; 3],
}

fn spawn_camera(
    mut commands: Commands,
    spawns: Res<Vec<CameraSpawn>>,
    pp_handle: Res<PerlinPipelineHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // x is used for transparency going further from start
    for spawn in spawns.iter() {
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
        let points: [Vec2; 3] = [v_pos[0].into(), v_pos[1].into(), v_pos[2].into()];
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(
            CAMERA_INDICES.to_vec(),
        )));
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, TRANSPARENCY_BASES.to_vec());

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
                points,
            })
            .insert_bundle(PerlinBundle::new(
                &pp_handle,
                NOISE_RESOLUTION,
                NOISE_OCTAVE,
                base_color(),
            ));
    }
}

fn base_color() -> Vec3 {
    Vec3::new(0.8, 0.8, 0.)
}

fn detected_color() -> Vec3 {
    Vec3::new(0.9, 0.1, 0.)
}

// https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
fn is_in_triangle(s: Vec2, triangle: [Vec2; 3]) -> bool {
    let [a, b, c] = triangle;
    let as_x = s.x - a.x;
    let as_y = s.y - a.y;

    let s_ab = (b.x - a.x) * as_y - (b.y - a.y) * as_x > 0.;
    let s_ac = (c.x - a.x) * as_y - (c.y - a.y) * as_x > 0.;

    if s_ac == s_ab {
        return false;
    }

    let last = (c.x - b.x) * (s.y - b.y) - (c.y - b.y) * (s.x - b.x) > 0.;

    last == s_ab
}

fn detect_player(
    cameras: Query<(&Camera, &Transform, &mut NoiseColorComponent)>,
    smoke_bombs: Query<(&SmokeBomb, &Transform)>,
    player: Query<&Transform, (With<Player>, Without<Dashing>)>,
) {
    let player_tr = if let Ok(x) = player.single() {
        x.translation.xy()
    } else {
        return;
    };
    let mut is_smoked = false;
    smoke_bombs.for_each(|(bomb, tr)| {
        let dist = player_tr - tr.translation.xy();
        if dist.length_squared() < bomb.radius * bomb.radius {
            is_smoked = true;
            // TODO: break early?
        }
    });
    let base_color = base_color();
    let detected_color = detected_color();
    cameras.for_each_mut(|(cam, tr, mut color)| {
        let tr = tr.translation.xy();
        let player_tr = player_tr - tr;
        if !is_smoked && is_in_triangle(player_tr, cam.points) {
            color.value = detected_color;
        } else {
            color.value = base_color;
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
