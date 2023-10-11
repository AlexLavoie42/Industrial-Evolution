use crate::*;

#[derive(Component)]
pub struct Worker;

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
            marker: Worker,
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

            commands.spawn(WorkerBundle {
                sprite: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(pos_x, pos_y, -1.0),
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
