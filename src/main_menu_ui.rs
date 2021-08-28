use bevy::{log, prelude::*, window::WindowResized};
use bevy_prototype_lyon::{prelude::*, shapes::SvgPathShape};
use lyon_path::{
    builder::BorderRadii,
    geom::euclid::{Point2D, Size2D},
    path::Builder,
    traits::PathBuilder,
    Path, Winding,
};

use crate::{GameState, HEIGHT, MainCamera, WIDTH};

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

fn setup(
    mut cmds: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let font_handle = asset_server.load("FiraSans-Bold.ttf");

    let sprite = Sprite::new(Vec2::new(WIDTH, HEIGHT));
    let main_menu = materials.add(ColorMaterial::texture(asset_server.load("001.png")));
    cmds.spawn_bundle(SpriteBundle{
        sprite,
        material: main_menu,
..Default::default()
    });
}

//     let bg = build_rectangle(3000., 3000.);
//     let bg_color = get_color(1, 10, 33);
//     let bg_color = Color::rgb_u8(0, 1, 3);
//     cmds.spawn_bundle(GeometryBuilder::build_as(
//         &bg,
//         ShapeColors::new(bg_color),
//         DrawMode::Fill(FillOptions::default()),
//         Transform::from_xyz(0., 0., 0.001),
//     ))
//     .with_children(|cmds| {
//         let next_bg_color = get_color(6, 14, 43);
//         let next_bg_outline_color = get_color(0, 7, 66);
//         let next_bg_outline_options = StrokeOptions::tolerance(0.1).with_line_width(6.);
//         let main_bg_color = get_color(30, 13, 13);
//         let left = build_rounded_rectangle(1280., 900.);
//         cmds.spawn_bundle(GeometryBuilder::build_as(
//             &left,
//             ShapeColors::outlined(next_bg_color, next_bg_outline_color),
//             DrawMode::Outlined {
//                 fill_options: FillOptions::default(),
//                 outline_options: next_bg_outline_options,
//             },
//             Transform::from_xyz(-210., 0., 0.001),
//         ))
//         .with_children(|cmds| {
//             let levels_header = Text::with_section(
//                     "Levels".to_string(),
//                     TextStyle {
//                         font: font_handle.clone(),
//                         font_size: 60.0,
//                         color: Color::rgb_u8(255, 252, 236),
//                     },
//                     Default::default()
//                 );
//             cmds.spawn_bundle(Text2dBundle {
//                 text: levels_header,
//                 transform: Transform::from_xyz(-470., 350., 0.001),
//                 ..Default::default()
//             });

//             let levels = build_rounded_rectangle(1240., 700.);
//             cmds.spawn_bundle(GeometryBuilder::build_as(
//                 &levels,
//                 ShapeColors::new(main_bg_color),
//                 DrawMode::Fill(FillOptions::default()),
//                 Transform::from_xyz(0., -50., 0.001),
//             ));
//         });

//         let right = build_rounded_rectangle(400., 900.);
//         cmds.spawn_bundle(GeometryBuilder::build_as(
//             &right,
//             ShapeColors::outlined(next_bg_color, next_bg_outline_color),
//             DrawMode::Outlined {
//                 fill_options: FillOptions::default(),
//                 outline_options: next_bg_outline_options,
//             },
//             Transform::from_xyz(650., 0., 0.001),
//         ))
//         .with_children(|cmds| {
//             let items_header = Text::with_section(
//                     "Items".to_string(),
//                     TextStyle {
//                         font: font_handle.clone(),
//                         font_size: 60.0,
//                         color: Color::rgb_u8(255, 252, 236),
//                     },
//                     Default::default()
//                 );
//             cmds.spawn_bundle(Text2dBundle {
//                 text: items_header,
//                 transform: Transform::from_xyz(-45., 350., 0.001),
//                 ..Default::default()
//             });

//             let levels = build_rounded_rectangle(360., 700.);
//             cmds.spawn_bundle(GeometryBuilder::build_as(
//                 &levels,
//                 ShapeColors::new(main_bg_color),
//                 DrawMode::Fill(FillOptions::default()),
//                 Transform::from_xyz(0., -50., 0.001),
//             ));
//         });
//     });
// }

