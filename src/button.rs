use bevy::{
    log,
    math::{Vec3Swizzles, Vec4Swizzles},
    prelude::*,
};

use crate::MainCamera;

#[derive(Debug, Default)]
pub struct MyButton<T: Default + Clone> {
    pub size: Vec2,
    pub id: T,
}

#[derive(Debug, Default, Bundle)]
pub struct MyButtonBundle<T: Default + Clone + Send + Sync + 'static> {
    pub button: MyButton<T>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

// selected, spawned to highlight selected
pub struct SelectedButton<T>(Entity, Entity, T);

pub struct SelectedButtonEvent<T>(T);
pub struct ClickedButtonEvent<T>(T);

struct SelectedButtonColor(Handle<ColorMaterial>);

#[allow(clippy::too_many_arguments)]
fn select_button<T: Default + Clone + Send + Sync + 'static>(
    mut commands: Commands,
    windows: Res<Windows>,
    buttons: Query<(Entity, &MyButton<T>, &GlobalTransform)>,
    mut selected: ResMut<Option<SelectedButton<T>>>,
    selected_color: Res<SelectedButtonColor>,
    q_camera: Query<&Transform, With<MainCamera>>,
    highlights: Query<Entity>,
    mut selected_events: EventWriter<SelectedButtonEvent<T>>,
) {
    let wnd = windows.get_primary().unwrap();
    let pos = if let Some(position) = wnd.cursor_position() {
        position
    } else {
        return;
    };

    // get the size of the window
    let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let p = pos - size / 2.0;
    // assuming there is exactly one main camera entity, so this is OK
    let camera_transform = q_camera.single().unwrap();

    // apply the camera transform
    let pos_wld = (camera_transform.compute_matrix() * p.extend(0.0).extend(1.0)).xy();

    // let pos = pos_wld.xy() -
    let mut new_selected = None;
    buttons.for_each(|(entity, button, tr)| {
        let pos = pos_wld - tr.translation.xy();
        let h_w = button.size.x * 0.5;
        let h_h = button.size.y * 0.5;
        if (pos.x > -h_w && pos.x < h_w) && (pos.y > -h_h && pos.y < h_h) {
            new_selected = Some((entity, button));
        }
    });
    if let Some(SelectedButton(entity, high_entity, _)) = selected.as_ref() {
        if let Some((new_entity, _)) = new_selected {
            if new_entity == *entity {
                return;
            }
        }
        if highlights.get(*high_entity).is_ok() {
        log::debug!("deselected a button");
            commands.entity(*high_entity).despawn_recursive();
            let _ = selected.take();
        }
    } else if let Some((new_entity, button)) = new_selected {
        log::debug!("selected a new button");
        commands.entity(new_entity).with_children(|cmds| {
            let new_high = cmds
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(button.size),
                    material: selected_color.0.clone(),
                    ..Default::default()
                })
                .id();
            let _ = selected.insert(SelectedButton(new_entity, new_high, button.id.clone()));
        });
        selected_events.send(SelectedButtonEvent(button.id.clone()));
    }
}

fn check_for_clicks<T: Default + Clone + Send + Sync + 'static>(
    mouse: Res<Input<MouseButton>>,
    selected: Res<Option<SelectedButton<T>>>,
    mut events: EventWriter<ClickedButtonEvent<T>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(SelectedButton(_, _, id)) = selected.as_ref() {
            events.send(ClickedButtonEvent(id.clone()))
        }
    }
}

pub fn register_my_button<T: Default + Clone + Send + Sync + 'static>(world: &mut AppBuilder) {
    world
        .init_resource::<Option<SelectedButton<T>>>()
        .add_system(select_button::<T>.system())
        .add_system(check_for_clicks::<T>.system())
        .add_event::<SelectedButtonEvent<T>>()
        .add_event::<ClickedButtonEvent<T>>();
}

impl FromWorld for SelectedButtonColor {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("assets materials missing");
        let handle = materials.add(Color::rgba(1., 1., 0., 0.1).into());
        SelectedButtonColor(handle)
    }
}

pub struct MyButtonPlugin;
impl Plugin for MyButtonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedButtonColor>();
    }
}
