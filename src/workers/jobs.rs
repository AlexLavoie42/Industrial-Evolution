use crate::*;
use workers::worker::*;

#[derive(Debug, Reflect, Clone, Copy)]
pub enum JobAction {
    Work {
        power: Power,
        assembly: Entity
    },
    Pickup {
        item: Entity,
    },
    Drop {
        item: Entity,
        container: Option<Entity>
    },
    Idle(f32)
}

#[derive(Debug, Reflect, Clone, Copy)]
pub struct JobPoint {
    pub point: TilePos,
    pub job_status: JobStatus,
    pub action: JobAction
}

#[derive(Component, Debug, Reflect)]
pub struct Job {
    pub path: Vec<JobPoint>,
    pub complexity: f32
}

#[derive(Component, Debug, PartialEq, Reflect, Clone, Copy)]
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
    mouse_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MouseTile>,
    selected_worker: Res<SelectedWorker>,
    mut q_worker: Query<&mut Job, With<Worker>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let click_ev = mouse_collision.iter().next().clone();
        if let Some(worker_entity) = selected_worker.selected {
            if let Ok(mut job) = q_worker.get_mut(worker_entity) {
                println!("ev: {:?}", click_ev);
                if let Some(ev) = click_ev {
                    if let Some((_, entity)) = ev.collision {
                        let power: Power = Power::Mechanical(100.0);
                        let action: JobAction = JobAction::Work {
                            power,
                            assembly: entity,
                        };
                        let job_point = JobPoint {
                            point: mouse_pos.0,
                            job_status: JobStatus::Active,
                            action,
                        };
                        job.path.push(job_point);
                    }
                } else {
                    let job_point = JobPoint {
                        point: mouse_pos.0,
                        job_status: JobStatus::Active,
                        action: JobAction::Idle(0.0),
                    };
                    job.path.push(job_point);
                }
                println!("Job: {:?}", job);
            }
        }
    }
}

pub fn worker_iterate_job(
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
            job.path = job.path.iter_mut().map(| job_point| -> JobPoint {
                job_point.job_status = JobStatus::Active;
                *job_point
            }).collect();
        }
    }
}

pub fn worker_do_job(
    mut q_jobs: Query<(&mut Job, Entity, &Transform), With<Worker>>,
    q_tilemap: Query<(&Transform, &TilemapSize, &TilemapGridSize, &TilemapType)>,
    mut ev_assembly_power: EventWriter<AssemblyPowerInput>,
    mut ev_item_pickup: EventWriter<WorkerPickUpItemEvent>,
    mut ev_item_drop: EventWriter<WorkerDropItemEvent>
) {
    let (map_transform, map_size, grid_size, map_type) = q_tilemap.single();
    for (mut job, worker_entity, transform) in q_jobs.iter_mut() {
        let mut active_jobs: Vec<&mut JobPoint> = job.path.iter_mut().filter(|path| -> bool {
            return path.job_status == JobStatus::Active;
        }).collect();
        if active_jobs.len() == 0 {
            continue;
        }
        let current_job = &mut active_jobs[0];
        let world_pos = get_world_pos(Vec2 { x: transform.translation.x, y: transform.translation.y }, map_transform);
        let tile_pos = TilePos::from_world_pos(&world_pos, map_size, grid_size, map_type);
        if let Some(tile_pos) = tile_pos {
            if tile_pos == current_job.point {
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
                    JobAction::Idle(_) => {
                        current_job.job_status = JobStatus::Completed;
                    },
                    JobAction::Pickup { item } => {
                        ev_item_pickup.send(WorkerPickUpItemEvent {
                            item,
                            worker: worker_entity
                        });
                        current_job.job_status = JobStatus::Completed;
                    },
                    JobAction::Drop { item, container } => {
                        ev_item_drop.send(WorkerDropItemEvent {
                            item,
                            worker: worker_entity,
                            container: container
                        });
                        current_job.job_status = JobStatus::Completed;
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