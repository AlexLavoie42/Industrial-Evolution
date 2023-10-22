use crate::*;

#[derive(Component, Default)]
pub struct AssemblyGhost;

pub fn assembly_ghost_tracking(
    mut q_assembly_ghost: Query<Option<&mut Transform>, With<AssemblyGhost>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &Transform
    ), Without<AssemblyGhost>>
) {
    if q_assembly_ghost.is_empty() {
        return;
    }

    let Some(mut transform) = q_assembly_ghost.single_mut() else { return; };
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
pub struct HideAssemblyGhost;

pub fn hide_assembly_ghost(
    mut commands: Commands,
    mut ev_hide_ghost: EventReader<HideAssemblyGhost>,
    q_assembly_ghost: Query<Entity, With<AssemblyGhost>>
) {
    for _ in ev_hide_ghost.iter() {
        q_assembly_ghost.for_each(|entity| commands.entity(entity).despawn());
    }
}

#[derive(Event)]
pub struct ShowAssemblyGhost;

pub fn show_assembly_ghost(
    mut commands: Commands,
    mut ev_show_ghost: EventReader<ShowAssemblyGhost>,
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

            commands.spawn((AssemblyBundle {
                sprite: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(pos.x, pos.y, -1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::YELLOW.with_a(0.5),
                        ..AssemblyBundle::default().sprite.sprite
                    },
                    ..AssemblyBundle::default().sprite
                },
                ..default()
            }, AssemblyGhost));
        } else {
            commands.spawn((AssemblyBundle {
                ..default()
            }, AssemblyGhost));
        }
    } 
}