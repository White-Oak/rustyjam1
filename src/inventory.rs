use bevy::{log, prelude::*};
use itertools::Itertools;

use crate::{
    button::{register_my_button, ClickedButtonEvent, MyButton, MyButtonBundle},
    cleanup::cleanup_system,
    items::{Item, PlayerItems, Slot},
    main_menu_ui::{change_camera_scale_from_resize, light_text_color},
    perlin::{PerlinBundle, PerlinPipelineHandle},
    GameState, RobotoFont,
};

struct InventoryScreenMarker;

#[derive(Debug, Clone, Copy, Default)]
struct ClickedItem(usize);

#[derive(Debug, Clone, Copy, Default)]
struct DeletedItem(usize);

struct UiTexture(Handle<ColorMaterial>);
struct UiCardTexture(Handle<ColorMaterial>);

#[derive(Debug, Clone, Copy, Default)]
struct ClickedBack;

#[derive(Debug, Clone, Copy, Default)]
struct ClickedNext;

#[derive(Debug, Clone, Copy, Default)]
struct ClickedPrev;

#[derive(Debug, Default)]
pub struct ViewInvSlot(pub Slot, pub u32);

struct CurrentItemsView(Entity);

const ITEMS_ON_PAGE: u32 = 9;

fn setup(
    mut commands: Commands,
    pp_handle: Res<PerlinPipelineHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    ui_texture: Res<UiTexture>,
    font: Res<RobotoFont>,
) {
    let v_pos = vec![
        [-2000., -2000.],
        [2000., -2000.],
        [2000., 2000.],
        [-2000., 2000.],
    ];
    let uv = [0.3; 4].to_vec();
    let indices = vec![0, 1, 2, 0, 2, 3];
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    commands
        .spawn()
        .insert(InventoryScreenMarker)
        .insert(GlobalTransform::default())
        .insert_bundle(MeshBundle {
            mesh: meshes.add(mesh),
            transform: Transform::from_xyz(0., 0., 0.6),
            ..Default::default()
        })
        .insert_bundle(PerlinBundle::new(
            &pp_handle,
            1500.,
            0.2,
            Vec3::new(0.09, 0.09, 0.09),
        ))
        .with_children(|cmds| {
            let sprite = Sprite::new(Vec2::new(950., 804.) * 1.2);
            cmds.spawn_bundle(SpriteBundle {
                sprite,
                material: ui_texture.0.clone(),
                transform: Transform::from_xyz(0., 0., 0.001),
                ..Default::default()
            })
            .with_children(|cmds| {
                let header = Text::with_section(
                    "All items".to_string(),
                    TextStyle {
                        font: font.0.clone(),
                        font_size: 50.0,
                        color: light_text_color(),
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..Default::default()
                    },
                );
                cmds.spawn_bundle(Text2dBundle {
                    text: header,
                    transform: Transform::from_xyz(0., 381., 0.001),
                    ..Default::default()
                });

                // low buttons
                cmds.spawn()
                    .insert(Transform::from_xyz(0., -423., 0.001))
                    .insert(GlobalTransform::default())
                    .with_children(|cmds| {
                        let ok_text = Text::with_section(
                            "OK".to_string(),
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 24.0,
                                color: light_text_color(),
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        );
                        cmds.spawn_bundle(Text2dBundle {
                            text: ok_text,
                            transform: Transform::from_xyz(0., 0., 0.001),
                            ..Default::default()
                        })
                        .with_children(|cmds| {
                            cmds.spawn_bundle(MyButtonBundle {
                                button: MyButton {
                                    size: Vec2::new(80., 30.),
                                    id: ClickedBack,
                                },
                                transform: Transform::from_xyz(0., 0., 0.0001),
                                ..Default::default()
                            });
                        });

                        let next_text = Text::with_section(
                            "Next".to_string(),
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 24.0,
                                color: light_text_color(),
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        );
                        cmds.spawn_bundle(Text2dBundle {
                            text: next_text,
                            transform: Transform::from_xyz(477., 0., 0.001),
                            ..Default::default()
                        })
                        .with_children(|cmds| {
                            cmds.spawn_bundle(MyButtonBundle {
                                button: MyButton {
                                    size: Vec2::new(80., 30.),
                                    id: ClickedNext,
                                },
                                transform: Transform::from_xyz(0., 0., 0.0001),
                                ..Default::default()
                            });
                        });

                        let prev_text = Text::with_section(
                            "Prev".to_string(),
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 24.0,
                                color: light_text_color(),
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        );
                        cmds.spawn_bundle(Text2dBundle {
                            text: prev_text,
                            transform: Transform::from_xyz(-477., 0., 0.001),
                            ..Default::default()
                        })
                        .with_children(|cmds| {
                            cmds.spawn_bundle(MyButtonBundle {
                                button: MyButton {
                                    size: Vec2::new(80., 30.),
                                    id: ClickedPrev,
                                },
                                transform: Transform::from_xyz(0., 0., 0.0001),
                                ..Default::default()
                            });
                        });
                    });
            });
        });
}

