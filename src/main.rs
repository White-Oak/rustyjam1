mod button;
mod camera_enemy;
mod cleanup;
mod inventory;
mod items;
mod light_radius;
mod main_menu_ui;
mod map;
mod movement;
mod perlin;
mod player;
mod reward;
mod smoke_bomb;
mod stats_screen;
mod treasure;
mod ui;
mod skills;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use button::MyButtonPlugin;
use camera_enemy::EnemyCameraPlugin;
use inventory::InventoryScreenPlugin;
use light_radius::LightRadiusPlugin;
use main_menu_ui::MainMenuUiPlugin;
use map::MapPlugin;
use perlin::PerlinPlugin;
use reward::RewardPlugin;
use skills::SkillsUiPlugin;
use smoke_bomb::SmokeBombPlugin;
use stats_screen::StatsScreenPlugin;
use treasure::TreasurePlugin;
use ui::UiPlugin;

use crate::{movement::MovementPlugin, player::PlayerPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    StatsScreen,
    InventoryScreen,
    LoadingLevel,
    Level,
    ChoosingTreasure,
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
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new().filter::<With<Node>>())
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
        .add_plugin(MyButtonPlugin)
        .add_plugin(StatsScreenPlugin)
        .add_plugin(InventoryScreenPlugin)
        .add_plugin(TreasurePlugin)
        .add_plugin(RewardPlugin)
        .add_plugin(SkillsUiPlugin)
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
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("no assets server");
        // let handle = asset_server.load("Roboto-Regular.ttf");
        let handle = asset_server.load("FiraSans-Bold.ttf");
        RobotoFont(handle)
    }
}
