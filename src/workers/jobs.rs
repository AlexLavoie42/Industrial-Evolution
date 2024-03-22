use std::time::Duration;

use crate::*;
use bevy_ecs_tilemap::helpers::transform;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
use workers::worker::*;

#[derive(Debug, Reflect, PartialEq, Clone)]
pub enum JobAction {
    Work {
        power: Power,
        assembly: Entity
    },
    Pickup {
        item: Entity,
    },
    ContainerPickup {
        container: Entity,
        pickup_amount: i32,
    },
    Drop {
        worker: Entity,
        input_container: Option<Entity>
    },
    Idle
}

#[derive(Component, Debug, Reflect)]
pub struct JobWaiting(pub bool);

#[derive(Debug, Reflect, PartialEq, Clone)]
pub struct JobPoint {
    pub id: u8,
    pub point: TilePos,
    pub point_size: IVec2,
    pub job_status: JobStatus,
    pub action: JobAction,
    pub timer: Option<Timer>
}

#[derive(Component, Debug, Reflect)]
pub struct Job {
    pub path: Vec<JobPoint>,
    pub complexity: f32,
    pub current_job: Option<usize>,
    pub lock: bool
}

#[derive(Component, Debug, PartialEq, Reflect, Clone)]
pub enum JobStatus {
    Active,
    Completed
}

#[derive(Resource)]
pub struct SelectedWorker {
    pub selected: Option<Entity>
}

pub fn activate_job_mode_on_click(
    q_worker: Query<Entity, With<workers::worker::Worker>>,
    mut mouse_collision: EventReader<GenericMouseCollisionEvent<Worker>>,
    input: Res<Input<MouseButton>>,
    player_state: Res<State<PlayerState>>,
    mut worker_selection: ResMut<SelectedWorker>,
    mut next_state: ResMut<NextState<PlayerState>>
    
) {
    for ev in mouse_collision.read() {
        if player_state.get() == &PlayerState::None {
            if input.just_pressed(MouseButton::Left) {
                if let Some((_, entity)) = ev.collision {
                    if let Ok(worker_entity) = q_worker.get(entity) {
                        worker_selection.selected = Some(worker_entity);
                        next_state.set(PlayerState::Jobs);
                        println!("Worker {} selected", worker_entity.index());
                    }
                }
            }
        }
    }
}

pub fn job_mode_creation(
    mut mouse_collision: EventReader<MouseCollisionEvent>,
    q_assemblies: Query<(Entity, &Transform, &EntityTileSize), With<Assembly>>,
    q_assembly_input: Query<(&ContainerInputSelector, &Parent, &GlobalTransform)>,
    q_assembly_output: Query<(&ContainerOutputSelector, &Parent, &GlobalTransform)>,
    q_items: Query<Entity, With<Item>>,
    mouse_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MouseTile>,
    selected_worker: Res<SelectedWorker>,
    mut q_worker: Query<(&mut Job, &PowerProduction), With<Worker>>,
    q_tilemap: Query<(&TilemapSize, &TilemapGridSize, &Transform, &TilemapType)>
) {
    let (tilemap_size, grid_size, map_transform, map_type) = q_tilemap.get_single().unwrap();
    let Some(worker_entity) = selected_worker.selected else { return; };
    let Ok((mut job, power_production)) = q_worker.get_mut(worker_entity) else { return; };
    for ev in mouse_collision.read() {
        if mouse_input.just_pressed(MouseButton::Left) {
            if let Some((_, entity)) = ev.collision {
                if let Ok((assembly, transform, tile_size)) = q_assemblies.get(entity) {
                    let assembly_world_pos = get_world_pos(Vec2 { x: transform.translation.x, y: transform.translation.y }, map_transform);
                    let assembly_pos = get_corner_tile_pos(assembly_world_pos, tile_size.0);
                    if let Some(assembly_tile_pos) = TilePos::from_world_pos(&assembly_pos, tilemap_size, grid_size, map_type) {
                        let action: JobAction = JobAction::Work {
                            power: power_production.power,
                            assembly,
                        };
                        let job_point = JobPoint {
                            id: job.path.len() as u8,
                            point: assembly_tile_pos,
                            job_status: JobStatus::Active,
                            point_size: tile_size.0,
                            action,
                            timer: None
                        };
                        job.path.push(job_point);
                        return;
                    }
                }
                if let Ok((assembly_input, parent, transform)) = q_assembly_input.get(entity) {
                    let job_point = JobPoint {
                        id: job.path.len() as u8,
                        point: TilePos::from_world_pos(&get_world_pos(transform.translation().xy(), map_transform), tilemap_size, grid_size, map_type).unwrap(),
                        point_size: IVec2::ONE,
                        job_status: JobStatus::Active,
                        action: JobAction::Drop {
                            input_container: Some(parent.get()),
                            worker: worker_entity
                        },
                        timer: None
                    };
                    job.path.push(job_point);
                    return;
                }
                if let Ok((assembly_output, parent, transform)) = q_assembly_output.get(entity) {
                    let job_point = JobPoint {
                        id: job.path.len() as u8,
                        point: TilePos::from_world_pos(&get_world_pos(transform.translation().xy(), map_transform), tilemap_size, grid_size, map_type).unwrap(),
                        point_size: IVec2::ONE,
                        job_status: JobStatus::Active,
                        action: JobAction::ContainerPickup {
                            container: parent.get(),
                            pickup_amount: 1
                        },
                        timer: None
                    };
                    job.path.push(job_point);
                    return;
                }
                // if let Ok(item) = q_items.get(entity) {
                //     let action: JobAction = JobAction::Pickup {
                //         item
                //     };
                //     let job_point = JobPoint {
                //         point: mouse_pos.0,
                //         point_size: IVec2::ONE,
                //         job_status: JobStatus::Active,
                //         action,
                //         timer: None
                //     };
                //     job.path.push(job_point);
                //     return;
                // }
            }
        }
    }
    
    if mouse_input.just_pressed(MouseButton::Left) {
        let job_point = JobPoint {
            id: job.path.len() as u8,
            point: mouse_pos.0,
            point_size: IVec2::ONE,
            job_status: JobStatus::Active,
            action: JobAction::Idle,
            timer: Some(Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once))
        };
        job.path.push(job_point);
    }
}