fn draw_slot(
    cmds: &mut ChildBuilder,
    item: &Item,
    font: &RobotoFont,
    index: usize,
    is_equipped: bool,
) {
    let color = light_text_color();

    //name
    let name = Text::with_section(
        item.name.clone(),
        TextStyle {
            font: font.0.clone(),
            font_size: 21.,
            color,
        },
        TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        },
    );
    cmds.spawn_bundle(Text2dBundle {
        text: name,
        transform: Transform::from_xyz(44., 80., 0.001),
        ..Default::default()
    });

    //mods
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
            color,
        },
        TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        },
    );
    cmds.spawn_bundle(Text2dBundle {
        text: item_mods,
        transform: Transform::from_xyz(40., 12., 0.001),
        ..Default::default()
    });

    if !is_equipped {
        // buttons
        let select = Text::with_section(
            "Select".to_string(),
            TextStyle {
                font: font.0.clone(),
                font_size: 16.,
                color,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        );
        cmds.spawn_bundle(Text2dBundle {
            text: select,
            transform: Transform::from_xyz(-91., -57., 0.001),
            ..Default::default()
        })
        .with_children(|cmds| {
            cmds.spawn_bundle(MyButtonBundle {
                button: MyButton {
                    size: Vec2::new(84., 30.),
                    id: ClickedItem(index),
                },
                transform: Transform::from_xyz(0., 0., 0.001),
                ..Default::default()
            });
        });

        let delet = Text::with_section(
            "Delete".to_string(),
            TextStyle {
                font: font.0.clone(),
                font_size: 16.,
                color,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        );
        cmds.spawn_bundle(Text2dBundle {
            text: delet,
            transform: Transform::from_xyz(91., -57., 0.001),
            ..Default::default()
        })
        .with_children(|cmds| {
            cmds.spawn_bundle(MyButtonBundle {
                button: MyButton {
                    size: Vec2::new(84., 30.),
                    id: DeletedItem(index),
                },
                transform: Transform::from_xyz(0., 0., 0.001),
                ..Default::default()
            });
        });
    }
}

fn dispatch_items(
    mut commands: Commands,
    mut cur_view: ResMut<Option<CurrentItemsView>>,
    view: Res<ViewInvSlot>,
    items: Res<PlayerItems>,
    texture: Res<UiCardTexture>,
    font: Res<RobotoFont>,
) {
    if !(view.is_changed() || items.is_changed()) && cur_view.is_some() {
        return;
    }
    if let Some(CurrentItemsView(entity)) = cur_view.take() {
        commands.entity(entity).despawn_recursive();
    }
    let next_view = commands
        .spawn()
        .insert(InventoryScreenMarker)
        .insert(Transform::from_xyz(0., -33., 0.7))
        .insert(GlobalTransform::default())
        .with_children(|cmds| {
            let min_el = view.1 as usize;
            let slot_items = items.slot_items(view.0);
            for (orig_i, item) in slot_items
                .available
                .iter()
                .enumerate()
                .skip(min_el)
                .take(ITEMS_ON_PAGE as usize)
            {
                let i = (orig_i - min_el) as i32;
                let x = i % 3 - 1;
                let y = -(i / 3 - 1);
                let x = x as f32 * 340.;
                let y = y as f32 * 232.;
                cmds.spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::new(270., 180.) * 1.2),
                    transform: Transform::from_xyz(x, y, 0.001),
                    material: texture.0.clone(),
                    ..Default::default()
                })
                .with_children(|cmds| {
                    draw_slot(cmds, item, &font, orig_i, slot_items.equipped == orig_i);
                });
            }
        })
        .id();
    *cur_view = Some(CurrentItemsView(next_view));
}

