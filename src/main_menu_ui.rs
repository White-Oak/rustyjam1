use bevy::{log, prelude::*, window::WindowResized};
use bevy_prototype_lyon::{prelude::*, shapes::SvgPathShape};
use itertools::Itertools;
use lyon_path::{
    builder::BorderRadii,
    geom::euclid::{Point2D, Size2D},
    path::Builder,
    traits::PathBuilder,
    Path, Winding,
};

use crate::{
    button::{register_my_button, ClickedButtonEvent, MyButton, MyButtonBundle},
    items::{Item, PlayerItems, Slot},
    GameState, MainCamera, HEIGHT, WIDTH,
};

#[derive(Debug, Clone, Copy, Default)]
struct ClickedStats;

#[derive(Debug, Clone, Copy, Default)]
struct ClickedSlot(Slot);

fn light_text_color() -> Color {
    Color::rgb_u8(255, 252, 236)
}

fn draw_slot(
    cmds: &mut ChildBuilder,
    item: &Item,
    asset_server: &AssetServer,
    materials: &mut Assets<ColorMaterial>,
) {
    let font_handle = asset_server.load("Roboto-Regular.ttf");
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
                font: font_handle.clone(),
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
                font: font_handle,
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
    items: Res<PlayerItems>,
) {
    let font_handle = asset_server.load("FiraSans-Bold.ttf");

    let sprite = Sprite::new(Vec2::new(WIDTH, HEIGHT));
    let main_menu = materials.add(ColorMaterial::texture(asset_server.load("001.png")));
    cmds.spawn_bundle(SpriteBundle {
        sprite,
        material: main_menu,
        ..Default::default()
    });

    cmds.spawn()
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
                transform: Transform::from_xyz(0., 340., 0.001),
                ..Default::default()
            });
        });

    cmds.spawn()
        .insert(Transform::from_xyz(530., 0., 0.001))
        .insert(GlobalTransform::default())
        .with_children(|mut cmds| {
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
                        size: Vec2::new(140., 30.),
                        id: ClickedStats,
                    },
                    transform: Transform::from_xyz(0., 0., 0.0001),
                    ..Default::default()
                });
            });

            draw_slot(
                &mut cmds,
                items.head.equipped(),
                &asset_server,
                &mut materials,
            );
            draw_slot(
                &mut cmds,
                items.cloak.equipped(),
                &asset_server,
                &mut materials,
            );
            draw_slot(
                &mut cmds,
                items.lockpick.equipped(),
                &asset_server,
                &mut materials,
            );
            draw_slot(
                &mut cmds,
                items.boots.equipped(),
                &asset_server,
                &mut materials,
            );
        });
}

fn clicked_stats(
    mut commands: Commands,
    mut event_reader: EventReader<ClickedButtonEvent<ClickedStats>>,
) {
    for _ in event_reader.iter() {
        // TODO: transition to stats
    }
}

fn clicked_slot(
    mut commands: Commands,
    mut event_reader: EventReader<ClickedButtonEvent<ClickedSlot>>,
) {
    for _ in event_reader.iter() {
        // TODO: transition to slot inv
    }
}

fn change_camera_scale_from_resize(
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
        app
        .init_resource::<PlayerItems>()
        .
        add_system_set(
            SystemSet::
                on_enter(GameState::MainMenu)
                .with_system(setup.system()),
        )
        .add_system_set(
            SystemSet::
                on_update(GameState::MainMenu)
                .with_system(change_camera_scale_from_resize.system()),
        )
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::steps_per_second(4.))
        //         .with_system(fps_change_text.system()),
        // )
        ;
        register_my_button::<ClickedStats>(app);
        register_my_button::<ClickedSlot>(app);
    }
}
