use bevy::{log, prelude::*};
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
struct RewardRare(Handle<ColorMaterial>);
struct RewardMagic(Handle<ColorMaterial>);

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

impl FromWorld for RewardRare {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("no assets server");
        let handle = asset_server.load("reward4.png");
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("no materials");
        let handle = materials.add(ColorMaterial::texture(handle));
        RewardRare(handle)
    }
}

impl FromWorld for RewardMagic {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("no assets server");
        let handle = asset_server.load("reward2.png");
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("no materials");
        let handle = materials.add(ColorMaterial::texture(handle));
        RewardMagic(handle)
    }
}

fn generate_rewards(
    mut commands: Commands,
    common: Res<RewardCommon>,
    magic: Res<RewardMagic>,
    rare: Res<RewardRare>,
    pp_handle: Res<PerlinPipelineHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    font: Res<RobotoFont>,
    player: Query<&Transform, With<Player>>,
) {
    let items = generate();
    let tr = player.single().expect("single player").translation;
    commands.insert_resource(RewardItems(items.clone()));

    let v_pos = vec![
        [-5000., -5000.],
        [5000., -5000.],
        [5000., 5000.],
        [-5000., 5000.],
    ];
    let uv = [0.3; 4].to_vec();
    let indices = vec![0, 1, 2, 0, 2, 3];
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    let mut tr = tr.clone();
    tr.z += 0.5;
    let mut tr = Transform::from_translation(tr);
    tr.scale = (Vec2::splat(0.3), 0.).into();
    commands
        .spawn()
        .insert(RewardMarker)
        .insert(GlobalTransform::default())
        .insert_bundle(MeshBundle {
            mesh: meshes.add(mesh),
            transform: tr,
            ..Default::default()
        })
        .insert_bundle(PerlinBundle::new(
            &pp_handle,
            1000.,
            0.3,
            Vec3::new(0.2, 0.2, 0.02),
        ))
        .with_children(|cmds| {
            for (i, item) in items.into_iter().enumerate() {
                let sprite = Sprite::new(Vec2::new(320., 578.) * 2.);
                let texture = match item.mods.len() {
                    1 => common.0.clone(),
                    2 => common.0.clone(),
                    4 => common.0.clone(),
                    _ => todo!(),
                };
                let x = (i as f32 - 1.) * 800.;
                cmds.spawn_bundle(SpriteBundle {
                    sprite,
                    material: texture,
                    visible: Visible {
                        is_visible: true,
                        is_transparent: false,
                    },
                    transform: Transform::from_xyz(x, 0., 0.1),
                    ..Default::default()
                })
                .with_children(|cmds| {
                    let name = Text::with_section(
                        item.name.clone(),
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 21.0,
                            color: light_text_color(),
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    );
                    cmds.spawn_bundle(Text2dBundle {
                        text: name,
                        transform: Transform::from_xyz(0., 400., 0.001),
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
                            font_size: 15.0,
                            color: light_text_color(),
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    );
                    cmds.spawn_bundle(Text2dBundle {
                        text: item_mods,
                        transform: Transform::from_xyz(0., -200., 0.001),
                        ..Default::default()
                    });

                    let select = Text::with_section(
                        "Select".to_string(),
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 16.,
                            color: light_text_color(),
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    );
                    cmds.spawn_bundle(Text2dBundle {
                        text: select,
                        transform: Transform::from_xyz(0., -450., 0.001),
                        ..Default::default()
                    })
                    .with_children(|cmds| {
                        cmds.spawn_bundle(MyButtonBundle {
                            button: MyButton {
                                size: Vec2::new(84., 30.),
                                id: ClickedReward(i),
                            },
                            transform: Transform::from_xyz(0., 0., 0.001),
                            ..Default::default()
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

pub struct RewardPlugin;
impl Plugin for RewardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<RewardCommon>()
            .init_resource::<RewardMagic>()
            .init_resource::<RewardRare>()
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
