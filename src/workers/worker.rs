use crate::*;
use workers::jobs::*;

#[derive(Component)]
pub struct PowerProduction {
    pub power: Power,
    pub output: Option<Entity>
}

#[derive(Component)]
pub struct Worker;
impl Clickable for Worker {}

#[derive(Component, PartialEq, Debug, Reflect)]
pub enum WorkerState {
    Paused,
    Working
}

#[derive(Bundle)]
pub struct WorkerBundle {
    pub marker: Worker,
    pub state: WorkerState,
    pub worker_items: ItemContainer,
    pub job: Job,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub pathfinding: MoveToTile,
    pub production: PowerProduction
}
impl Default for WorkerBundle {
    fn default() -> WorkerBundle {
        WorkerBundle {
            marker: Worker,
            state: WorkerState::Paused,
            job: Job {
                path: Vec::new(),
                complexity: 0.0
            },
            worker_items: ItemContainer { items: Vec::new(), max_items: 2 },
            production: PowerProduction {
                power: Power::Mechanical(100.0),
                output: None
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::ORANGE,
                    custom_size: Some(Vec2::new(25.0, 50.0)),
                    ..default()
                },
                ..default()
            },
            movement: Movement { speed_x: 1.25, speed_y: 1.25, input: None },
            pathfinding: MoveToTile { target: None, path: None, path_i: 0 }
        }
    }
}

pub fn place_worker(
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

            commands.spawn(WorkerBundle {
                sprite: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(pos.x, pos.y, -1.0),
                        ..WorkerBundle::default().sprite.transform
                    },
                    ..WorkerBundle::default().sprite
                },
                ..default()
            });
        }


    }
}

pub fn input_toggle_worker_mode(
    input: Res<Input<KeyCode>>,
    state: Res<State<PlayerState>>,
    mut next_state: ResMut<NextState<PlayerState>>
) {
    if input.just_pressed(KeyCode::H) {
        if state.get() == &PlayerState::Workers {
            next_state.set(PlayerState::None);
        } else {
            next_state.set(PlayerState::Workers);
            
        }
    }
}

pub fn toggle_worker_state(
    mut q_job_status: Query<&mut WorkerState>,
    mut ev_mouse_collision: EventReader<GenericMouseCollisionEvent<Worker>>,
    input: Res<Input<KeyCode>>
) {
    if input.just_pressed(KeyCode::Space) {
        if let Some(ev) = ev_mouse_collision.iter().next() {
            if let Some((_, entity)) = ev.collision {
                if let Ok(mut job_status) = q_job_status.get_mut(entity) {
                    if *job_status == WorkerState::Working {
                        *job_status = WorkerState::Paused;
                    } else {
                        *job_status = WorkerState::Working;
                    }
                }
            }
        }
    }
}

#[derive(Event)]
pub struct WorkerPickUpItemEvent {
    pub worker: Entity,
    pub item: Entity
}

pub fn worker_pick_up_item(
    mut commands: Commands,
    mut q_worker_item_container: Query<&mut ItemContainer, With<Worker>>,
    mut ev_pick_up: EventReader<WorkerPickUpItemEvent>
) {
    for ev in ev_pick_up.iter() {
        if let Ok(mut container) = q_worker_item_container.get_mut(ev.worker) {
            if let Ok(_) = container.add_item(Some(ev.item)) { 
                commands.entity(ev.worker).add_child(ev.item);
            }
        }
    }
}

#[derive(Event)]
pub struct WorkerDropItemEvent {
    pub worker: Entity,
    pub item: Entity,
    pub container: Option<Entity>
}

pub fn worker_drop_item(
    mut commands: Commands,
    mut q_item_containers: Query<&mut ItemContainer, Without<Worker>>,
    mut ev_drop: EventReader<WorkerDropItemEvent>
) {
    for ev in ev_drop.iter() {
        if let Ok(mut worker_container) = q_item_containers.get_mut(ev.worker) {
            if let Ok(_) = worker_container.remove_item(Some(ev.item)) { 
                commands.entity(ev.worker).remove_children([ev.item].as_slice());
                if let Some(container_entity) = ev.container {
                    if let Ok(mut container) = q_item_containers.get_mut(container_entity) {
                        if let Ok(_) = container.add_item(Some(ev.item)) {}
                    }
                }
            }
        }
    }
}