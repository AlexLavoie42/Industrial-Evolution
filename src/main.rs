use bevy::{prelude::*, window::PrimaryWindow, math::vec3};
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
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, factory_setup)
        .add_systems(FixedUpdate, player_movement)
        .add_systems(Update, camera_follow)
        .add_state::<PlayerState>()
        .add_systems(OnEnter(PlayerState::Assemblies),
         |mut ev_show_ghost: EventWriter<ShowAssemblyGhost>| {
            ev_show_ghost.send(ShowAssemblyGhost);
        })
        .add_systems(OnExit(PlayerState::Assemblies),
         |mut ev_hide_ghost: EventWriter<HideAssemblyGhost>| {
            ev_hide_ghost.send(HideAssemblyGhost);
        })
        .add_systems(Update, 
            (
                (place_assembly, assembly_ghost_tracking).run_if(in_state(PlayerState::Assemblies)),
                place_worker.run_if(in_state(PlayerState::Workers)),
                (input_toggle_assembly_mode, input_toggle_worker_mode),
                show_assembly_ghost,
                hide_assembly_ghost
            )
        )
        .add_event::<HideAssemblyGhost>()
        .add_event::<ShowAssemblyGhost>()
        .run();
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

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraFollow {
    pub lerp: f32
}

impl CameraFollow {
    fn default() -> Self {
        Self { lerp: 0.1 }
    }
}

#[derive(Component)]
pub struct Movement {
    pub speed_x: f32,
    pub speed_y: f32
}

pub fn camera_follow(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<CameraFollow>)>,
    mut follow_query: Query<&Transform, (With<CameraFollow>, Without<MainCamera>)>
) {
    let mut cam_transform: Mut<'_, Transform> = camera_query.single_mut();
    let player_transform: &Transform = follow_query.single_mut();

    cam_transform.translation = cam_transform.translation.lerp(player_transform.translation, 0.1);
}