// fn setup(
//     mut commands: Commands,
//     asset_server: ResMut<AssetServer>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let font_handle = asset_server.load("FiraSans-Bold.ttf");
//     let text = Text::with_section(
//         "TEST".to_string(),
//         TextStyle {
//             font: font_handle.clone(),
//             font_size: 30.0,
//             color: Color::WHITE,
//         },
//         TextAlignment {
//             vertical: VerticalAlign::Top,
//             horizontal: HorizontalAlign::Left,
//         },
//     );
//     let mut ui_bundle = commands.spawn_bundle(UiCameraBundle::default());
//     let cmds = ui_bundle // root node
//         .commands();
//     // let mut builder = Builder::new();
//     // builder.add_rounded_rectangle(
//     //     &euclid::Rect::new(Point2D::zero(), Size2D::new(100., 100.)),
//     //     &BorderRadii::new(6.),
//     //     Winding::Positive,
//     // );
//     // let path = builder.build();
//     let node = Node {
//         size: Vec2::new(WIDTH, HEIGHT),
//     };

//     // cmds.spawn_bundle(geometrybuilder::build_as(
//     //     &path,
//     //     shapecolors::new(color::white),
//     //     drawmode::fill(filloptions::default()),
//     //     transform::default(),
//     // ));
//     let bg_material = materials.add(Color::rgb_u8(0, 7, 66).into());
//     let style = Style {
//         position_type: PositionType::Relative,
//         // position: Rect {
//         //     right: Val::Px(0.),
//         //     top: Val::Px(0.),
//         //     left: Val::Px(0.),
//         //     bottom: Val::Px(0.),
//         //     ..Default::default()
//         // },
//         ..Default::default()
//     };
//     let rounded_rect = build_rounded_rectangle(1., 1.);
//     cmds.spawn_bundle(NodeBundle {
//         style: Style {
//             size: Size {
//                 width: Val::Percent(100.),
//                 height: Val::Percent(100.),
//             },
//             min_size: Size {
//                 width: Val::Percent(100.),
//                 height: Val::Percent(100.),
//             },
//             padding: Rect::all(Val::Px(20.)),
//             // position: Rect {
//             //     left: Val::Px(10.),
//             //     top: Val::Px(10.),
//             //     ..Default::default()
//             // },
//             ..Default::default()
//         },
//         material: bg_material.clone(),
//         ..Default::default()
//     })
//     .with_children(|cmds| {
//         let left = build_rounded_rectangle(20., 100.);
//         let right = build_rounded_rectangle(1., 1.);
//         let next_bg_color = Color::rgb_u8(6, 14, 43);
//         let next_bg_outline_color = Color::rgb_u8(0, 7, 66);
//         let main_bg_color = Color::rgb_u8(30, 13, 13);
//         let next_bg_outline_options = StrokeOptions::tolerance(0.1).with_line_width(0.5);
//         cmds.spawn()
//             .insert_bundle(GeometryBuilder::build_as(
//                 &left,
//                 ShapeColors::outlined(next_bg_color, next_bg_outline_color),
//                 DrawMode::Outlined {
//                     fill_options: FillOptions::default(),
//                     outline_options: next_bg_outline_options,
//                 },
//                 Transform::default(),
//             ))
//             .insert(Node::default())
//             .insert(Style {
//                     flex_grow: 1.,
//                     margin: Rect {
//                         right: Val::Px(10.),
//                         ..Default::default()
//                     },
//                     // flex_direction: FlexDirection::ColumnReverse,
//                     padding: Rect::all(Val::Px(10.)),
//                     ..Default::default()

//             })
//             .with_children(|cmds| {
//                 let levels_header = Text::with_section(
//                     "Levels".to_string(),
//                     TextStyle {
//                         font: font_handle,
//                         font_size: 30.0,
//                         // color: Color::rgb_u8(255, 252, 236),
//                         color: Color::RED,
//                     },
//                     Default::default()
//                     // TextAlignment {
//                     //     vertical: VerticalAlign::Center,
//                     //     horizontal: HorizontalAlign::Center,
//                     // },
//                 );
//                 // cmds.spawn_bundle(NodeBundle {
//                 //     material: materials.add(Color::NONE.into()),
//                 //     ..Default::default()
//                 // })
//                 // .with_children(|cmds| {
//                     cmds.spawn_bundle(TextBundle {
//                         text: levels_header,
//                         ..Default::default()
//                     });
//                 // });
//                 // levels list
//                 // cmds.spawn_bundle(GeometryBuilder::build_as(
//                 //     &rounded_rect,
//                 //     ShapeColors::new(main_bg_color),
//                 //     DrawMode::Fill(FillOptions::default()),
//                 //     Transform::default(),
//                 // ))
//                 // .insert(Node::default())
//                 // .insert(Style {
//                 //     size: Size {
//                 //         width: Val::Px(200.),
//                 //         height: Val::Px(200.),
//                 //     },
//                 //     ..Default::default()
//                 // });
//             });
//         cmds.spawn_bundle(GeometryBuilder::build_as(
//             &rounded_rect,
//             ShapeColors::outlined(next_bg_color, next_bg_outline_color),
//             DrawMode::Outlined {
//                 fill_options: FillOptions::default(),
//                 outline_options: next_bg_outline_options,
//             },
//             Transform::default(),
//         ))
//         .insert(node)
//         .insert(Style {
//             size: Size {
//                 width: Val::Px(200.),
//                 height: Val::Auto,
//             },
//             // min_size: Size {
//             //     width: Val::Px(200.),
//             //     height: Val::Auto,
//             // },
//             ..Default::default()
//         });
//         // cmds.spawn_bundle(NodeBundle {
//         //     style: Style {

