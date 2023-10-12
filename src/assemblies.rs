use crate::*;

pub enum Power {
    Mechanical(f32),
    Thermal(f32),
    Electrical(f32)
}

#[derive(Component)]
pub struct Assembly {
    pub production: Good
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
                production: Good::Paper
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(50.0, 50.0)),
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
                production: Good::Paper
            },
            marker: PulpMill,
            sprite: SpriteBundle {
                ..default()
            },
        }
    }
}

pub fn test(
     q_assembly: Query<&Assembly>,
) {
    for assembly in q_assembly.iter() {
        match assembly.production {
            Good::Paper => {

            }
        }
    }
}

#[derive(Component, Default)]
pub struct AssemblyGhost;

pub fn assembly_ghost_tracking(
    mut commands: Commands,
    mut q_assembly_ghost: Query<Option<&mut Transform>, With<AssemblyGhost>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if q_assembly_ghost.is_empty() {
        return;
    }

    let Some(mut transform) = q_assembly_ghost.single_mut() else { return; };
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(cursor_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let pos_x = (cursor_position.x / GRID_SIZE).round() * GRID_SIZE;
        let pos_y = (cursor_position.y / GRID_SIZE).round() * GRID_SIZE;
        transform.translation = vec3(pos_x, pos_y, transform.translation.z)
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
    for _ev in ev_hide_ghost.iter() {
        q_assembly_ghost.for_each(|entity| commands.entity(entity).despawn());
    }
}

#[derive(Event)]
pub struct ShowAssemblyGhost;

pub fn show_assembly_ghost(
    mut commands: Commands,
    mut ev_show_ghost: EventReader<ShowAssemblyGhost>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_window: Query<&Window, With<PrimaryWindow>>
) {
    for _ev in ev_show_ghost.iter() {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();
    
        if let Some(cursor_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let pos_x = (cursor_position.x / GRID_SIZE).round() * GRID_SIZE;
            let pos_y = (cursor_position.y / GRID_SIZE).round() * GRID_SIZE;

            commands.spawn((AssemblyBundle {
                sprite: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(pos_x, pos_y, -1.0),
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

) {
    if input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let pos_x = (world_position.x / GRID_SIZE).round() * GRID_SIZE;
            let pos_y = (world_position.y / GRID_SIZE).round() * GRID_SIZE;

            commands.spawn(AssemblyBundle {
                sprite: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(pos_x, pos_y, -1.0),
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
