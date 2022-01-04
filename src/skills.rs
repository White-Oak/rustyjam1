use std::time::Duration;

use bevy::prelude::*;

use crate::{cleanup::cleanup_system, player::SpellKind, GameState};

const BASE_WIDTH: f32 = 352. / 2560. * 100. * 0.75;
const BASE_HEIGHT: f32 = 227. / 1440. * 100. * 0.75;

// TODO: less updates for skill ui
// const UPDATE_SKILLS_SECS: f32 = 0.05;

struct SkillsUiHandles {
    layout: Handle<ColorMaterial>,
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
    materials.add(ColorMaterial::texture(handle))
}

impl FromWorld for SkillsUiHandles {
    fn from_world(world: &mut World) -> Self {
        let layout = get_handle(world, "skills_ui.png");
        let mut color_materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("no materials");
        let none_material = color_materials.add(Color::NONE.into());
        SkillsUiHandles {
            layout,
            none_material,
        }
    }
}

#[derive(Debug, Default)]
pub struct SkillsState {
    q: SkillState,
    e: SkillState,
    r: SkillState,
}

impl SkillsState {
    pub fn get_state(&self, kind: SpellKind) -> &SkillState {
        match kind {
            SpellKind::Dash => &self.q,
            SpellKind::Smoke => &self.e,
            SpellKind::Emp => &self.r,
        }
    }

    pub fn get_state_mut(&mut self, kind: SpellKind) -> &mut SkillState {
        match kind {
            SpellKind::Dash => &mut self.q,
            SpellKind::Smoke => &mut self.e,
            SpellKind::Emp => &mut self.r,
        }
    }

    fn tick_states(&mut self, delta: Duration) {
        for state in [&mut self.q, &mut self.e, &mut self.r] {
            if let Some(time_to_cd) = &mut state.time_to_cd {
                time_to_cd.tick(delta);
                if time_to_cd.finished() {
                    state.time_to_cd = None;
                }
            }
        }
    }

    fn needs_to_tick(&self) -> bool {
        [&self.q, &self.e, &self.r]
            .iter()
            .any(|s| s.time_to_cd.is_some())
    }
}

#[derive(Debug, Default)]
pub struct SkillState {
    pub time_to_cd: Option<Timer>,
}

struct SkillsUiMarker;

trait UiSkill {
    fn icon_name(&self) -> &'static str;
    fn enabled(&self) -> bool;
}

impl UiSkill for SpellKind {
    fn icon_name(&self) -> &'static str {
        match self {
            SpellKind::Dash => "Q",
            SpellKind::Smoke => "E",
            SpellKind::Emp => "R",
        }
    }

    fn enabled(&self) -> bool {
        !matches!(self, SpellKind::Emp)
    }
}

struct CooldownMarker;
struct IconMarker;

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
    ui_cmds
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(2.),
                    bottom: Val::Px(2.),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Percent(BASE_WIDTH),
                    height: Val::Percent(BASE_HEIGHT),
                },
                min_size: Size {
                    width: Val::Percent(BASE_WIDTH),
                    height: Val::Percent(BASE_HEIGHT),
                },
                ..Default::default()
            },
            material: material.clone(),
            ..Default::default()
        })
        .insert(SkillsUiMarker)
        .with_children(|ec| {
            ec.spawn_bundle(ImageBundle {
                material: textures.layout.clone(),
                style: Style {
                    min_size: Size {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                    },
                    size: Size {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|ec| {
                for s in [SpellKind::Dash, SpellKind::Smoke] {
                    ec.spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::ColumnReverse,
                            size: Size {
                                width: Val::Percent(50.),
                                height: Val::Percent(100.),
                            },
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::FlexEnd,
                            ..Default::default()
                        },
                        material: material.clone(),
                        ..Default::default()
                    })
                    .with_children(|ec| {
                        let text = Text::with_section(
                            "10.9",
                            TextStyle {
                                font: font_handle.clone(),
                                font_size: 12.0,
                                color: Color::WHITE,
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        );
                        ec.spawn_bundle(TextBundle {
                            visible: Visible {
                                is_transparent: true,
                                is_visible: false,
                            },
                            text,
                            ..Default::default()
                        })
                        .insert(s)
                        .insert(CooldownMarker);
                        ec.spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size {
                                    height: Val::Percent(80.),
                                    width: Val::Percent(100.),
                                },
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            material: material.clone(),
                            ..Default::default()
                        })
                        .with_children(|ec| {
                            let text = Text::with_section(
                                s.icon_name(),
                                TextStyle {
                                    font: font_handle.clone(),
                                    font_size: 48.0,
                                    color: Color::WHITE,
                                },
                                TextAlignment {
                                    vertical: VerticalAlign::Center,
                                    horizontal: HorizontalAlign::Center,
                                },
                            );
                            ec.spawn_bundle(TextBundle {
                                text,
                                ..Default::default()
                            })
                            .insert(s)
                            .insert(IconMarker);
                        });
                    });
                }
            });
        });
}

fn tick_states(time: Res<Time>, mut skills_state: ResMut<SkillsState>) {
    if skills_state.needs_to_tick() {
        skills_state.tick_states(time.delta());
    }
}

#[allow(clippy::type_complexity)]
fn update_texts(
    skills_state: Res<SkillsState>,
    cooldowns: Query<
        (&mut Text, &mut Visible, &SpellKind),
        (With<CooldownMarker>, Without<IconMarker>),
    >,
    icons: Query<(&mut Text, &SpellKind), (With<IconMarker>, Without<CooldownMarker>)>,
) {
    if skills_state.is_changed() {
        cooldowns.for_each_mut(|(mut text, mut visible, kind)| {
            let state = skills_state.get_state(*kind);
            if let Some(time_to_cd) = &state.time_to_cd {
                let left = (time_to_cd.duration() - time_to_cd.elapsed()).as_secs_f32();
                text.sections[0].value = format!("{left:.1}");
                visible.is_visible = true;
            } else {
                visible.is_visible = false;
            }
        });
        icons.for_each_mut(|(mut text, kind)| {
            let state = skills_state.get_state(*kind);
            if state.time_to_cd.is_some() {
                text.sections[0].style.color = Color::GRAY;
            } else {
                text.sections[0].style.color = Color::WHITE;
            }
        });
    }
}

pub struct SkillsUiPlugin;
impl Plugin for SkillsUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SkillsUiHandles>()
            .init_resource::<SkillsState>()
            .add_system_set(SystemSet::on_enter(GameState::Level).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Level)
                    .with_system(tick_states.system().label("skills_ui_tick"))
                    .with_system(update_texts.system().after("skills_ui_tick")),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Level)
                    .with_system(cleanup_system::<SkillsUiMarker>.system()),
            );
    }
}
