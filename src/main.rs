use bevy::{prelude::*, window::PrimaryWindow, math::vec3, sprite::collide_aabb::{self, Collision}, ecs::query::ReadOnlyWorldQuery};
use bevy_ecs_tilemap::prelude::*;

mod player;
use player::*;

mod assemblies;
use assemblies::*;

mod workers;
use workers::*;

mod items;
use items::*;

const GRID_SIZE: TilemapSize = TilemapSize { x: 100, y: 100 };

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum PlayerState {
    #[default]
    None,
    Assemblies,
    Workers,
    Jobs
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_plugins(AssembliesPlugin)
        .add_plugins(WorkerPlugin)
        .add_systems(Startup, factory_setup)
        .add_systems(FixedUpdate, player_movement)
        .add_systems(Update, camera_follow)
        .add_state::<PlayerState>()
        .add_systems(Update, (set_mouse_pos_res, set_mouse_tile_res))
        .insert_resource(MousePos(Vec2::ZERO))
        .insert_resource(MouseTile(TilePos::new(0, 0)))
        .add_event::<MouseCollisionEvent>()
        .run();
}

#[derive(Resource)]
pub struct MousePos(Vec2);

#[derive(Resource)]
pub struct MouseTile(TilePos);

pub fn set_mouse_pos_res(
    mut mouse_pos: ResMut<MousePos>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = q_window.single();
    let (camera, camera_transform) = q_camera.single();
    if let Some(pos) = get_mouse_world_pos(&window, &camera, &camera_transform) {
        mouse_pos.0 = pos;
    }
}

pub fn set_mouse_tile_res(
    mut mouse_tile: ResMut<MouseTile>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &Transform
    )>
) {
    let window = q_window.single();
    let (camera, camera_transform) = q_camera.single();
    let (tilemap_size, grid_size, map_type, map_transform) = tilemap_q.single();
    if let Some(tile_pos) = get_mouse_tile(
        &window,
        &camera,
        &camera_transform,
        &tilemap_size,
        &grid_size,
        &map_type,
        &map_transform
    ) {
        mouse_tile.0 = tile_pos;
    }
}

pub fn get_mouse_world_pos(window: &Window, camera: &Camera, camera_transform: &GlobalTransform) -> Option<Vec2> {
    window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
}

pub fn get_mouse_tile(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    tilemap_size: &TilemapSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    map_transform: &Transform
) -> Option<TilePos> {
    if let Some(cursor_pos) = get_mouse_world_pos(window, camera, camera_transform) {
        // Once we have a world position we can transform it into a possible tile position.
        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 0.0 and 1.0
            let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            Vec2 {
                x: cursor_in_map_pos.x,
                y: cursor_in_map_pos.y,
            }
        };
        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, tilemap_size, grid_size, map_type)
        {
            return Some(tile_pos);
        } else {
            return None;
        }
    } else {
        return None;
    }
}

pub fn get_tile_world_pos(
    position: &TilePos,
    map_transform: &Transform,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType
) -> Vec2 {
    let pos = Vec4::from((position.center_in_world(grid_size, map_type), 0.0, 1.0));
    let world_pos = map_transform.compute_matrix() * pos;
    Vec2 {
        x: world_pos.x,
        y: world_pos.y,
    }
}

#[derive(Event)]
pub struct MouseCollisionEvent {
    pub collision: Option<(Collision, Entity)>,
}

pub trait MouseCollider: Component {
    fn check_collision(&self, mouse_pos: &MousePos, transform: &Transform, sprite: &Sprite) -> Option<Collision>;
}

impl<T: Component> MouseCollider for T {
    fn check_collision(&self, mouse_pos: &MousePos, transform: &Transform, sprite: &Sprite) -> Option<Collision> {
        let mouse_vec = Vec3 {
            x: mouse_pos.0.x,
            y: mouse_pos.0.y,
            z: 0.0,
        };
        // TODO: Proper size / proper colliders / tilemap collision?
        let mouse_collision = collide_aabb::collide(
            transform.translation,
            sprite.custom_size.unwrap(),
            mouse_vec,
            Vec2 { x: 1.0, y: 1.0 },
        );
        return mouse_collision;
    }
}

pub fn mouse_collision_system<T: MouseCollider>(
    components: Query<(&T, &Transform, &Sprite, Entity)>,
    mouse_pos: Res<MousePos>,
    mouse_input: Res<Input<MouseButton>>,
    mut events: EventWriter<MouseCollisionEvent>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (component, transform, sprite, entity) in components.iter() {
            if let Some(collision) = component.check_collision(&mouse_pos, transform, sprite) {
                events.send(MouseCollisionEvent {
                    collision: Some((collision, entity)),
                });
                break;
            }
        }
    }
}

#[derive(Component)]
struct Factory;

pub fn factory_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles_map.png");

    commands.spawn((Camera2dBundle::default(), MainCamera));

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(GRID_SIZE);

    helpers::filling::fill_tilemap(TileTextureIndex(8), GRID_SIZE, TilemapId(tilemap_entity), &mut commands, &mut tile_storage);

    let tile_size: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size: TilemapGridSize = tile_size.into();
    let map_type: TilemapType = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: GRID_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&GRID_SIZE, &grid_size, &map_type, -100.0),
        ..Default::default()
    });

    commands.spawn(PlayerBundle {
        marker: Player,
        camera_follow: CameraFollow::default(),
        movement: Movement { speed_x: 2.0, speed_y: 2.0 },
        sprite: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(18.0, 25.0)),
                color: Color::RED,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        }
    });
}
