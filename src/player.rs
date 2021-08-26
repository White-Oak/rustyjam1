use bevy::{log, prelude::*};

use crate::{map::SpawnPoint, movement::Velocity, GameState};

pub const PLAYER_SIZE: f32 = 32.;
const PLAYER_SPEED: f32 = 2.;
const LIGHT_RADIUS: f32 = 3000.;

const DASH_CAST_TIME: f32 = 0.0;
const SMOKE_CAST_TIME: f32 = 1.0;
const EMP_CAST_TIME: f32 = 1.5;

const DASH_DURATION: f32 = 0.5;
const SMOKE_DURATION: f32 = 1.0;
const EMP_DURATION: f32 = 1.5;

const DASH_VEL_MULTI: f32 = 3.;

struct CastingSpell {
    kind: SpellKind,
    timer: Timer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpellKind {
    Dash,
    Smoke,
    Emp,
}

struct Casting {
    kind: SpellKind,
    timer: Timer,
}

// struct Channeling {
//     kind: SpellKind
// }

enum CastingCommand {
    Cast(SpellKind),
    Interrupt,
}

impl SpellKind {
    fn cast_time(&self) -> f32 {
        match self {
            SpellKind::Dash => DASH_CAST_TIME,
            SpellKind::Smoke => SMOKE_CAST_TIME,
            SpellKind::Emp => EMP_CAST_TIME,
        }
    }

    fn duration(&self) -> f32 {
        match self {
            SpellKind::Dash => DASH_DURATION,
            SpellKind::Smoke => SMOKE_DURATION,
            SpellKind::Emp => EMP_DURATION,
        }
    }
}

#[derive(Debug)]
struct LastVelocity(Vec2);
impl Default for LastVelocity {
    fn default() -> Self {
        // TODO: init with modified player speed
        Self(Vec2::new(0., -PLAYER_SPEED))
    }
}
struct Dashing(Timer);

pub struct Player;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<ColorMaterial>>,
    spawn: Res<SpawnPoint>,
) {
    let smile = asset_server.load("smile.png");
    let sprite = Sprite::new(Vec2::splat(PLAYER_SIZE));
    let material = assets.add(ColorMaterial {
        color: Color::BLACK,
        texture: Some(smile),
    });

    let particle = asset_server.load("13.png");
    // let light_size = Vec2::new(4000., 4000.);
    let light_size = Vec2::splat(LIGHT_RADIUS);
    let light = Sprite::new(light_size / 2.);
    let light_material = assets.add(ColorMaterial {
        color: Color::rgba(1., 1., 1., 1.),
        texture: Some(particle),
    });
    let spawn = spawn.0.expect("loaded");
    let spawn = (spawn, 0.5).into();

    commands
        .spawn_bundle(SpriteBundle {
            sprite,
            material,
            transform: Transform::from_translation(spawn),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity::default())
        .with_children(|ec| {
            ec.spawn_bundle(SpriteBundle {
                sprite: light,
                material: light_material,
                visible: Visible {
                    is_visible: true,
                    is_transparent: true,
                },
                ..Default::default()
            });
        });
}

fn move_keyboard(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, (With<Player>, Without<Dashing>)>,
    mut casting_command: ResMut<Option<CastingCommand>>,
    cast_res: Res<Option<Casting>>,
    mut last_velocity: ResMut<LastVelocity>,
) {
    let mut velocity = if let Ok(x) = query.single_mut() {
        x
    } else {
        return;
    };
    let up = keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up);
    let down = keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down);
    let left = keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left);
    let right = keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right);
    let q = keys.pressed(KeyCode::Q);
    let e = keys.pressed(KeyCode::E);
    let r = keys.pressed(KeyCode::R);
    velocity.0.x = 0.;
    velocity.0.y = 0.;
    if cast_res.is_none() {
        if up {
            velocity.0.y += PLAYER_SPEED;
        }
        if down {
            velocity.0.y -= PLAYER_SPEED;
        }
        if right {
            velocity.0.x += PLAYER_SPEED;
        }
        if left {
            velocity.0.x -= PLAYER_SPEED;
        }
        if velocity.0 != Vec2::ZERO {
            last_velocity.0 = velocity.0;
        }
    }
    // if any move input - queue casting interrupt.
    if up || down || left || right {
        // player.casting = None;
        *casting_command = Some(CastingCommand::Interrupt);
    }
    if q {
        *casting_command = Some(CastingCommand::Cast(SpellKind::Dash));
    } else if e {
        *casting_command = Some(CastingCommand::Cast(SpellKind::Smoke));
    } else if r {
        *casting_command = Some(CastingCommand::Cast(SpellKind::Emp));
    }
}

