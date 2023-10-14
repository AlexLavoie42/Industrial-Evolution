use bevy::sprite::collide_aabb::{Collision, self};

use crate::*;

pub struct WorkerPlugin;

impl Plugin for WorkerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, 
                (
                    (place_worker).run_if(in_state(PlayerState::Workers)),
                    input_toggle_worker_mode,
                    activate_job_mode_on_click,
                    (job_mode_creation).run_if(in_state(PlayerState::Jobs)),
                    mouse_collision_system::<Worker>,
                )
            )
            .insert_resource(SelectedWorker {
                selected: None
            });
    }
}

#[derive(Component)]
pub struct PowerProduction {
    pub power: Power,
    pub output: Option<Entity>
}

#[derive(Component)]
pub struct Worker;

pub enum JobAction {
    Work {
        power: Power,
        assembly: Entity
    },
    Idle(f32)
}

pub struct JobPoint {
    pub point: Vec2,
    pub action: JobAction
}

#[derive(Component)]
pub struct Job {
    pub path: Vec<JobPoint>,
    pub active: Option<usize>,
    pub complexity: f32
}

#[derive(Bundle)]
pub struct WorkerBundle {
    pub marker: Worker,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub production: PowerProduction
}
impl Default for WorkerBundle {
    fn default() -> WorkerBundle {
        WorkerBundle {
            marker: Worker,
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
            movement: Movement { speed_x: 1.25, speed_y: 1.25 }
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
                        ..default()
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

#[derive(Resource)]
pub struct SelectedWorker {
    pub selected: Option<Entity>
}

pub fn activate_job_mode_on_click(
    q_worker: Query<Entity, With<Worker>>,
    mut mouse_collision: EventReader<MouseCollisionEvent>,
    player_state: Res<State<PlayerState>>,
    mut worker_selection: ResMut<SelectedWorker>,
    mut next_state: ResMut<NextState<PlayerState>>
    
) {
    if player_state.get() == &PlayerState::None {
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

pub fn job_mode_creation(
    mut mouse_collision: EventReader<MouseCollisionEvent>,
    mouse_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePos>,
    selected_worker: Res<SelectedWorker>,
    mut q_worker: Query<&mut Job, With<Worker>>,
    q_assembly: Query<(Entity, &Transform, &Sprite, &Assembly)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(worker_entity) = selected_worker.selected {
            for event in mouse_collision.iter() {
                if let Some((_, entity)) = event.collision {
                    if let Ok(_) = q_assembly.get(entity) {
                        let power = Power::Mechanical(100.0);
                        let action = JobAction::Work {
                            power,
                            assembly: entity,
                        };
                        let job_point = JobPoint {
                            point: mouse_pos.0,
                            action,
                        };
                        if let Ok(mut job) = q_worker.get_mut(worker_entity) {
                            job.path.push(job_point);
                        }
                    }
                } else {
                    let job_point = JobPoint {
                        point: mouse_pos.0,
                        action: JobAction::Idle(0.0),
                    };
                    if let Ok(mut job) = q_worker.get_mut(worker_entity) {
                        job.path.push(job_point);
                    }
                }
            }
        }
    }
}

pub fn worker_power_assembler(
    q_jobs: Query<(&Job, Entity, &Transform), With<Worker>>,
    q_assemblies: Query<(&Assembly, Entity, &Transform), Without<Worker>>,
    mut ev_assembly_power: EventWriter<AssemblyPowerInput>
) {
    for (job, worker_entity, _) in q_jobs.iter() {
        if let Some(current_job_i) = job.active {
            let current_job = &job.path[current_job_i];
            match current_job.action {
                // TODO: Pathfinding
                JobAction::Work { power, assembly } => {
                    ev_assembly_power.send(AssemblyPowerInput {
                        assembly,
                        source: worker_entity,
                        power
                    });
                },
                JobAction::Idle(_) => {}
            };
        }
    }
}
