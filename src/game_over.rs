use bevy::{log, prelude::*};

use crate::{
    button::{register_my_button, ClickedButtonEvent, MyButton, MyButtonBundle},
    cleanup::cleanup_system,
    player::Player,
    GameState, RobotoFont, items::PlayerItems,
};

struct GameoverMarker;

#[derive(Default, Clone, Debug)]
struct ClickedOk;

fn setup(mut commands: Commands, font: Res<RobotoFont>, player: Query<&Transform, With<Player>>) {
    let mut tr = player.single().expect("single player").translation;
    tr.z += 0.5;
    let mut tr = Transform::from_translation(tr);
    let mut ui_bundle = commands.spawn_bundle(UiCameraBundle::default());
    let ui_cmds = ui_bundle // root node
        .commands();
    let gameover = Text::with_section(
        "GAME OVER".to_string(),
        TextStyle {
            font: font.0.clone(),
            font_size: 96.,
            color: Color::RED,
        },
        TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        },
    );
    ui_cmds
        .spawn_bundle(Text2dBundle {
            text: gameover,
            transform: tr,
            ..Default::default()
        })
        .insert(GameoverMarker);
    let ok = Text::with_section(
        "OK".to_string(),
        TextStyle {
            font: font.0.clone(),
            font_size: 96.,
            color: Color::WHITE,
        },
        TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        },
    );
    tr.translation.y -= 100.;
    ui_cmds
        .spawn_bundle(Text2dBundle {
            text: ok,
            transform: tr,
            ..Default::default()
        })
        .insert(GameoverMarker)
        .with_children(|cmds| {
            cmds.spawn_bundle(MyButtonBundle {
                button: MyButton {
                    size: Vec2::new(320., 60.),
                    id: ClickedOk,
                },
                transform: Transform::from_xyz(0., 0., 0.001),
                ..Default::default()
            });
        });
    log::debug!("built game over");
}

fn clicked_ok(
    mut event_reader: EventReader<ClickedButtonEvent<ClickedOk>>,
    mut state: ResMut<State<GameState>>,
    mut items: ResMut<PlayerItems>,
) {
    if event_reader.iter().next().is_some() {
        //hack
        let _: &mut PlayerItems = &mut items;
        log::debug!("moving back to menu");
        state.pop().expect("cant move back from reward screen");
    }
}

pub struct GameoverPlugin;
impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::GameOver)
                    .with_system(clicked_ok.system().after("button_click")),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::GameOver)
                    .with_system(cleanup_system::<GameoverMarker>.system()),
            );
        register_my_button::<ClickedOk>(app, GameState::GameOver);
    }
}
