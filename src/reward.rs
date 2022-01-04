use std::f32::consts::PI;

use bevy::{log, math::Mat2, prelude::*};
use itertools::Itertools;

use crate::{
    button::{register_my_button, ClickedButtonEvent, MyButton, MyButtonBundle},
    cleanup::cleanup_system,
    items::{generate, Item, PlayerItems},
    main_menu_ui::light_text_color,
    perlin::{PerlinBundle, PerlinPipelineHandle},
    player::Player,
    GameState, RobotoFont,
};

struct RewardCommon(Handle<ColorMaterial>);

struct RewardMarker;

#[derive(Debug, Clone, Default)]

struct ClickedReward(usize);

#[derive(Debug, Clone, Default)]
struct RewardItems(Vec<Item>);

impl FromWorld for RewardCommon {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("no assets server");
        let handle = asset_server.load("reward1.png");
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("no materials");
        let handle = materials.add(ColorMaterial::texture(handle));
        RewardCommon(handle)
    }
}

fn generate_rewards(
    mut commands: Commands,
    common: Res<RewardCommon>,
    pp_handle: Res<PerlinPipelineHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    font: Res<RobotoFont>,
    player: Query<&Transform, With<Player>>,
) {
    let items = generate();
    let tr = player.single().expect("single player").translation;
    commands.insert_resource(RewardItems(items.clone()));

    let mesh = build_bg_mesh();
    let mut tr = tr;
    tr.z += 0.5;
    let tr = Transform::from_translation(tr);
    let mut ui_bundle = commands.spawn_bundle(UiCameraBundle::default());
    let ui_cmds = ui_bundle // root node
        .commands();
    let mut cmds = ui_cmds.spawn();
    cmds.insert(RewardMarker)
        .insert_bundle(MeshBundle {
            mesh: meshes.add(mesh),
            transform: tr,
            ..Default::default()
        })
        .insert_bundle(PerlinBundle::new(
            &pp_handle,
            1000.,
            0.09,
            Vec3::new(0.05, 0.05, 0.01),
        ))
        .with_children(|cmds| {
            for (i, item) in items.into_iter().enumerate() {
                let sprite = Sprite::new(Vec2::new(CARD_WIDTH, CARD_HEIGHT) * 2.);
                let texture = common.0.clone();
                let x = (i as f32 - 1.) * 500.;
                cmds.spawn_bundle(MeshBundle {
                    mesh: meshes.add(build_card_mesh()),
                    transform: Transform::from_xyz(x, 0., 0.1),
                    ..Default::default()
                })
                .insert_bundle(PerlinBundle::new(
                    &pp_handle,
                    CARD_SHADER_RESOLUTION,
                    CARD_SHADER_OCTAVE,
                    get_card_shader_color(item.mods.len()),
                ))
                .with_children(|cmds| {
                    cmds.spawn_bundle(SpriteBundle {
                        sprite,
                        material: texture,
                        visible: Visible {
                            is_visible: true,
                            is_transparent: true,
                        },
                        transform: Transform::from_xyz(0., 0., 0.1),
                        ..Default::default()
                    })
                    .with_children(|cmds| {
                        let name = Text::with_section(
                            item.name.clone(),
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 32.0,
                                color: light_text_color(),
                            },
                            TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                ..Default::default()
                            },
                        );
                        cmds.spawn_bundle(Text2dBundle {
                            text: name,
                            transform: Transform::from_xyz(0., 300., 0.001),
                            ..Default::default()
                        });

                        let text = item
                            .mods
                            .iter()
                            .map(|item_mod| item_mod.to_string())
                            .collect_vec()
                            .join("\n");
                        let item_mods = Text::with_section(
                            text,
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 20.0,
                                color: light_text_color(),
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        );
                        cmds.spawn_bundle(Text2dBundle {
                            text: item_mods,
                            transform: Transform::from_xyz(0., -100., 0.001),
                            ..Default::default()
                        });

                        let select = Text::with_section(
                            "Select".to_string(),
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 18.,
                                color: light_text_color(),
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        );
                        cmds.spawn_bundle(Text2dBundle {
                            text: select,
                            transform: Transform::from_xyz(0., -279., 0.001),
                            ..Default::default()
                        })
                        .with_children(|cmds| {
                            cmds.spawn_bundle(MyButtonBundle {
                                button: MyButton {
                                    size: Vec2::new(320., 60.),
                                    id: ClickedReward(i),
                                },
                                transform: Transform::from_xyz(0., 0., 0.001),
                                ..Default::default()
                            });
                        });
                    });
                });
            }
        });
}

