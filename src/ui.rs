use bevy::{
    core::FixedTimestep,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

struct FpsCounter;

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font_handle = asset_server.load("FiraSans-Bold.ttf");
    let text = Text::with_section(
        "FPS: ".to_string(),
        TextStyle {
            font: font_handle,
            font_size: 30.0,
            color: Color::WHITE,
        },
        TextAlignment {
            vertical: VerticalAlign::Top,
            horizontal: HorizontalAlign::Left,
        },
    );
    // let material = color_materials.add(Color::rgba(0., 0., 0., 0.));
    let material = color_materials.add(Color::NONE.into());
    let mut ui_bundle = commands.spawn_bundle(UiCameraBundle::default());
    let ui_cmds = ui_bundle // root node
        .commands();
    ui_cmds
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .with_children(|ec| {
            ec.spawn_bundle(TextBundle {
                text,
                ..Default::default()
            })
            .insert(FpsCounter);
        });
}

fn fps_change_text(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsCounter>>) {
    let fps = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).unwrap();
    if let Some(v) = fps.average() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("FPS: {}", v as i64);
        }
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // app.add_startup_system(setup.system()).add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::steps_per_second(4.))
        //         .with_system(fps_change_text.system()),
        // );
    }
}
