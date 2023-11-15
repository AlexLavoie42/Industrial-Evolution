use std::{cmp::{min, max}, time::Duration};

use bevy::{prelude::*, window::PrimaryWindow, math::vec3, sprite::collide_aabb::{self, Collision}, time::common_conditions::{on_fixed_timer, on_timer}};
use bevy_ecs_tilemap::{prelude::*, helpers::{hex_grid::neighbors, square_grid::neighbors::Neighbors}};
use pathfinding::prelude::astar;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

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

mod money;
use money::*;

mod ghost;
use ghost::*;

const GRID_SIZE: TilemapSize = TilemapSize { x: 100, y: 100 };
const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum PlayerState {
    #[default]
    None,
    Assemblies,
    Workers,
    Jobs,
    Recievables,
    TradeDepot
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(ResourceInspectorPlugin::<Economy>::default())
        .add_plugins(ResourceInspectorPlugin::<PlayerMoney>::default())
        .add_plugins(ResourceInspectorPlugin::<SelectedAssembly>::default())
        .add_plugins(TilemapPlugin)

        .add_plugins(AssembliesPlugin)
        .add_plugins(WorkerPlugin)
        .add_plugins(ItemPlugin)
        .add_plugins(MoneyPlugin)

        .add_systems(Startup, factory_setup)
        .add_systems(FixedUpdate, (player_movement, move_entities, player_pickup_item, player_drop_item))
        .add_systems(Update, (camera_follow, camera_scroll_zoom))
        .add_systems(PostUpdate, despawn_later_system)

        .add_systems(PostUpdate, (set_tilemap_collisions, debug_collision).run_if(on_timer(Duration::from_secs_f32(0.1))))

        .add_systems(Update, (hide_hover_ghost, hover_ghost_tracking))
        .add_event::<HideHoverGhost>()

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

#[derive(Component, Clone, Copy)]
pub struct EntityTileSize (IVec2);

// TODO: Change detection?
pub fn set_tilemap_collisions (
    mut commands: Commands,
    q_tilemap: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &TileStorage, &Transform)>,
    q_collisions: Query<(Entity, &TilePos), With<TileMapCollision>>,
    q_solid: Query<(&Transform, Option<&EntityTileSize>), With<SolidEntity>>,
) {
    for (entity, _) in q_collisions.iter() {
        commands.entity(entity).remove::<TileMapCollision>();
    }
    for (transform, tile_size) in q_solid.iter() {
        let default_size = &EntityTileSize(IVec2::new(1, 1));
        let tile_size = tile_size.unwrap_or(default_size);
        let (map_size, grid_size, map_type, tile_storage, map_transform) = q_tilemap.single();

        let world_pos = get_world_pos(Vec2 { x: transform.translation.x, y: transform.translation.y }, map_transform)
            - Vec2::new((((tile_size.0.x as f32) / 2.0) - 0.5) * TILE_SIZE.x, (((tile_size.0.y as f32) / 2.0) - 0.5) * TILE_SIZE.y);

        // TODO: Rotation
        let Some(tile_pos) = TilePos::from_world_pos(&world_pos, map_size, grid_size, map_type) else { continue };
        let x = tile_size.0.x;
        let y = tile_size.0.y;

        for x in 0..x as u32 {
            for y in 0..y as u32 {
                let tile_pos = TilePos { x: tile_pos.x + x, y: tile_pos.y + y };
                let Some(tile) = tile_storage.get(&tile_pos) else { continue };
                commands.entity(tile).insert(TileMapCollision);
            }
        }
    }
}

fn debug_collision(
    mut q_collisions: Query<(&mut TileColor, Option<&TileMapCollision>)>,
) {
    for (mut color, collision) in q_collisions.iter_mut() {
        color.0 = if collision.is_some() {Color::RED} else {Color::WHITE};
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

    let grid_size: TilemapGridSize = TILE_SIZE.into();
    let map_type: TilemapType = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: GRID_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TILE_SIZE,
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
        },
        container: ItemContainer { 
            items: Vec::new(),
            max_items: 2
        }
    });
}