//         //     },
//         //     material: materials.add(Color::BLUE.into()),
//         //     draw: (),
//         //     visible: (),
//         //     render_pipelines: (),
//         //     transform: (),
//         //     global_transform: (),
//         // })
//         // let rectangle = build_rectangle(1., 1.);
//         // cmds.spawn_bundle(GeometryBuilder::build_as(
//         //     &rectangle,
//         //     ShapeColors::new(Color::BLUE),
//         //     DrawMode::Fill(FillOptions::default()),
//         //     Transform::default(),
//         // )).insert(node).

//         // cmds.spawn_bundle(GeometryBuilder::build_as(
//         //     &path,
//         //     ShapeColors::new(Color::WHITE),
//         //     DrawMode::Fill(FillOptions::default()),
//         //     Transform::default(),
//         // ))
//         // .insert(node)
//         // .insert(style);
//     });
//     // let main_layout = asset_server.load("001.png");
//     // let sprite = Sprite::new(Vec2::new(WIDTH, HEIGHT));
//     // let node = Node {
//     //     size: Vec2::new(WIDTH, HEIGHT),
//     // };

//     // let material = assets.add(ColorMaterial {
//     //     color: Color::rgba(1., 1., 1., 1.),
//     //     texture: Some(main_layout),
//     // });
//     // let style = Style {
//     //     position_type: PositionType::Relative,
//     //     // position: Rect {
//     //     //     right: Val::Px(0.),
//     //     //     top: Val::Px(0.),
//     //     //     left: Val::Px(0.),
//     //     //     bottom: Val::Px(0.),
//     //     //     ..Default::default()
//     //     // },
//     //     size: Size {
//     //         width: Val::Percent(100.),
//     //         height: Val::Percent(100.),
//     //     },
//     //     min_size: Size {
//     //         width: Val::Percent(100.),
//     //         height: Val::Percent(100.),
//     //     },
//     //     ..Default::default()
//     // };

//     // cmds.spawn_bundle(SpriteBundle {
//     //     sprite,
//     //     material,
//     //     ..Default::default()
//     // });

//     // cmds.spawn()
//     //     .insert(node)
//     //     .insert(style)
//     //     .with_children(|cmds| {
//     //         cmds.spawn_bundle(TextBundle {
//     //             text,
//     //             ..Default::default()
//     //         });
//     //     });
//     // cmds.spawn_bundle(NodeBundle {
//     //     style: Style {
//     //         position_type: PositionType::Absolute,
//     //         position: Rect {
//     //             right: Val::Px(0.),
//     //             top: Val::Px(0.),
//     //             left: Val::Px(0.),
//     //             bottom: Val::Px(0.),
//     //             ..Default::default()
//     //         },
//     //         size: Size {
//     //             width: Val::Px(WIDTH),
//     //             height: Val::Px(HEIGHT),
//     //         },
//     //         max_size: Size {
//     //             width: Val::Px(WIDTH),
//     //             height: Val::Px(HEIGHT),
//     //         },
//     //         ..Default::default()
//     //     },
//     //     ..Default::default()
//     // })
//     // .with_children(|ec| {});
// }

// TODO: instead recreate shape
// fn resize_shape(query: Query<(&mut Transform, &Node), (With<ShapeColors>, Changed<Node>)>) {
//     query.for_each_mut(|(mut tr, size)| {
//         dbg!(size);
//         tr.scale.x = size.size.x / 100.;
//         tr.scale.y = size.size.y / 100.;
//     });
// }

fn change_camera_scale_from_resize(
    mut query: Query<&mut Transform, With<MainCamera>>,
    mut events: EventReader<WindowResized>,
    windows: Res<Windows>,
) {
    for _ in events.iter() {
        let wnd = windows.get_primary().unwrap();
        for mut proj in query.iter_mut() {
            log::debug!("new width {} new hright {}", wnd.width(), wnd.height());
            proj.scale.x= WIDTH / wnd.width();
            proj.scale.y = HEIGHT / wnd.height();
        }
    }
}


pub struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.
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
