use bevy::{prelude::*, window::PrimaryWindow, math::vec3, sprite::collide_aabb::{self, Collision}};
use bevy_ecs_tilemap::prelude::*;
use pathfinding::prelude::astar;

mod player;
use player::*;

mod assemblies;
use assemblies::*;

mod workers;
use workers::*;

mod items;
use items::*;

mod tile_pathfinding;
use tile_pathfinding::*;

mod utils;
use utils::*;

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
        .add_systems(FixedUpdate, (player_movement, move_entities))
        .add_systems(Update, camera_follow)
        .add_state::<PlayerState>()
        .add_systems(PreUpdate, (set_mouse_pos_res, set_mouse_tile_res))
        .insert_resource(MousePos(Vec2::ZERO))
        .insert_resource(MouseTile(TilePos::new(0, 0)))
        .run();
}

#[derive(Component, Debug)]
pub struct Path (Vec<TilePos>);

#[derive(Component)]
pub struct TileMapCollision;

#[derive(Component)]
pub struct SolidEntity;

pub fn set_tilemap_collisions (
    mut commands: Commands,
    q_tilemap: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &TileStorage)>,
    q_collisions: Query<(Entity, &TilePos), With<TileMapCollision>>,
    q_solid: Query<&Transform, With<SolidEntity>>,
) {
    for (entity, _) in q_collisions.iter() {
        commands.entity(entity).remove::<TileMapCollision>();
    }
    for transform in q_solid.iter() {
        let world_pos = Vec2 {
            x: transform.translation.x,
            y: transform.translation.y
        };
        let (map_size, grid_size, map_type, tile_storage) = q_tilemap.single();
        // TODO: Multi-tile entities
        if let Some(tile_pos) = TilePos::from_world_pos(&world_pos, map_size, grid_size, map_type) {
            if let Some(tile) = tile_storage.get(&tile_pos) {
                commands.entity(tile).insert(TileMapCollision);
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

    let tilemap_id = TilemapId(tilemap_entity);
    commands.entity(tilemap_id.0).with_children(|parent| {
        for x in 0..GRID_SIZE.x {
            for y in 0..GRID_SIZE.y {
                let tile_pos = TilePos { x, y };
                let tile_entity = parent
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture_index: TileTextureIndex(8),
                        ..Default::default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });

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
        movement: Movement { speed_x: 2.0, speed_y: 2.0, input: None },
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