fn move_keyboard_for_dashing(
    keys: Res<Input<KeyCode>>,
    query: Query<Entity, (With<Player>, With<Dashing>)>,
    mut casting_command: ResMut<Option<CastingCommand>>,
) {
    if query.single().is_err() {
        return;
    }
    let up = keys.just_pressed(KeyCode::W) || keys.just_pressed(KeyCode::Up);
    let down = keys.just_pressed(KeyCode::S) || keys.just_pressed(KeyCode::Down);
    let left = keys.just_pressed(KeyCode::A) || keys.just_pressed(KeyCode::Left);
    let right = keys.just_pressed(KeyCode::D) || keys.just_pressed(KeyCode::Right);
    let e = keys.just_pressed(KeyCode::E);
    let r = keys.just_pressed(KeyCode::R);
    // if any move input - queue casting interrupt.
    if up || down || left || right {
        log::debug!("setting interrupt for dash");
        // player.casting = None;
        *casting_command = Some(CastingCommand::Interrupt);
    }
    if e {
        *casting_command = Some(CastingCommand::Cast(SpellKind::Smoke));
    } else if r {
        *casting_command = Some(CastingCommand::Cast(SpellKind::Emp));
    }
}

fn dash(
    mut commands: Commands,
    query: Query<(Entity, &mut Dashing, &mut Velocity)>,
    time: Res<Time>,
    casting_command: Res<Option<CastingCommand>>,
) {
    query.for_each_mut(|(entity, mut dashing, mut velocity)| {
        // TODO: normalize per frame
        let delta = time.delta();
        let mut needs_interrupt = false;
        if dashing.0.tick(delta).just_finished() {
            needs_interrupt = true;
        }
        // TODO: so maybe events?
        if let Some(CastingCommand::Cast(SpellKind::Dash)) = casting_command.as_ref() {
            // don't interrupt dash on dash
        } else {
            log::debug!("found interrupt for dash");
            needs_interrupt = true;
        }
        if needs_interrupt {
            log::debug!("finished dashing");
            commands.entity(entity).remove::<Dashing>();
            velocity.0 = Vec2::ZERO;
        }
    });
}

fn start_dash(
    query: Query<&mut Velocity, Added<Dashing>>,
    last_velocity: Res<LastVelocity>,
    // mut casting_command: ResMut<Option<CastingCommand>>,
) {
    // TODO: maybe move to events after all
    // or move this take inside
    // let _ = casting_command.take();
    query.for_each_mut(|mut velocity| velocity.0 = last_velocity.0 * DASH_VEL_MULTI);
}

fn process_casting(
    mut commands: Commands,
    casting_command: Res<Option<CastingCommand>>,
    mut cast_res: ResMut<Option<Casting>>,
    player: Query<Entity, With<Player>>,
    time: Res<Time>,
) {
    if let Some(casting) = cast_res.as_mut() {
        casting.timer.tick(time.delta());
    }
    if let Some(event) = casting_command.as_ref() {
        if let Some(casting) = cast_res.as_mut() {
            match event {
                CastingCommand::Cast(cast_kind) if *cast_kind == casting.kind => {
                    // don't interrupt or anything
                }

                CastingCommand::Cast(cast_kind) => {
                    // TODO: remove animation
                    casting.kind = *cast_kind;
                    // TODO: scale with stats
                    casting.timer = Timer::from_seconds(cast_kind.cast_time(), false);
                    // TODO: spawn animation
                }
                CastingCommand::Interrupt => {
                    // TODO: remove animation
                    // but Dash will take care of itself
                    cast_res.take();
                }
            }
        } else if let CastingCommand::Cast(cast_kind) = event {
            let kind = *cast_kind;
            // TODO: scale with stats
            let timer = Timer::from_seconds(cast_kind.cast_time(), false);
            let casting = Casting { kind, timer };
            *cast_res = Some(casting);
            log::debug!("started casting");
        }
    }

    if let Some(casting) = cast_res.as_mut() {
        if casting.timer.just_finished() {
            log::debug!("finished casting");
            let player = player.single().expect("single player");
            match casting.kind {
                SpellKind::Dash => {
                    let timer = Timer::from_seconds(casting.kind.duration(), false);
                    commands.entity(player).insert(Dashing(timer));
                    log::debug!("starting dashing");
                }
                SpellKind::Smoke => {}
                SpellKind::Emp => {}
            }
            let _ = cast_res.take();
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Level).with_system(spawn_player.system()),
        )
        .init_resource::<LastVelocity>()
        .init_resource::<Option<Casting>>()
        .init_resource::<Option<CastingCommand>>()
        .add_system_set(
            SystemSet::on_update(GameState::Level)
                .with_system(move_keyboard.system().label("control"))
                .with_system(move_keyboard_for_dashing.system().label("control"))
                .with_system(start_dash.system().before("control"))
                // TODO: is it control or after control
                .with_system(dash.system().after("control"))
                .with_system(process_casting.system().after("control")),
        );
    }
}
