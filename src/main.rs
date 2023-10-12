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

pub const GRID_SIZE: f32 = 15.0;

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

#[derive(Component)]
struct Factory;
const map_size: TilemapSize = TilemapSize { x: 100, y: 100 };

pub fn factory_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles_map.png");

    commands.spawn((Camera2dBundle::default(), MainCamera));

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    helpers::filling::fill_tilemap(TileTextureIndex(8), map_size, TilemapId(tilemap_entity), &mut commands, &mut tile_storage);

    let tile_size: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size: TilemapGridSize = tile_size.into();
    let map_type: TilemapType = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, -100.0),
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
