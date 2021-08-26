use bevy::{math::Vec2, prelude::*};
use bevy_ecs_tilemap::TiledMap;

use crate::{map::Boundaries, GameState, MainCamera};

/// Only uses x, y.
#[derive(Debug, Default)]
pub struct Velocity(pub Vec2);

fn move_system(
    query: Query<(&mut Transform, &Velocity), Without<MainCamera>>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
    boundaries: Res<Boundaries>,
) {
    let mut cam = camera.single_mut().expect("camera doesnt exist");
    query.for_each_mut(|(mut trnsf, velocity)| {
        let mut x = trnsf.translation.x;
        let mut y = trnsf.translation.y;
        if boundaries
            .collide(Vec3::new(x + velocity.0.x, y, 0.))
            .is_none()
        {
            x += velocity.0.x;
        }
        if boundaries
            .collide(Vec3::new(x, y + velocity.0.y, 0.))
            .is_none()
        {
            y += velocity.0.y;
        }
        trnsf.translation.x = x;
        trnsf.translation.y = y;
        cam.translation.x = x;
        cam.translation.y = y;
    });
}


pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Level)
                .with_system(move_system.system())
                .after("control"),
        );
    }
}
