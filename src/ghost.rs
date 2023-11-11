use std::marker::PhantomData;

use crate::*;

#[derive(Component, Default)]
pub struct Ghost;

#[derive(Component, Default)]
pub struct HoverGhost;

pub fn hover_ghost_tracking(
    mut q_assembly_ghost: Query<Option<&mut Transform>, With<HoverGhost>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &Transform
    ), Without<HoverGhost>>
) {
    if q_assembly_ghost.is_empty() {
        return;
    }

    let Ok(Some(mut transform)) = q_assembly_ghost.get_single_mut() else { return; };
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    let (tilemap_size, grid_size, map_type, map_transform) = tilemap_q.single();
    if let Some(tile_pos) = get_mouse_tile(window, camera, camera_transform, tilemap_size, grid_size, map_type, map_transform)
    {
        let cursor_position = get_tile_world_pos(&tile_pos, map_transform, grid_size, map_type);
        transform.translation = vec3(cursor_position.x, cursor_position.y, transform.translation.z)
    }
}

#[derive(Event)]
pub struct HideHoverGhost;

pub fn hide_hover_ghost(
    mut commands: Commands,
    mut ev_hide_ghost: EventReader<HideHoverGhost>,
    q_hover_ghost: Query<Entity, With<HoverGhost>>
) {
    for ev in ev_hide_ghost.iter() {
        let Ok(entity) = q_hover_ghost.get_single() else { continue; };
        commands.entity(entity).despawn();
    }
}

#[derive(Event)]
pub struct ShowHoverGhost<T: GetSpriteBundle + Default> {
    pub bundle: PhantomData<T>
}

pub trait GetSpriteBundle: Bundle {
    fn get_sprite_bundle(&self) -> SpriteBundle;
}

pub fn show_hover_ghost<T: GetSpriteBundle + Default>(
    mut commands: Commands,
    mut ev_show_ghost: EventReader<ShowHoverGhost<T>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &Transform
    )>
) {
    for _ev in ev_show_ghost.iter() {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();
        let (tilemap_size, grid_size, map_type, map_transform) = tilemap_q.single();
    
        if let Some(tile_pos) = get_mouse_tile(window, camera, camera_transform, tilemap_size, grid_size, map_type, map_transform)
        {
            let pos = get_tile_world_pos(&tile_pos, map_transform, grid_size, map_type);
            let mut sprite_bundle = T::default().get_sprite_bundle();
            sprite_bundle.transform.translation = vec3(pos.x, pos.y, sprite_bundle.transform.translation.z);
            sprite_bundle.sprite.color.set_a(0.5);
            commands.spawn((sprite_bundle, HoverGhost::default()));
        } else {
            commands.spawn((AssemblyBundle {
                ..default()
            }, Ghost));
        }
    } 
}