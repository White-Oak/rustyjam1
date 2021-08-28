mod camera_enemy;
mod light_radius;
mod map;
mod movement;
mod perlin;
mod player;
mod smoke_bomb;
mod ui;
mod main_menu_ui;
mod items;
mod button;
mod cleanup;
mod stats_screen;
mod inventory;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin, widgets::InspectorQuery};
use bevy_prototype_lyon::plugin::ShapePlugin;
use button::MyButtonPlugin;
use camera_enemy::EnemyCameraPlugin;
use inventory::InventoryScreenPlugin;
use light_radius::LightRadiusPlugin;
use main_menu_ui::MainMenuUiPlugin;
use map::MapPlugin;
use perlin::{NoiseColorComponent, PerlinPlugin};
use smoke_bomb::{SmokeBomb, SmokeBombPlugin};
use stats_screen::StatsScreenPlugin;
use ui::UiPlugin;

use crate::{movement::MovementPlugin, player::PlayerPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    StatsScreen,
    InventoryScreen,
    LoadingLevel,
    Level,
}

pub const WIDTH: f32 = 1920. * 0.9;
pub const HEIGHT: f32 = 1080. * 0.9;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        // .add_plugin(InspectorPlugin::<InspectorQuery<(Entity,)>>::new())
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(InspectorPlugin::<InspectorQuery<&mut NoiseColorComponent, With<SmokeBomb>>>::new())
        .add_state(GameState::MainMenu)
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
        .add_plugin(MainMenuUiPlugin)
        // .add_plugin(ShapePlugin)
        .add_plugin(MyButtonPlugin)
        .add_plugin(StatsScreenPlugin)
        .add_plugin(InventoryScreenPlugin)
        .add_startup_system(setup.system())
        .init_resource::<RobotoFont>()
        .run();
}

pub struct MainCamera;
fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}


pub struct RobotoFont(pub Handle<Font>);
impl FromWorld for RobotoFont {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().expect("no assets server");
        let handle = asset_server.load("Roboto-Regular.ttf");
        RobotoFont(handle)
    }
}