use std::{marker::PhantomData, fmt::Debug};

use crate::*;

#[derive(Resource)]
pub struct MousePos(pub Vec2);

#[derive(Resource)]
pub struct MouseTile(pub TilePos);

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

pub fn get_world_pos(
    pos: Vec2,
    map_transform: &Transform,
) -> Vec2 {
    let pos4 = Vec4::from((pos, 0.0, 1.0));
    let map_pos = map_transform.compute_matrix().inverse() * pos4;
    Vec2 {
        x: map_pos.x,
        y: map_pos.y,
    }
}

pub fn get_corner_tile_pos(
    pos: Vec2,
    size: IVec2
) -> Vec2 {
    return pos + Vec2::new((((size.x as f32) / 2.0) - 0.5) * TILE_SIZE.x, (((size.y as f32) / 2.0) - 0.5) * TILE_SIZE.y);
}

pub fn is_near_tile(
    point: TilePos,
    target: TilePos,
    map_size: &TilemapSize
) -> bool {
    return point == target || Neighbors::get_square_neighboring_positions(&point, map_size, true).iter().any(|p| { target == *p });
}

pub fn is_near_tile_group(
    point: TilePos,
    target: TilePos,
    target_size: IVec2,
    map_size: &TilemapSize
) -> bool {
    for x in 0..target_size.x {
        if is_near_tile(TilePos { x: point.x + x as u32, y: point.y }, target, map_size) {
            return true;
        }
        if is_near_tile(TilePos { x: point.x + x as u32, y: point.y + target_size.y as u32 }, target, map_size) {
            return true;
        }
    }
    for y in 0..target_size.y {
        if is_near_tile(TilePos { x: point.x, y: point.y + y as u32 }, target, map_size) {
            return true;
        }
        if is_near_tile(TilePos { x: point.x + target_size.x as u32, y: point.y + y as u32 }, target, map_size) {
            return true;
        }
    }
    return false;
}

#[derive(Event, Debug)]
pub struct GenericMouseCollisionEvent<T: Component> {
    pub collision: Option<(Collision, Entity)>,
    marker: PhantomData<T>
}

#[derive(Event, Debug)]
pub struct MouseCollisionEvent {
    pub collision: Option<(Collision, Entity)>,
}

pub trait MouseCollider: Component {
    fn check_mouse_collision(&self, mouse_pos: &MousePos, transform: &GlobalTransform, sprite: &Sprite) -> Option<Collision>;
}

pub trait Clickable: Component {}

impl<T: Clickable> MouseCollider for T {
    fn check_mouse_collision(&self, mouse_pos: &MousePos, transform: &GlobalTransform, sprite: &Sprite) -> Option<Collision> {
        let mouse_vec = Vec3 {
            x: mouse_pos.0.x,
            y: mouse_pos.0.y,
            z: 0.0,
        };
        // TODO: Proper size / proper colliders / tilemap collision?
        let mouse_collision = collide_aabb::collide(
            transform.translation(),
            sprite.custom_size.unwrap(),
            mouse_vec,
            Vec2 { x: 1.0, y: 1.0 },
        );
        return mouse_collision;
    }
}

// Not working for child entities?
pub fn mouse_collision_system<T: MouseCollider + Debug>(
    components: Query<(&T, &GlobalTransform, &Sprite, Entity)>,
    sheet_components: Query<(&T, &GlobalTransform, &TextureAtlasSprite, Entity)>,
    mouse_pos: Res<MousePos>,
    mut ev_generic: EventWriter<GenericMouseCollisionEvent<T>>,
    mut ev_all: EventWriter<MouseCollisionEvent>
) {
    for (component, transform, sprite, entity) in components.iter() {
        if let Some(collision) = component.check_mouse_collision(&mouse_pos, transform, sprite) {
            let ev = MouseCollisionEvent {
                collision: Some((collision, entity))
            };
            ev_all.send(ev);

            let ev = GenericMouseCollisionEvent {
                collision: Some((collision, entity)),
                marker: PhantomData::<T>
            };
            ev_generic.send(ev);
        }
    }
    for (component, transform, sprite, entity) in sheet_components.iter() {
        if let Some(collision) = component.check_mouse_collision(&mouse_pos, transform, &Sprite {
            custom_size: sprite.custom_size,
            ..Default::default()
        }) {
            let ev = MouseCollisionEvent {
                collision: Some((collision, entity))
            };
            ev_all.send(ev);

            let ev = GenericMouseCollisionEvent {
                collision: Some((collision, entity)),
                marker: PhantomData::<T>
            };
            ev_generic.send(ev);
        }
    }
}

#[derive(Component)]
pub struct DespawnLater;

pub fn despawn_later_system(
    mut commands: Commands,
    q_despawn_entities: Query<Entity, With<DespawnLater>>,
) {
    for entity in q_despawn_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

trait BundleWithTranslate {
    fn bundle_with_translate(&mut self, translation: Vec3) -> Self;
}
