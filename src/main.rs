use std::{cmp::{min, max}, time::Duration};

use bevy::{prelude::*, window::PrimaryWindow, math::vec3, sprite::collide_aabb::{self, Collision}, time::common_conditions::{on_fixed_timer, on_timer}};
use bevy_ecs_tilemap::{prelude::*, helpers::{hex_grid::neighbors, square_grid::neighbors::Neighbors}};
use pathfinding::prelude::astar;
use bevy_inspector_egui::quick::{WorldInspectorPlugin, StateInspectorPlugin};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use kayak_ui::{
    prelude::{widgets::*, *},
    CameraUIKayak,
};

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

mod ui;
use ui::*;

mod hud;
use hud::*;

mod day_cycle;
use day_cycle::*;

const GRID_SIZE: TilemapSize = TilemapSize { x: 100, y: 100 };
const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum PlayerState {
    #[default]
    None,
    Assemblies,
    Workers,
    Jobs,
    Receivables,
    TradeDepot,
    Power
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default, Reflect)]
pub enum PlacementState {
    Blocked,
    #[default]
    Allowed,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(WorldInspectorPlugin::default())
        // .add_plugins(ResourceInspectorPlugin::<Economy>::default())
        .add_plugins(TilemapPlugin)
        .add_plugins((KayakContextPlugin, KayakWidgets))

        .add_plugins(AssembliesPlugin)
        .add_plugins(WorkerPlugin)
        .add_plugins(ItemPlugin)
        .add_plugins(MoneyPlugin)

        .add_systems(Update, day_timer_system.run_if(in_state(DayCycleState::Day)))
        .add_systems(OnEnter(DayCycleState::Night), (
            |mut day_timer: ResMut<DayTimer>| {
                day_timer.day_timer.reset();
                day_timer.day_count += 1;
            },
            reset_factory
        ))
        .add_state::<DayCycleState>()
        .insert_resource(DayTimer::default())
        .insert_resource(ReceivableSelections::default())

        .add_systems(Startup, (factory_setup, apply_deferred, hud_setup).chain())
        .add_systems(FixedUpdate, (
            (player_movement).run_if(not(in_state(PlayerState::Power))),
            move_entities
        ).run_if(in_state(DayCycleState::Day)))
        .add_systems(OnEnter(PlayerState::Power), |mut query: Query<(&mut Movement, &mut PlayerPowerProduction), With<Player>>| {
            let (mut movement, mut power) = query.single_mut();
            movement.input = None;
            power.input_count = 0;
        })
        .add_systems(Update, (
            player_pickup_item,
            player_drop_item,
            player_power_assembly,
            activate_power_mode_on_click
        ).run_if(in_state(DayCycleState::Day)))
        .add_systems(Update, (camera_follow, camera_scroll_zoom).run_if(in_state(DayCycleState::Day)))
        .add_systems(PostUpdate, despawn_later_system)
        .add_systems(Update, input_reset_player_mode)
        .add_systems(Update, (sprite_direction_system, movement_animation_system))
        .insert_resource(AssemblyPowerSelection::default())

        .add_systems(PostUpdate, (set_tilemap_collisions, debug_collision).run_if(on_timer(Duration::from_secs_f32(0.1))))

        .add_systems(Update, (hide_hover_ghost, hover_ghost_tracking))
        .add_event::<HideHoverGhost>()

        .add_state::<PlayerState>()
        .add_state::<PlacementState>()
        .add_systems(PreUpdate, (set_mouse_pos_res, set_mouse_tile_res))
        .insert_resource(MousePos(Vec2::ZERO))
        .insert_resource(MouseTile(TilePos::new(0, 0)))
        .insert_resource(SpriteStorage {
            workers: vec![],
            pulp_mill: Handle::default(),
            paper_press: Handle::default(),
            paper_drier: Handle::default(),
        })
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

pub fn input_reset_player_mode(
    mut next_state: ResMut<NextState<PlayerState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(PlayerState::None);
    }
}

#[derive(Resource)]
pub struct SpriteStorage {
    pub workers: Vec<Handle<TextureAtlas>>,
    pub pulp_mill: Handle<Image>,
    pub paper_press: Handle<Image>,
    pub paper_drier: Handle<Image>,
}

#[derive(Component)]
struct Factory;

pub fn factory_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut sprites: ResMut<SpriteStorage>,
) {
    for i in 0..2 {
        let texture_handle: Handle<Image> = asset_server.load(format!("Worker {}.png", i));
        let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 64.0), 24, 3, None, None);

        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        sprites.workers.push(texture_atlas_handle.clone());
    }

    sprites.pulp_mill = asset_server.load("Pulp Mill Icon.png");
    sprites.paper_press = asset_server.load("Paper Press Icon.png");
    sprites.paper_drier = asset_server.load("Paper Drier Icon.png");

    let texture_handle: Handle<Image> = asset_server.load("tiles_map.png");

    commands.spawn((Camera2dBundle::default(), MainCamera, CameraUIKayak));

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

    let texture_handle = asset_server.load("Character placeholder.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 64.0), 24, 3, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(PlayerBundle {
        marker: Player,
        camera_follow: CameraFollow::default(),
        movement: Movement { speed_x: 2.0, speed_y: 2.0, input: None },
        direction: SpriteDirection::default(),
        sprite_sheet: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(3),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        container: ItemContainer { 
            items: Vec::new(),
            item_type: None,
            max_items: 2
        },
        production: {PlayerPowerProduction {
            max_output: Power::Mechanical(60.0),
            min_output: Power::Mechanical(5.0),
            count_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            input_count: 0,
            no_input_count: 0,
        }}
    });

    let mut output_bundle = ContainerOutputSelectorBundle::new(asset_server.clone());
    output_bundle.sprite.transform.translation = Vec3::new(0.0, -42.0, 1.0);
    let output_entity = commands.spawn(output_bundle).id();
    commands.spawn(ItemReceivableBundle::from_translation(vec3(6.0 * TILE_SIZE.x, 8.0 * TILE_SIZE.y, 1.0))).push_children(&[output_entity]);

    let mut input_bundle = ContainerInputSelectorBundle::new(asset_server.clone());
    input_bundle.sprite.transform.translation = Vec3::new(0.0, 42.0, 1.0);
    let input_entity = commands.spawn(input_bundle).id();

    commands.spawn(TradeDepotBundle::from_translation(vec3(-4.0 * TILE_SIZE.x, -12.0 * TILE_SIZE.y, 1.0))).push_children(&[input_entity]);
}

pub fn reset_factory(
    mut commands: Commands,
    mut q_workers: Query<(&mut Transform, &mut Job), (With<Worker>, Without<Player>)>,
    mut q_player: Query<&mut Transform, (With<Player>, Without<Worker>)>,
    mut player_state: ResMut<NextState<PlayerState>>,
) {
    let mut player = q_player.get_single_mut().unwrap();
    player.translation.x = 0.0;
    player.translation.y = 0.0;

    player_state.set(PlayerState::None);

    for (mut worker_transform, mut worker_job) in q_workers.iter_mut() {
        worker_transform.translation.x = 0.0;
        worker_transform.translation.y = 0.0;
    }
}
