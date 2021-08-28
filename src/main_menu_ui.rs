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
    items::{Item, PlayerItems},
    GameState, MainCamera, HEIGHT, WIDTH,
};

fn build_rectangle(width: f32, height: f32) -> Path {
    let mut builder = Builder::new();
    builder.add_rectangle(
        &euclid::Rect::new(
            Point2D::new(-width / 2., -height / 2.),
            Size2D::new(width, height),
        ),
        Winding::Positive,
    );
    builder.build()
}

fn build_rounded_rectangle(width: f32, height: f32) -> Path {
    let mut builder = Builder::new();
    builder.add_rounded_rectangle(
        // &euclid::Rect::new(Point2D::new(-50., -50.), Size2D::new(100., 100.)),
        &euclid::Rect::new(
            Point2D::new(-width / 2., -height / 2.),
            Size2D::new(width, height),
        ),
        &BorderRadii::new(7.),
        Winding::Positive,
    );
    builder.build()
}

fn get_color(r: u8, g: u8, b: u8) -> Color {
    Color::rgb(
        r as f32 / (255. * 8.),
        g as f32 / (255. * 8.),
        b as f32 / (255. * 8.),
    )
}

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
        crate::items::Slot::Head => 200.,
        crate::items::Slot::Cloak => 35.,
        crate::items::Slot::Lockpick => -135.,
        crate::items::Slot::Boots => -301.,
    };
    let x = 95.;
    let color = light_text_color();
    let name = Text::with_section(
        item.name.clone(),
        TextStyle {
            font: font_handle.clone(),
            font_size: 21.,
            color
        },
        TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center
        },
    );
    cmds.spawn_bundle(Text2dBundle {
        text: name,
        transform: Transform::from_xyz(x, y + 70., 0.001),
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
            color
        },
        TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center
        },
    );
    cmds.spawn_bundle(Text2dBundle {
        text: item_mods,
        transform: Transform::from_xyz(x, y, 0.001),
        ..Default::default()
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
                    color: Color::rgb_u8(255, 252, 236),
                },
                Default::default(),
            );
            cmds.spawn_bundle(Text2dBundle {
                text: items_header,
                transform: Transform::from_xyz(0., 340., 0.001),
                ..Default::default()
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
    }
}