fn clicked_select(
    mut event_reader: EventReader<ClickedButtonEvent<ClickedReward>>,
    mut items: ResMut<PlayerItems>,
    mut state: ResMut<State<GameState>>,
    rewards: Res<RewardItems>,
) {
    if let Some(ClickedButtonEvent(ClickedReward(index))) = event_reader.iter().next() {
        log::debug!("selecting a treasure");
        let item = rewards.0[*index].clone();
        items.slot_items_mut(item.slot).available.push(item);
        log::debug!("moving back to menu");
        state.pop().expect("cant move back from reward screen");
    }
}

const BG_SIZE: f32 = 5000.;

fn build_bg_mesh() -> Mesh {
    let v_pos = vec![
        [-BG_SIZE, -BG_SIZE],
        [BG_SIZE, -BG_SIZE],
        [BG_SIZE, BG_SIZE],
        [-BG_SIZE, BG_SIZE],
    ];
    let uv = [0.3; 4].to_vec();
    let indices = vec![0, 1, 2, 0, 2, 3];
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    mesh
}

const CARD_WIDTH: f32 = 192.;
const CARD_HEIGHT: f32 = 347.;
const CARD_MARGIN: f32 = 50.;
const CARD_ROUND_LEN: f32 = 40.;
const CARD_ROUND_COUNT: usize = 5;
const CARD_SHADER_RESOLUTION: f32 = 200.;
const CARD_SHADER_OCTAVE: f32 = 0.03;
const CARD_UV_EXTERIOR: f32 = -0.1;

fn build_card_mesh() -> Mesh {
    let w = CARD_WIDTH + CARD_MARGIN;
    let h = CARD_HEIGHT + CARD_MARGIN;
    let l = CARD_ROUND_LEN;
    #[rustfmt::skip]
    let corners = vec![
        ([-w, -h + l], [-w + l, -h], [-w + l, -h + l]),
        ([w - l, -h], [w, -h + l], [w - l, -h + l]),
        ([w, h - l], [w - l, h], [w - l, h - l]),
        ([-w + l, h], [-w, h - l], [-w + l, h - l]),
    ];

    let mut v_pos = vec![[0., 0.]];
    let mut uv = vec![2.];
    let mut indices = vec![];
    let mut count = 0;
    let mut last_end = None;
    for (start, end, circle_center) in corners.into_iter() {
        let start = Vec2::new(start[0], start[1]);
        v_pos.push([start.x, start.y]);
        uv.push(CARD_UV_EXTERIOR);
        if let Some(_) = last_end {
            indices.extend([0, count, count + 1]);
        }
        count += 1;
        let end = Vec2::new(end[0], end[1]);
        let angle_d = (PI / 2.) / CARD_ROUND_COUNT as f32;
        let circle_center = Vec2::new(circle_center[0], circle_center[1]);
        let start_to_center = start - circle_center;
        for i in 1..CARD_ROUND_COUNT {
            let target_angle = angle_d * i as f32;
            let rotation_mat = Mat2::from_angle(target_angle);
            let target = rotation_mat * start_to_center + circle_center;
            v_pos.push([target.x, target.y]);
            indices.extend([0, count, count + 1]);
            uv.push(CARD_UV_EXTERIOR);
            count += 1;
        }
        v_pos.push([end.x, end.y]);
        indices.extend([0, count, count + 1]);
        uv.push(CARD_UV_EXTERIOR);
        count += 1;
        last_end = Some(end);
    }
    indices.extend([0, count, 1]);
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    mesh
}

fn get_card_shader_color(mods_len: usize) -> Vec3 {
    match mods_len {
        1 => Vec3::new(0.1, 1., 0.1),
        2 => Vec3::new(0.1, 0.1, 1.),
        4 => Vec3::new(0.7, 0.7, 0.1),
        _ => unreachable!(),
    }
}

pub struct RewardPlugin;
impl Plugin for RewardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<RewardCommon>()
            .add_system_set(
                SystemSet::on_enter(GameState::ChoosingTreasure)
                    .with_system(generate_rewards.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::ChoosingTreasure)
                    .with_system(clicked_select.system().after("button_click")),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::ChoosingTreasure)
                    .with_system(cleanup_system::<RewardMarker>.system()),
            );
        register_my_button::<ClickedReward>(app, GameState::ChoosingTreasure);
    }
}