fn clicked_back(
    mut event_reader: EventReader<ClickedButtonEvent<ClickedBack>>,
    mut state: ResMut<State<GameState>>,
) {
    if event_reader.iter().next().is_some() {
        log::debug!("moving back to menu");
        state.pop().expect("cant move back from stats screen");
    }
}

fn clicked_prev(
    mut event_reader: EventReader<ClickedButtonEvent<ClickedPrev>>,
    mut cur_view: ResMut<ViewInvSlot>,
) {
    if event_reader.iter().next().is_some() {
        log::debug!("moving back a page");
        if cur_view.1 < ITEMS_ON_PAGE {
            log::debug!("already back enough");
        } else {
            cur_view.1 -= ITEMS_ON_PAGE;
        }
    }
}

fn clicked_next(
    mut event_reader: EventReader<ClickedButtonEvent<ClickedNext>>,
    mut cur_view: ResMut<ViewInvSlot>,
    items: Res<PlayerItems>,
) {
    if event_reader.iter().next().is_some() {
        log::debug!("moving next a page");
        if cur_view.1 + ITEMS_ON_PAGE
            > items.all_available_items_for_slot(cur_view.0).count() as u32
        {
            log::debug!("already next enough");
        } else {
            cur_view.1 += ITEMS_ON_PAGE;
        }
    }
}

fn clicked_select(
    mut event_reader: EventReader<ClickedButtonEvent<ClickedItem>>,
    cur_view: Res<ViewInvSlot>,
    mut items: ResMut<PlayerItems>,
    mut state: ResMut<State<GameState>>,
) {
    if let Some(ClickedButtonEvent(ClickedItem(index))) = event_reader.iter().next() {
        log::debug!("selecting an item");
        items.equip_on_slot(cur_view.0, *index);
        log::debug!("moving back to menu");
        state.pop().expect("cant move back from stats screen");
    }
}

fn clicked_delete(
    mut event_reader: EventReader<ClickedButtonEvent<DeletedItem>>,
    cur_view: Res<ViewInvSlot>,
    mut items: ResMut<PlayerItems>,
) {
    if let Some(ClickedButtonEvent(DeletedItem(index))) = event_reader.iter().next() {
        log::debug!("deleting an item");
        items.delete_for_slot(cur_view.0, *index);
    }
}

impl FromWorld for UiTexture {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("no assets server");
        let handle = asset_server.load("inv.png");
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("no materials");
        let handle = materials.add(ColorMaterial::texture(handle));
        UiTexture(handle)
    }
}

impl FromWorld for UiCardTexture {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("no assets server");
        let handle = asset_server.load("inv_card.png");
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("no materials");
        let handle = materials.add(ColorMaterial::texture(handle));
        UiCardTexture(handle)
    }
}

pub struct InventoryScreenPlugin;
impl Plugin for InventoryScreenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InventoryScreen).with_system(setup.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::InventoryScreen)
                .with_system(cleanup_system::<InventoryScreenMarker>.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InventoryScreen)
                .with_system(clicked_back.system().after("button_click"))
                .with_system(
                    clicked_next
                        .system()
                        .after("button_click")
                        .before("dispatch_inventory"),
                )
                .with_system(
                    clicked_prev
                        .system()
                        .after("button_click")
                        .before("dispatch_inventory"),
                )
                .with_system(
                    clicked_delete
                        .system()
                        .after("button_click")
                        .before("dispatch_inventory"),
                )
                .with_system(
                    clicked_select
                        .system()
                        .after("button_click")
                        .before("dispatch_inventory"),
                )
                .with_system(dispatch_items.system().label("dispatch_inventory"))
                .with_system(change_camera_scale_from_resize.system()),
        )
        .init_resource::<UiTexture>()
        .init_resource::<UiCardTexture>()
        .init_resource::<ViewInvSlot>()
        .init_resource::<Option<CurrentItemsView>>();
        register_my_button::<ClickedBack>(app, GameState::InventoryScreen);
        register_my_button::<ClickedNext>(app, GameState::InventoryScreen);
        register_my_button::<ClickedPrev>(app, GameState::InventoryScreen);
        register_my_button::<ClickedItem>(app, GameState::InventoryScreen);
        register_my_button::<DeletedItem>(app, GameState::InventoryScreen);
    }
}
