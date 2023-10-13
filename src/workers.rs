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
                    (job_mode_creation).run_if(in_state(PlayerState::Jobs))
                )
            )
            .insert_resource(WorkerJobSelection {
                selected: None
            });
    }
}

#[derive(Component)]
pub struct Worker {
    pub job: Job
}

pub enum JobAction {
    Work(Power),
    Idle(f32)
}

pub struct JobPoint {
    pub point: Vec2,
    pub action: JobAction
}

#[derive(Component)]
pub struct Job {
    pub path: Vec<JobPoint>,
    pub complexity: f32
}

#[derive(Bundle)]
pub struct WorkerBundle {
    pub marker: Worker,
    pub sprite: SpriteBundle,
    pub movement: Movement
}
impl Default for WorkerBundle {
    fn default() -> WorkerBundle {
        WorkerBundle {
            marker: Worker {
                job: Job {
                    path: vec![],
                    complexity: 0.0
                }
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
pub struct WorkerJobSelection {
    pub selected: Option<Entity>
}

pub fn activate_job_mode_on_click(
    q_worker: Query<(Entity, &Transform, &Sprite), With<Worker>>,
    mouse_pos: Res<MousePos>,
    mouse_input: Res<Input<MouseButton>>,
    player_state: Res<State<PlayerState>>,
    mut worker_selection: ResMut<WorkerJobSelection>,
    mut next_state: ResMut<NextState<PlayerState>>
    
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (worker, transform, sprite) in q_worker.iter() {
            let mouse_vec = Vec3 {
                x: mouse_pos.0.x,
                y: mouse_pos.0.y,
                z: 0.0
            };
            // TODO: Proper size / proper colliders / tilemap collision?
            let mouse_collision = collide_aabb::collide(
                transform.translation,
                sprite.custom_size.unwrap(),
                mouse_vec, 
                Vec2{ x: 1.0, y: 1.0 }
            );
            if mouse_collision.is_some() && player_state.get() == &PlayerState::None {
                worker_selection.selected = Some(worker);
                next_state.set(PlayerState::Jobs);
            }
        }
    }
}

pub fn job_mode_creation(

) {

}
