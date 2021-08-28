use bevy::{log, prelude::*};
use itertools::Itertools;

use crate::{
    button::{register_my_button, ClickedButtonEvent, MyButton, MyButtonBundle},
    cleanup::cleanup_system,
    items::PlayerItems,
    main_menu_ui::{change_camera_scale_from_resize, light_text_color},
    perlin::{PerlinBundle, PerlinPipelineHandle},
    GameState, RobotoFont,
};

struct StatsScreenMarker;

#[derive(Debug, Clone, Copy, Default)]
struct ClickedBack;

struct UiTexture(Handle<ColorMaterial>);

fn setup(
    mut commands: Commands,
    items: Res<PlayerItems>,
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
        .insert(StatsScreenMarker)
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert_bundle(MeshBundle {
            mesh: meshes.add(mesh),
            transform: Transform::from_xyz(0., 0., 0.2),
            ..Default::default()
        })
        .insert_bundle(PerlinBundle::new(
            &pp_handle,
            1500.,
            0.2,
            Vec3::new(0.05, 0.05, 0.05),
        ))
        .with_children(|cmds| {
            let sprite = Sprite::new(Vec2::new(547., 719.));
            cmds.spawn_bundle(SpriteBundle {
                sprite,
                material: ui_texture.0.clone(),
                transform: Transform::from_xyz(0., 0., 0.001),
                ..Default::default()
            })
            .with_children(|cmds| {
                let header = Text::with_section(
                    "All stats".to_string(),
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
                    transform: Transform::from_xyz(0., 281., 0.001),
                    ..Default::default()
                });

                let text = items
                    .all_equipped_mods()
                    .map(|a_mod| a_mod.to_string())
                    .collect_vec()
                    .join("\n");

                let item_mods = Text::with_section(
                    text,
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
                    text: item_mods,
                    transform: Transform::from_xyz(0., 0., 0.001),
                    ..Default::default()
                });

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
                    transform: Transform::from_xyz(0., -281., 0.001),
                    ..Default::default()
                })
                .with_children(|cmds| {
                    cmds.spawn_bundle(MyButtonBundle {
                        button: MyButton {
                            size: Vec2::new(140., 30.),
                            id: ClickedBack,
                        },
                        transform: Transform::from_xyz(0., 0., 0.0001),
                        ..Default::default()
                    });
                });
            });
        });
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

impl FromWorld for UiTexture {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("no assets server");
        let handle = asset_server.load("stats_ui.png");
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("no materials");
        let handle = materials.add(ColorMaterial::texture(handle));
        UiTexture(handle)
    }
}

pub struct StatsScreenPlugin;
impl Plugin for StatsScreenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::StatsScreen).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_exit(GameState::StatsScreen)
                    .with_system(cleanup_system::<StatsScreenMarker>.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::StatsScreen)
                    .with_system(clicked_back.system().after("button_click"))
                    .with_system(change_camera_scale_from_resize.system()),
            )
            .init_resource::<UiTexture>();
        register_my_button::<ClickedBack>(app, GameState::StatsScreen);
    }
}
