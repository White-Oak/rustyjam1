use bevy::{
    asset::LoadState,
    log,
    prelude::*,
    render::texture::FilterMode,
    sprite::collide_aabb::{collide, Collision},
};
use bevy_ecs_tilemap::prelude::*;
use tiled::PropertyValue;

use crate::{camera_enemy::CameraSpawn, player::PLAYER_SIZE, GameState, MainCamera};

// pub struct TiledMapHandle(Handle<TiledMap>);

#[derive(Default)]
pub struct Boundaries(Vec<(Vec3, Vec2)>);

#[derive(Default)]
pub struct SpawnPoint(pub Option<Vec2>);

#[derive(Default)]
struct CurrentLevelHandle(Handle<TiledMap>);

impl Boundaries {
    pub fn collide(&self, player_pos: Vec3) -> Option<Collision> {
        let player_size = Vec2::splat(PLAYER_SIZE);
        self.0
            .iter()
            .find_map(|(pos, size)| collide(*pos, *size, player_pos, player_size))
    }
}

fn load(asset_server: Res<AssetServer>, mut current_level: ResMut<CurrentLevelHandle>) {
    let handle: Handle<TiledMap> = asset_server.load("level1.tmx");
    current_level.0 = handle;
}

fn spawn_map(mut commands: Commands, current_level: Res<CurrentLevelHandle>) {
    let map_entity = commands.spawn().id();

    commands.entity(map_entity).insert_bundle(TiledMapBundle {
        tiled_map: current_level.0.clone(),
        map: Map::new(0u16, map_entity),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn load_boundaries(
    mut bounds: ResMut<Boundaries>,
    mut spawn: ResMut<SpawnPoint>,
    mut camera_spawns: ResMut<Vec<CameraSpawn>>,
    map_assets: ResMut<Assets<TiledMap>>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    let handle: Handle<TiledMap> = asset_server.load("level1.tmx");
    let map = if let Some(x) = map_assets.get(handle) {
        x
    } else {
        return;
    };
    let map_y = (map.map.height * map.map.tile_height) as f32;
    let mut spawn_x = 0.;
    let mut spawn_y = 0.;
    for group in map.map.object_groups.iter() {
        match group.name.as_str() {
            "Obstacles" => {
                for obj in group.objects.iter() {
                    bounds.0.push((
                        Vec3::new(
                            obj.x + obj.width / 2.,
                            (map_y - obj.y) - obj.height / 2.,
                            0.6,
                        ),
                        Vec2::new(obj.width, obj.height),
                    ));
                }
            }
            "Spawn" => {
                let spawn_obj = group.objects.first().expect("at least one spawn point");
                spawn_x = spawn_obj.x;
                spawn_y = map_y - spawn_obj.y;
                spawn.0 = Some(Vec2::new(spawn_x, spawn_y));
            }
            "Cameras" => {
                for spawn_obj in group.objects.iter() {
                    let x = spawn_obj.x;
                    let y = map_y - spawn_obj.y;
                    let props = &spawn_obj.properties;
                    let radius = if let Some(PropertyValue::FloatValue(x)) = props.get("radius") {
                        *x
                    } else {
                        panic!("no start_angle")
                    };
                    let start_angle =
                        if let Some(PropertyValue::FloatValue(x)) = props.get("start_angle") {
                            x.to_radians()
                        } else {
                            panic!("no start_angle")
                        };
                    let end_angle =
                        if let Some(PropertyValue::FloatValue(x)) = props.get("end_angle") {
                            x.to_radians()
                        } else {
                            panic!("no start_angle")
                        };
                    camera_spawns.push(CameraSpawn {
                        x,
                        y,
                        radius,
                        start_angle,
                        end_angle,
                    });
                }
            }
            _ => {
                log::error!("Unknown object layer: {}", group.name);
            }
        }
    }
    let mut camera_tr = camera.single_mut().expect("inexisting camera");
    camera_tr.translation.x = spawn_x;
    camera_tr.translation.y = spawn_y;
    camera_tr.scale.x = 1. / 2.;
    camera_tr.scale.y = 1. / 2.;
    state.set(GameState::Level).expect("cant set state");
}

fn debug_boundaries(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    boundaries: Res<Boundaries>,
) {
    let red = materials.add(ColorMaterial {
        color: Color::rgba(1., 0., 0., 0.5),
        texture: None,
    });
    for (pos, size) in boundaries.0.iter() {
        let sprite = Sprite::new(*size);
        commands.spawn_bundle(SpriteBundle {
            sprite,
            material: red.clone(),
            transform: Transform::from_translation(*pos),
            ..Default::default()
        });
    }
}

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Texture>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        if let AssetEvent::Created { handle } = event {
            if let Some(mut texture) = textures.get_mut(handle) {
                texture.sampler.min_filter = FilterMode::Nearest;
            }
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CurrentLevelHandle>()
            .init_resource::<SpawnPoint>()
            .init_resource::<Boundaries>()
            .add_system(set_texture_filters_to_nearest.system())
            .add_system_set(SystemSet::on_enter(GameState::LoadingLevel).with_system(load.system()))
            .add_system_set(
                SystemSet::on_update(GameState::LoadingLevel).with_system(load_boundaries.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::LoadingLevel).with_system(spawn_map.system()), // .with_system(debug_boundaries.system()),
            );
    }
}