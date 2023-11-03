use std::time::Duration;

use crate::*;
use workers::worker::*;

#[derive(Debug, Reflect, PartialEq)]
pub enum JobAction {
    Work {
        power: Power,
        assembly: Entity
    },
    Pickup {
        item: Entity,
    },
    ContainerPickup {
        container: Entity
    },
    Drop {
        worker: Entity,
        input_container: Option<Entity>
    },
    Idle
}

#[derive(Debug, Reflect, PartialEq)]
pub struct JobPoint {
    pub point: TilePos,
    pub job_status: JobStatus,
    pub action: JobAction,
    pub timer: Option<Timer>
}

#[derive(Component, Debug, Reflect)]
pub struct Job {
    pub path: Vec<JobPoint>,
    pub complexity: f32,
    pub current_job: Option<usize>
}

#[derive(Component, Debug, PartialEq, Reflect)]
pub enum JobStatus {
    Active,
    Completed,
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
    if player_state.get() == &PlayerState::None {
        if input.just_pressed(MouseButton::Left) {
            for ev in mouse_collision.iter() {
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
    q_assemblies: Query<Entity, With<Assembly>>,
    q_transforms: Query<&Transform>,
    q_assembly_input: Query<(&AssemblyInput, &Parent)>,
    q_assembly_output: Query<(&AssemblyOutput, &Parent)>,
    q_items: Query<Entity, With<Item>>,
    mouse_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MouseTile>,
    selected_worker: Res<SelectedWorker>,
    mut q_worker: Query<&mut Job, With<Worker>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let click_evs: Vec<&MouseCollisionEvent> = mouse_collision.iter().collect();
        if let Some(worker_entity) = selected_worker.selected {
            if let Ok(mut job) = q_worker.get_mut(worker_entity) {
                if let Some(ev) = click_evs.first() {
                    if let Some((_, entity)) = ev.collision {
                        if let Ok(assembly) = q_assemblies.get(entity) {
                            let power: Power = Power::Mechanical(100.0);
                            let action: JobAction = JobAction::Work {
                                power,
                                assembly,
                            };
                            let job_point = JobPoint {
                                point: mouse_pos.0,
                                job_status: JobStatus::Active,
                                action,
                                timer: None
                            };
                            job.path.push(job_point);
                        }

                        if let Ok((assembly_input, parent)) = q_assembly_input.get(entity) {
                            let job_point = JobPoint {
                                point: mouse_pos.0,
                                job_status: JobStatus::Active,
                                action: JobAction::Drop {
                                    input_container: Some(parent.get()),
                                    worker: worker_entity
                                },
                                timer: None
                            };
                            job.path.push(job_point);
                        }

                        if let Ok((assembly_output, parent)) = q_assembly_output.get(entity) {
                            let job_point = JobPoint {
                                point: mouse_pos.0,
                                job_status: JobStatus::Active,
                                action: JobAction::ContainerPickup {
                                    container: parent.get()
                                },
                                timer: None
                            };
                            job.path.push(job_point);
                        }

                        if let Ok(item) = q_items.get(entity) {
                            let action: JobAction = JobAction::Pickup {
                                item
                            };
                            let job_point = JobPoint {
                                point: mouse_pos.0,
                                job_status: JobStatus::Active,
                                action,
                                timer: None
                            };
                            job.path.push(job_point);
                        }
                    }
                } else {
                    let job_point = JobPoint {
                        point: mouse_pos.0,
                        job_status: JobStatus::Active,
                        action: JobAction::Idle,
                        timer: Some(Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once))
                    };
                    job.path.push(job_point);
                }
            }
        }
    }
}

pub fn worker_iterate_jobs(
    mut q_jobs: Query<(&mut Job, &WorkerState)>,
) {
    for (mut job,  state) in q_jobs.iter_mut() {
        if state == &WorkerState::Paused {
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

pub fn worker_do_job(
    time: Res<Time>,
    mut q_jobs: Query<(&mut Job, Entity, &Transform), With<Worker>>,
    q_tilemap: Query<(&Transform, &TilemapSize, &TilemapGridSize, &TilemapType)>,
    mut q_item_containers: Query<&mut ItemContainer>,
    mut ev_assembly_power: EventWriter<AssemblyPowerInput>,
    mut ev_item_pickup: EventWriter<WorkerPickUpItemEvent>,
    mut ev_item_drop: EventWriter<WorkerDropItemEvent>
) {
    let (map_transform, map_size, grid_size, map_type) = q_tilemap.single();
    for (mut job, worker_entity, transform) in q_jobs.iter_mut() {
        let world_pos = get_world_pos(Vec2 { x: transform.translation.x, y: transform.translation.y }, map_transform);
        let tile_pos = TilePos::from_world_pos(&world_pos, map_size, grid_size, map_type);
        if let (Some(tile_pos), Some(current_job_i)) = (tile_pos, job.current_job) {
            let current_job = &mut job.path[current_job_i];
            if tile_pos == current_job.point {
                if let Some(timer) = &mut current_job.timer {
                    timer.tick(time.delta());
                    if !timer.finished() {
                        continue;
                    }
                    timer.reset();
                }
                match current_job.action {
                    JobAction::Work { power, assembly } => {
                        // TODO: Timer
                        ev_assembly_power.send(AssemblyPowerInput {
                            assembly,
                            source: worker_entity,
                            power
                        });
                        current_job.job_status = JobStatus::Completed;
                    },
                    JobAction::Idle => {
                        current_job.job_status = JobStatus::Completed;
                    },
                    JobAction::Pickup { item } => {
                        ev_item_pickup.send(WorkerPickUpItemEvent {
                            item,
                            worker: worker_entity
                        });
                        current_job.job_status = JobStatus::Completed;
                    },
                    JobAction::ContainerPickup { container } => {
                        if let Ok(mut item_container) = q_item_containers.get_mut(container) {
                            if let Some(Some(item)) = item_container.items.last() {
                                ev_item_pickup.send(WorkerPickUpItemEvent {
                                    item: *item,
                                    worker: worker_entity
                                });
                                current_job.job_status = JobStatus::Completed;
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
                                current_job.job_status = JobStatus::Completed;
                            }
                        }
                        // TODO: Worker error state?
                    }
                };
            }
        }
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
            return path.job_status == JobStatus::Active;
        }).collect();
        if let Some(job_point) = active_jobs.first() {
            let worker_pos = Vec2 {
                x: transform.translation.x,
                y: transform.translation.y,
            };
            let worker_world_pos = get_world_pos(worker_pos, map_transform);
            if let Some(tile_pos) = TilePos::from_world_pos(&worker_world_pos, map_size, grid_size, map_type) {
                if job_point.point != tile_pos {
                    movement.target = Some(job_point.point);
                } else {
                    movement.target = None;
                    movement.path = None;
                    movement.path_i = 0;
                }
            }
        }
    }
}