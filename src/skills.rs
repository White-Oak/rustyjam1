use std::time::Duration;

use bevy::{asset::Asset, prelude::*};

use crate::{cleanup::cleanup_system, GameState};

const BASE_WIDTH: f32 = 352. / 2560. * 100. * 0.75;
const BASE_HEIGHT: f32 = 227. / 1440. * 100. * 0.75;

struct SkillsUiHandles {
    layout: Handle<ColorMaterial>,
    dash: Handle<ColorMaterial>,
    dash_gray: Handle<ColorMaterial>,
    bomb: Handle<ColorMaterial>,
    bomb_gray: Handle<ColorMaterial>,
    none_material: Handle<ColorMaterial>,
}

fn get_handle(world: &mut World, path: &str) -> Handle<ColorMaterial> {
    let asset_server = world
        .get_resource::<AssetServer>()
        .expect("no assets server");
    let handle = asset_server.load(path);
    let mut materials = world
        .get_resource_mut::<Assets<ColorMaterial>>()
        .expect("no materials");
    let handle = materials.add(ColorMaterial::texture(handle));
    handle
}

fn get_handle_gray(world: &mut World, path: &str) -> Handle<ColorMaterial> {
    let asset_server = world
        .get_resource::<AssetServer>()
        .expect("no assets server");
    let handle = asset_server.load(path);
    let mut materials = world
        .get_resource_mut::<Assets<ColorMaterial>>()
        .expect("no materials");
    let handle = materials.add(ColorMaterial::modulated_texture(handle, Color::GRAY));
    handle
}

impl FromWorld for SkillsUiHandles {
    fn from_world(world: &mut World) -> Self {
        let layout = get_handle(world, "skills_ui.png");
        let dash = get_handle(world, "skills_q.png");
        let dash_gray = get_handle_gray(world, "skills_q.png");
        let bomb = get_handle(world, "skills_e.png");
        let bomb_gray = get_handle_gray(world, "skills_e.png");
        let mut color_materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("no materials");
        let none_material = color_materials.add(Color::NONE.into());
        SkillsUiHandles {
            layout,
            dash,
            bomb,
            none_material,
            dash_gray,
            bomb_gray,
        }
    }
}

#[derive(Debug, Default)]
struct SkillsState {
    q: SkillState,
    e: SkillState,
}

#[derive(Debug, Default)]
struct SkillState {
    time_to_cd: Option<Duration>,
}

struct SkillsUiMarker;

fn setup(
    mut commands: Commands,
    textures: Res<SkillsUiHandles>,
    asset_server: ResMut<AssetServer>,
) {
    let mut ui_bundle = commands.spawn_bundle(UiCameraBundle::default());
    let ui_cmds = ui_bundle // root node
        .commands();
    let material = textures.none_material.clone();
    let font_handle = asset_server.load("FiraSans-Bold.ttf");
    let text = Text::with_section(
        "FPS: ".to_string(),
        TextStyle {
            font: font_handle.clone(),
            font_size: 30.0,
            color: Color::WHITE,
        },
        TextAlignment {
            vertical: VerticalAlign::Top,
            horizontal: HorizontalAlign::Left,
        },
    );
    ui_cmds
        // .spawn()
        // .insert(Transform::default())
        // .insert(GlobalTransform::default())
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(0.),
                    bottom: Val::Px(0.),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Percent(BASE_WIDTH),
                    height: Val::Percent(BASE_HEIGHT),
                },
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .insert(SkillsState::default())
        .with_children(|ec| {
            ec.spawn_bundle(ImageBundle {
                material: textures.layout.clone(),
                style: Style {
                    min_size: Size {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|ec| {
                for s in ["Q", "E"] {
                    ec.spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|ec| {
                        ec.spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size {
                                    height: Val::Percent(80.),
                                    width: Val::Auto,
                                },
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|ec| {
                            let text = Text::with_section(
                                s.to_string(),
                                TextStyle {
                                    font: font_handle.clone(),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                                TextAlignment {
                                    vertical: VerticalAlign::Top,
                                    horizontal: HorizontalAlign::Left,
                                },
                            );
                            ec.spawn_bundle(TextBundle {
                                text,
                                ..Default::default()
                            });
                        });
                    });
                }
            });
        });
}

pub struct SkillsUiPlugin;
impl Plugin for SkillsUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SkillsUiHandles>()
            .add_system_set(SystemSet::on_enter(GameState::Level).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_exit(GameState::Level)
                    .with_system(cleanup_system::<SkillsState>.system()),
            );
    }
}
