use crate::*;
use workers::jobs::*;

#[derive(Component)]
pub struct PowerProduction {
    pub power: Power,
    pub output: Option<Entity>
}

#[derive(Component, Debug)]
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
    pub job_error: JobError,
    pub job_waiting: JobWaiting,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub pathfinding: MoveToTile,
    pub production: PowerProduction
}
impl GetGhostBundle for WorkerBundle {
    fn get_sprite_bundle(&self) -> SpriteBundle {
        self.sprite.clone()
    }
    fn get_tile_size(&self) -> Option<EntityTileSize> {
        None
    }
}
impl Default for WorkerBundle {
    fn default() -> WorkerBundle {
        WorkerBundle {
            marker: Worker,
            state: WorkerState::Paused,
            job: Job {
                path: Vec::new(),
                complexity: 0.0,
                current_job: None,
                lock: false
            },
            job_error: JobError::new(),
            job_waiting: JobWaiting(false),
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
    
        let Some(tile_pos) = get_mouse_tile(window, camera, camera_transform, tilemap_size, grid_size, map_type, map_transform) else { return };
        let pos = get_tile_world_pos(&tile_pos, map_transform, grid_size, map_type);

        commands.spawn(WorkerBundle {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(pos.x, pos.y, 5.0),
                    ..WorkerBundle::default().sprite.transform
                },
                ..WorkerBundle::default().sprite
            },
            ..default()
        });
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
        let Some(ev) = ev_mouse_collision.iter().next() else { return; };
        let Some((_, entity)) = ev.collision else { return; };
        let Ok(mut job_status) = q_job_status.get_mut(entity) else { return; };

        if *job_status == WorkerState::Working {
            *job_status = WorkerState::Paused;
        } else {
            *job_status = WorkerState::Working;
        }
    }
}

#[derive(Event)]
pub struct WorkerPickUpItemEvent {
    pub worker: Entity,
    pub item: Entity,
    pub tile_pos: TilePos,
    pub container: Option<Entity>
}

pub fn worker_pick_up_item(
    mut commands: Commands,
    mut q_item_transforms: Query<(&mut Transform, &GlobalTransform), (With<Item>, Without<Worker>)>,
    mut q_worker_item_container: Query<(&mut ItemContainer, &GlobalTransform, &mut Job, &mut JobError), (With<Worker>, Without<Item>)>,
    mut q_io_item_containers: Query<&mut ItemIOContainer>,
    mut q_item_containers: Query<&mut ItemContainer, Without<Worker>>,
    q_tilemap: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform), (Without<Worker>, Without<Item>)>,
    mut ev_pick_up: EventReader<WorkerPickUpItemEvent>
) {
    for ev in ev_pick_up.iter() {
        // TODO: Lock each item to avoid a race condition with multiple workers
        let Ok((mut container, worker_transform, mut job, mut job_error))
            = q_worker_item_container.get_mut(ev.worker) else { continue };
        let Ok((mut item_transform, item_g_transform)) = q_item_transforms.get_mut(ev.item) else {
            job_error.set_error("Item not found");
            continue;
        };
        if container.items.iter().any(|i| *i == Some(ev.item)) {
            if let Some(current_job_i) = job.current_job {
                let Some(current_job) = job.path.get_mut(current_job_i) else { continue; };
                current_job.job_status = JobStatus::Completed;
                job_error.clear_error();
            }
            continue;
        }

        let (map_size, grid_size, map_type, map_transform) = q_tilemap.single();

        let worker_world_pos = get_world_pos(Vec2 { x: worker_transform.translation().x, y: worker_transform.translation().y }, map_transform);
        let Some(worker_tile_pos) = TilePos::from_world_pos(&worker_world_pos, map_size, grid_size, map_type) else { continue; };

        if worker_tile_pos != ev.tile_pos {
            if let Some(current_job_i) = job.current_job {
                let Some(current_job) = job.path.get_mut(current_job_i) else { continue; };
                current_job.job_status = JobStatus::Active;
            }
            continue;
        }

        let Ok(_) = container.add_item(Some(ev.item)) else {
            if let Some(current_job_i) = job.current_job {
                let Some(current_job) = job.path.get_mut(current_job_i) else { continue; };
                current_job.job_status = JobStatus::Active;
            }
            continue;
        }; 
        commands.entity(ev.worker).add_child(ev.item);
        item_transform.translation = Vec3::new(16.0, 8.0, item_transform.translation.z);
        println!("Picked up item {:?}", container);
        
        if let Some(container_entity) = ev.container {
            if let Ok(mut container) = q_item_containers.get_mut(container_entity) {
                if let Err(err) = container.remove_item(Some(ev.item)) {}
            } else if let Ok(mut container) = q_io_item_containers.get_mut(container_entity) {
                if let Err(err) = container.output.remove_item(Some(ev.item)) {}
            }
        }
        if let Some(current_job_i) = job.current_job {
            let Some(current_job) = job.path.get_mut(current_job_i) else { continue; };
            current_job.job_status = JobStatus::Completed;
            job_error.clear_error();
        }
    }
}

