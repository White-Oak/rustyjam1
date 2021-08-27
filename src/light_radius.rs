use std::iter::{once, repeat};

use bevy::{
    math::{Mat2, Vec2},
    prelude::*,
};
use itertools::Itertools;

use crate::{
    perlin::{PerlinBundle, PerlinPipelineHandle},
    player::Player,
    GameState,
};

pub struct SmokeBomb;
pub struct LightRadius;

fn base_color() -> Vec3 {
    Vec3::splat(0.)
}

fn spawn_smoke(
    mut commands: Commands,
    query: Query<Entity, Added<Player>>,
    pp_handle: Res<PerlinPipelineHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    query.for_each(|entity| {
        println!("GO GO GO GO");
        let mut v_pos = vec![[0., 0.]];
        let radius = 300.;
        let origin = Vec2::new(radius, 0.);
        let mut indices = vec![];
        let divisions = 180;
        let one_angle = 360. / (divisions as f32);
        for angle in 0..divisions {
            let angle = (angle as f32).to_radians() * one_angle;
            let rotation_mat = Mat2::from_angle(angle);
            let res = rotation_mat * origin;
            v_pos.push(res.into());
        }
        for (prev, next) in (1..=divisions).tuple_windows() {
            indices.extend_from_slice(&[prev as u32, next as u32, 0]);
        }
        indices.extend_from_slice(&[1, 0, divisions]);
        let uv: Vec<_> = once(-0.4)
            .chain(repeat(1.1).take(divisions as usize))
            .collect();
        let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices.clone())));
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);
        commands.entity(entity).with_children(|ec| {
            ec.spawn_bundle(MeshBundle {
                mesh: meshes.add(mesh),
                transform: Transform::from_translation(Vec3::new(0.,0., 0.2)),
                ..Default::default()
            })
            .insert_bundle(PerlinBundle::new(&pp_handle, 50., 0.1, base_color()));
        });
    });
}

pub struct LightRadiusPlugin;
impl Plugin for LightRadiusPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Level).with_system(spawn_smoke.system()),
        );
    }
}
