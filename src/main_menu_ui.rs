use bevy::{log, prelude::*, window::WindowResized};
use itertools::Itertools;

use crate::{
    button::{register_my_button, ClickedButtonEvent, MyButton, MyButtonBundle},
    cleanup::cleanup_system,
    inventory::ViewInvSlot,
    items::{Item, PlayerItems, PlayerStatsMods, Slot},
    GameState, MainCamera, RobotoFont, HEIGHT, WIDTH,
};

#[derive(Debug, Clone, Copy, Default)]
struct ClickedStats;

#[derive(Debug, Clone, Copy, Default)]
struct ClickedSlot(Slot);

#[derive(Debug, Clone, Copy, Default)]
struct ClickedLevel(u32);

#[derive(Debug, Clone, Copy, Default)]
pub struct SelectedLevel(pub u32);

struct MainMenuMarker;

struct CurrentItemsView(Entity);

pub fn light_text_color() -> Color {
    Color::rgb_u8(255, 252, 236)
}

fn draw_slot(cmds: &mut ChildBuilder, item: &Item, font_handle: &RobotoFont) {
    let y = match item.slot {
        crate::items::Slot::Head => 220.,
        crate::items::Slot::Cloak => 55.,
        crate::items::Slot::Lockpick => -115.,
        crate::items::Slot::Boots => -281.,
    };
    let x = 51.;
    let color = light_text_color();
    cmds.spawn_bundle(MyButtonBundle {
        button: MyButton {
            size: Vec2::new(344., 156.),
            id: ClickedSlot(item.slot),
        },
        transform: Transform::from_xyz(44., y, 0.001),
        ..Default::default()
    })
    .with_children(|cmds| {
        let name = Text::with_section(
            item.name.clone(),
            TextStyle {
                font: font_handle.0.clone(),
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
            transform: Transform::from_xyz(x, 55., 0.001),
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
                font: font_handle.0.clone(),
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
            transform: Transform::from_xyz(x, -20., 0.001),
            ..Default::default()
        });
    });
}

fn setup(
    mut cmds: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut camera: Query<&mut Transform, With<MainCamera>>
) {
    let mut cam = camera.single_mut().unwrap();
    cam.translation = Vec3::new(0., 0., 999.);
    cam.scale = Vec3::splat(1.);
    let font_handle = asset_server.load("FiraSans-Bold.ttf");

    let sprite = Sprite::new(Vec2::new(WIDTH, HEIGHT));
    let main_menu = materials.add(ColorMaterial::texture(asset_server.load("001.png")));
    cmds.spawn_bundle(SpriteBundle {
        sprite,
        material: main_menu,
        ..Default::default()
    });

    cmds.spawn()
        .insert(MainMenuMarker)
        .insert(Transform::from_xyz(-615., 0., 0.001))
        .insert(GlobalTransform::default())
        .with_children(|cmds| {
            let levels_header = Text::with_section(
                "Levels".to_string(),
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 60.0,
                    color: Color::rgb_u8(255, 252, 236),
                },
                Default::default(),
            );
            cmds.spawn_bundle(Text2dBundle {
                text: levels_header,
                transform: Transform::from_xyz(307., 340., 0.001),
                ..Default::default()
            });

            cmds.spawn_bundle(MyButtonBundle {
                button: MyButton {
                    size: Vec2::new(940., 156.),
                    id: ClickedLevel(1),
                },
                transform: Transform::from_xyz(330., 220., 0.001),
                ..Default::default()
            })
            .with_children(|cmds| {
                let level1 = Text::with_section(
                    "Level 1".to_string(),
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 50.0,
                        color: Color::rgb_u8(255, 252, 236),
                    },
                    Default::default(),
                );
                cmds.spawn_bundle(Text2dBundle {
                    text: level1,
                    transform: Transform::from_xyz(-330., 20., 0.001),
                    ..Default::default()
                });
            });

            cmds.spawn_bundle(MyButtonBundle {
                button: MyButton {
                    size: Vec2::new(940., 156.),
                    id: ClickedLevel(2),
                },
                transform: Transform::from_xyz(330., 45., 0.001),
                ..Default::default()
            })
            .with_children(|cmds| {
                let level1 = Text::with_section(
                    "Debug reward level".to_string(),
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 50.0,
                        color: Color::rgb_u8(255, 252, 236),
                    },
                    Default::default(),
                );
                cmds.spawn_bundle(Text2dBundle {
                    text: level1,
                    transform: Transform::from_xyz(-90., 20., 0.001),
                    ..Default::default()
                });
            });
        });

    cmds.spawn()
        .insert(MainMenuMarker)
        .insert(Transform::from_xyz(530., 0., 0.001))
        .insert(GlobalTransform::default())
        .with_children(|cmds| {
            let items_header = Text::with_section(
                "Items".to_string(),
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 60.0,
                    color: light_text_color(),
                },
                Default::default(),
            );
            cmds.spawn_bundle(Text2dBundle {
                text: items_header,
                transform: Transform::from_xyz(0., 340., 0.001),
                ..Default::default()
            });

            let inventory = Text::with_section(
                "Stats".to_string(),
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 20.0,
                    color: light_text_color(),
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            );
            cmds.spawn_bundle(Text2dBundle {
                text: inventory,
                transform: Transform::from_xyz(160., 365., 0.001),
                ..Default::default()
            })
            .with_children(|cmds| {
                cmds.spawn_bundle(MyButtonBundle {
                    button: MyButton {
                        size: Vec2::new(80., 30.),
                        id: ClickedStats,
                    },
                    transform: Transform::from_xyz(10., 0., 0.0001),
                    ..Default::default()
                });
            });
        });
}