#[derive(Event)]
pub struct WorkerDropItemEvent {
    pub worker: Entity,
    pub item: Entity,
    pub container: Option<Entity>,
}

pub fn worker_drop_item(
    mut commands: Commands,
    mut q_item_transforms: Query<&mut Transform, With<Item>>,
    mut q_item_containers: Query<&mut ItemContainer, Without<Worker>>,
    mut q_worker_containers: Query<(&mut ItemContainer, &mut Job, &mut JobError), With<Worker>>,
    mut q_assembly_item_containers: Query<&mut ItemIOContainer>,
    mut ev_drop: EventReader<WorkerDropItemEvent>
) {
    for ev in ev_drop.iter() {
        let Ok((mut worker_container, mut job, mut job_error)) = q_worker_containers.get_mut(ev.worker) else {
            continue;
        };
        // TODO: Drop item with no container?
        let (Some(container_entity), Ok(mut item_transform)) = (ev.container, q_item_transforms.get_mut(ev.item)) else {
            job_error.set_error("Item or Container not found");
            continue;
        };
        if worker_container.items.iter().all(|i| *i != Some(ev.item)) {
            // TODO: Auto skip?
            job_error.set_error("Item not found in worker");
            continue;
        }

        println!("Dropping item {:?} into {:?}", ev.item, container_entity);

        let mut drop_item = |container: &mut ItemContainer, worker_container: &mut ItemContainer| {
            // TODO: Safe child push (check entity exists)
            commands.entity(ev.worker).remove_children([ev.item].as_slice());
            commands.entity(container_entity).push_children(&[ev.item]);
            item_transform.translation = Vec3::new(0.0, 0.0, item_transform.translation.z);
            println!("Dropping item {:?} into {:?}", ev.item, container_entity);
            
            if let Err(err) = container.add_item(Some(ev.item)) {
                // TODO: Waiting?
                job_error.set_error(format!("Error adding item to container: {err}").as_str());
                commands.entity(container_entity).remove_children([ev.item].as_slice());
                commands.entity(ev.worker).push_children(&[ev.item]);
                if let Err(err) = worker_container.add_item(Some(ev.item)) {
                    job_error.set_error(format!("Error picking item back up: {err}").as_str());
                }
            } else {
                if let Some(current_job_i) = job.current_job {
                    if let Some(current_job) = job.path.get_mut(current_job_i) {
                        current_job.job_status = JobStatus::Completed;
                        job_error.clear_error();
                    }
                }
            }
        };

        if let Ok(mut container) = q_item_containers.get_mut(container_entity) {
            if container.items.iter().any(|i| *i == Some(ev.item)) {
                // TODO: Auto skip?
                job_error.set_error("Item already in container");
                continue;
            }

            let item_res = worker_container.remove_item(Some(ev.item));
            if let Ok(_) = item_res {
                drop_item(&mut container, &mut worker_container);
            }
        } else if let Ok(mut container) = q_assembly_item_containers.get_mut(container_entity) {
            if container.input.items.iter().any(|i| *i == Some(ev.item)) {
                // TODO: Auto skip?
                job_error.set_error("Item already in assembly");
                continue;
            }
            
            let item_res = worker_container.remove_item(Some(ev.item));
            if let Ok(_) = item_res {
                drop_item(&mut container.input, &mut worker_container);
            } else if let Err(err) = item_res {
                // TODO: Waiting?
                job_error.set_error(err);
            }
        }
    }
}