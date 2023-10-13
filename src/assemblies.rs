use crate::*;

pub struct AssembliesPlugin;
impl Plugin for AssembliesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayerState::Assemblies),
                |mut ev_show_ghost: EventWriter<ShowAssemblyGhost>| {
                    ev_show_ghost.send(ShowAssemblyGhost);
                }
            )
            .add_systems(OnExit(PlayerState::Assemblies),
                |mut ev_hide_ghost: EventWriter<HideAssemblyGhost>| {
                    ev_hide_ghost.send(HideAssemblyGhost);
                }
            )
            .add_systems(Update,
            (
                (place_assembly, assembly_ghost_tracking).run_if(in_state(PlayerState::Assemblies)),
                input_toggle_assembly_mode,
                show_assembly_ghost,
                hide_assembly_ghost
            ))
            .add_event::<HideAssemblyGhost>()
            .add_event::<ShowAssemblyGhost>();
    }
}

pub enum Power {
    Mechanical(f32),
    Thermal(f32),
    Electrical(f32)
}

#[derive(Component)]
pub struct Assembly {
    pub production: Option<Good>,
    pub resource: Option<items::Material>,
    pub work: Option<Power>
}

#[derive(Bundle)]
pub struct AssemblyBundle {
    pub marker: Assembly,
    pub sprite: SpriteBundle
}
impl Default for AssemblyBundle {
    fn default() -> AssemblyBundle {
        AssemblyBundle {
            marker: Assembly {
                production: None,
                resource: None,
                work: None
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            }
        }
    }
}

#[derive(Component)]
pub struct PulpMill;

#[derive(Bundle)]
pub struct PulpMillBundle {
    pub assembly: Assembly,
    pub sprite: SpriteBundle,
    pub marker: PulpMill
}
impl Default for PulpMillBundle {
    fn default() -> PulpMillBundle {
        PulpMillBundle {
            assembly: Assembly {
                production: Some(Good::Paper),
                resource: Some(items::Material::Pulp),
                work: Some(Power::Mechanical(10.0))
            },
            marker: PulpMill,
            sprite: SpriteBundle {
                ..default()
            },
        }
    }
}

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

pub fn input_toggle_assembly_mode(
    input: Res<Input<KeyCode>>,
    state: Res<State<PlayerState>>,
    mut next_state: ResMut<NextState<PlayerState>>
) {
    if input.just_pressed(KeyCode::R) {
        if state.get() == &PlayerState::Assemblies {
            next_state.set(PlayerState::None);
        } else {
            next_state.set(PlayerState::Assemblies);
            
        }
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

pub fn place_assembly(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &Transform
    )>
) {
    if input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();
        let (tilemap_size, grid_size, map_type, map_transform) = tilemap_q.single();

        if let Some(tile_pos) = get_mouse_tile(window, camera, camera_transform, tilemap_size, grid_size, map_type, map_transform)
        {
            let pos = get_tile_world_pos(&tile_pos, map_transform, grid_size, map_type);
            commands.spawn(AssemblyBundle {
                sprite: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(pos.x, pos.y, -1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::YELLOW,
                        custom_size: Some(Vec2::new(50.0, 50.0)),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
        }
    }
}