pub fn worker_iterate_jobs(
    mut q_jobs: Query<(&mut Job, &mut WorkerState)>,
) {
    for (mut job,  state) in q_jobs.iter_mut() {
        if *state == WorkerState::Paused {
            continue;
        }
        let active_jobs: Vec<&JobPoint> = job.path.iter().filter(|path| -> bool {
            return path.job_status == JobStatus::Active;
        }).collect();
        if active_jobs.len() == 0 && job.path.len() > 0 {
            for job_path in &mut job.path {
                job_path.job_status = JobStatus::Active;
            }
            job.current_job = Some(0);
        } else {
            let current_job_i = job.path.iter().position(|j| { j == active_jobs[0] }).unwrap_or(0);
            job.current_job = Some(current_job_i);
        }
    }
}

#[derive(Resource, Reflect, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct ItemJobLock {
    pub items: Vec<Entity>,
}

pub fn worker_do_job(
    time: Res<Time>,
    mut q_jobs: Query<(&mut Job, Entity, &Transform), With<Worker>>,
    q_tilemap: Query<(&Transform, &TilemapSize, &TilemapGridSize, &TilemapType)>,
    mut q_item_containers: Query<&mut ItemContainer>,
    mut q_assembly_containers: Query<&mut ItemIOContainer>,
    mut locked_items: ResMut<ItemJobLock>,
    mut ev_assembly_power: EventWriter<AssemblyPowerInput>,
    mut ev_item_pickup: EventWriter<WorkerPickUpItemEvent>,
    mut ev_item_drop: EventWriter<WorkerDropItemEvent>
) {
    let (map_transform, map_size, grid_size, map_type) = q_tilemap.single();
    for (mut job, worker_entity, transform) in q_jobs.iter_mut() {
        // Make sure each job is only run once per frame
        if job.lock {
            continue;
        }
        job.lock = true;
        let world_pos = get_world_pos(Vec2 { x: transform.translation.x, y: transform.translation.y }, map_transform);
        let tile_pos = TilePos::from_world_pos(&world_pos, map_size, grid_size, map_type);
        if let (Some(tile_pos), Some(current_job_i)) = (tile_pos, job.current_job) {
            if job.path.len() < current_job_i + 1 {
                job.lock = false;
                continue;
            }
            let current_job = &mut job.path[current_job_i];
            if is_near_tile_group(tile_pos, current_job.point,  current_job.point_size, map_size) {
                if let Some(timer) = &mut current_job.timer {
                    timer.tick(time.delta());
                    if !timer.finished() {
                        job.lock = false;
                        continue;
                    }
                    timer.reset();
                }
                if current_job.job_status != JobStatus::Active {
                    job.lock = false;
                    continue;
                }
                match current_job.action {
                    JobAction::Work { power, assembly } => {
                        ev_assembly_power.send(AssemblyPowerInput {
                            assembly,
                            source: worker_entity,
                            power,
                        });
                    },
                    JobAction::Idle => {
                        current_job.job_status = JobStatus::Completed;
                    },
                    JobAction::Pickup { item } => {
                        if locked_items.items.contains(&item) {
                            continue;
                        }

                        locked_items.items.push(item);
                        ev_item_pickup.send(WorkerPickUpItemEvent {
                            item,
                            worker: worker_entity,
                            tile_pos,
                            container: None
                        });
                    },
                    JobAction::ContainerPickup { container, pickup_amount } => {
                        if let Ok(item_container) = q_item_containers.get_mut(container) {
                            // TODO: Grab any available item or configurable?
                            if let Some(Some(item)) = item_container.items.iter()
                            .filter(|i| {
                                if let Some(i) = i {
                                    !locked_items.items.contains(i)
                                } else {
                                    false
                                }
                            })
                            .last() {
                                println!("Picking up item {:?}", item);
                                locked_items.items.push(*item);
                                ev_item_pickup.send(WorkerPickUpItemEvent {
                                    item: *item,
                                    worker: worker_entity,
                                    tile_pos,
                                    container: Some(container)
                                });
                            }
                        } else if let Ok(assembly_container) = q_assembly_containers.get_mut(container) {
                            if let Some(Some(item)) = assembly_container.output.items.iter()
                            .filter(|i| {
                                if let Some(i) = i {
                                    !locked_items.items.contains(i)
                                } else {
                                    false
                                }
                            })
                            .last() {
                                locked_items.items.push(*item);
                                ev_item_pickup.send(WorkerPickUpItemEvent {
                                    item: *item,
                                    worker: worker_entity,
                                    tile_pos,
                                    container: Some(container)
                                });
                            }
                        }
                    }
                    JobAction::Drop { worker: container, input_container } => {
                        if let Ok(item_container) = q_item_containers.get(container) {
                            if let Some(Some(item)) = item_container.items.last() {
                                ev_item_drop.send(WorkerDropItemEvent {
                                    item: *item,
                                    worker: worker_entity,
                                    container: input_container
                                });
                            } else {
                                current_job.job_status = JobStatus::Completed;
                            }
                        }
                    }
                };
            }
        }

        job.lock = false;
    }
}

