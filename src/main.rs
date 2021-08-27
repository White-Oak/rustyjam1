mod camera_enemy;
mod light_radius;
mod map;
mod movement;
mod perlin;
mod player;
mod smoke_bomb;
mod ui;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::{widgets::InspectorQuery, };
use camera_enemy::EnemyCameraPlugin;
use light_radius::LightRadiusPlugin;
use map::MapPlugin;
use perlin::PerlinPlugin;
use smoke_bomb::SmokeBombPlugin;
use ui::UiPlugin;

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
            width: 1920.0 * 0.8,
            height: 1080.0 * 0.8,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        // .add_plugin(InspectorPlugin::<InspectorQuery<&mut PerlinComponent>>::new())
        // .add_plugin(InspectorPlugin::<InspectorQuery<&mut NoiseColorComponent>>::new())
        .add_state(GameState::LoadingLevel)
        .add_plugin(PlayerPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(TiledMapPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PerlinPlugin)
        .add_plugin(EnemyCameraPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(SmokeBombPlugin)
        .add_plugin(LightRadiusPlugin)
        .add_startup_system(setup.system())
        .run();
}

pub struct MainCamera;
fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