fn dispatch_items(
    mut commands: Commands,
    mut cur_view: ResMut<Option<CurrentItemsView>>,
    items: Res<PlayerItems>,
    font: Res<RobotoFont>,
) {
    if !items.is_changed() && cur_view.is_some() {
        return;
    }
    if let Some(CurrentItemsView(entity)) = cur_view.take() {
        commands.entity(entity).despawn_recursive();
    }
    let next_view = commands
        .spawn()
        .insert(MainMenuMarker)
        .insert(Transform::from_xyz(530., 0., 0.3))
        .insert(GlobalTransform::default())
        .with_children(|cmds| {
            draw_slot(cmds, items.head.equipped(), &font);
            draw_slot(cmds, items.cloak.equipped(), &font);
            draw_slot(cmds, items.lockpick.equipped(), &font);
            draw_slot(cmds, items.boots.equipped(), &font);
        })
        .id();

    *cur_view = Some(CurrentItemsView(next_view));
}

fn clicked_stats(
    mut event_reader: EventReader<ClickedButtonEvent<ClickedStats>>,
    mut state: ResMut<State<GameState>>,
) {
    if event_reader.iter().next().is_some() {
        log::debug!("moving to stats screen");
        state
            .push(GameState::StatsScreen)
            .expect("cant move to stats screen");
    }
}

fn clicked_slot(
    mut event_reader: EventReader<ClickedButtonEvent<ClickedSlot>>,
    mut state: ResMut<State<GameState>>,
    mut view: ResMut<ViewInvSlot>,
) {
    for ClickedButtonEvent(ClickedSlot(slot)) in event_reader.iter() {
        log::debug!("moving to inventory screen");
        view.0 = *slot;
        view.1 = 0;
        state
            .push(GameState::InventoryScreen)
            .expect("cant move to inventory screen");
    }
}

fn clicked_level(
    mut event_reader: EventReader<ClickedButtonEvent<ClickedLevel>>,
    mut state: ResMut<State<GameState>>,
    items: Res<PlayerItems>,
    mut stats: ResMut<PlayerStatsMods>,
    mut sel_level: ResMut<SelectedLevel>,
) {
    for ClickedButtonEvent(ClickedLevel(level)) in event_reader.iter() {
        log::debug!("moving to playing");
        sel_level.0 = *level;
        *stats = items.stats();
        state
            .set(GameState::LoadingLevel)
            .expect("cant move to playing");
    }
}

pub fn change_camera_scale_from_resize(
    mut query: Query<&mut Transform, With<MainCamera>>,
    mut events: EventReader<WindowResized>,
    windows: Res<Windows>,
) {
    for _ in events.iter() {
        let wnd = windows.get_primary().unwrap();
        for mut proj in query.iter_mut() {
            log::debug!("new width {} new hright {}", wnd.width(), wnd.height());
            proj.scale.x = WIDTH / wnd.width();
            proj.scale.y = HEIGHT / wnd.height();
        }
    }
}

pub struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PlayerItems>()
            .init_resource::<PlayerStatsMods>()
            .init_resource::<Option<CurrentItemsView>>()
            .init_resource::<SelectedLevel>()
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu)
                    .with_system(cleanup_system::<MainMenuMarker>.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(clicked_slot.system().after("button_click"))
                    .with_system(clicked_stats.system().after("button_click"))
                    .with_system(clicked_level.system().after("button_click"))
                    .with_system(dispatch_items.system().label("dispatch_inventory"))
                    .with_system(change_camera_scale_from_resize.system()),
            );
        register_my_button::<ClickedStats>(app, GameState::MainMenu);
        register_my_button::<ClickedSlot>(app, GameState::MainMenu);
        register_my_button::<ClickedLevel>(app, GameState::MainMenu);
    }
}