pub fn worker_path_to_next_job(
    mut q_workers: Query<(&Job, &WorkerState, Entity, &Transform, &mut MoveToTile), With<Worker>>,
    q_tilemap: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform)>,
) {
    let (map_size, grid_size, map_type, map_transform) = q_tilemap.single();
    for (job, job_status, worker_entity, transform, mut movement) in q_workers.iter_mut() {
        if *job_status == WorkerState::Paused {
            movement.path = None;
            movement.path_i = 0;
            movement.target = None;
            continue;
        }
        let active_jobs: Vec<&JobPoint> = job.path.iter().filter(|path| -> bool {
            return path.job_status != JobStatus::Completed;
        }).collect();
        if let Some(job_point) = active_jobs.first() {
            let worker_pos = Vec2 {
                x: transform.translation.x,
                y: transform.translation.y,
            };
            let worker_world_pos = get_world_pos(worker_pos, map_transform);
            let worker_tile_pos = TilePos::from_world_pos(&worker_world_pos, map_size, grid_size, map_type);
            if let Some(worker_tile_pos) = worker_tile_pos {
                if is_near_tile(worker_tile_pos, job_point.point, map_size) {
                    movement.target = None;
                    movement.path_i = 0;
                    movement.path = None;
                    continue;
                }
            }
            if let Some(path) = &movement.path {
                if let Some(path_target) = path.last() {
                    if movement.target == Some(job_point.point) && is_near_tile(job_point.point, *path_target, map_size) { continue; }
                }
            }
            movement.target = Some(job_point.point);
            movement.path_i = 0;
            movement.path = None;
        }
    }
}
