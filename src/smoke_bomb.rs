
use std::iter::repeat;

use bevy::{
    math::{Mat2, Vec2},
    prelude::*,
};
use itertools::Itertools;

use crate::{GameState, perlin::{PerlinBundle, PerlinPipelineHandle}};

pub struct SmokeBomb;

fn base_color() -> Vec3 {
    Vec3::splat(0.1)
}

fn spawn_smoke(
    mut commands: Commands,
    query: Query<(Entity,&Transform), Added<SmokeBomb>>,
    pp_handle: Res<PerlinPipelineHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    query.for_each(|(entity, tr)| {
        println!("GO GO GO GO");
        let mut v_pos = vec![[0., 0.]];
        let radius = 100.;
        let origin = Vec2::new(radius, 0.);
        let mut indices = vec![];
        let divisions = 360;
        for angle in 0..divisions {
            let angle = (angle as f32).to_radians();
            let rotation_mat = Mat2::from_angle(angle);
            let res = rotation_mat * origin;
            v_pos.push(res.into());
        }
        for (prev, next) in (1..(divisions - 1)).tuple_windows() {
            indices.extend_from_slice(&[prev as u32, next as u32, 0]);
        }
        indices.extend_from_slice(&[360, 1, 0]);
        let uv: Vec<_> = repeat(1.).take(divisions + 1).collect();
        let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices.clone())));
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);
        let mut tr = *tr;
        tr.translation.z -= 0.1;
        commands
            .entity(entity)
            .insert_bundle(MeshBundle {
                mesh: meshes.add(mesh),
                transform: tr,
                ..Default::default()
            })
            .insert_bundle(PerlinBundle::new(
                &pp_handle,
                Vec2::splat(100.),
                base_color(),
            ));
    });
}

pub struct SmokeBombPlugin;
impl Plugin for SmokeBombPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Level).with_system(spawn_smoke.system()),
            );
    }
}
