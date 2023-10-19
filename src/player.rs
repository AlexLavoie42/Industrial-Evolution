use std::f32::consts::SQRT_2;

use pathfinding::num_traits::{Float, FloatConst};

use crate::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub camera_follow: CameraFollow
}

pub fn player_movement(mut query: Query<&mut Movement, With<Player>>, keys: Res<Input<KeyCode>>) {
    let mut movement = query.single_mut();
    let input = Vec2 {
        x: if keys.pressed(KeyCode::A) { -1.0 } else if keys.pressed(KeyCode::D) { 1.0 } else { 0.0 },
        y: if keys.pressed(KeyCode::S) { -1.0 } else if keys.pressed(KeyCode::W) { 1.0 } else { 0.0 }
    };
    movement.input = Some(input);
}

pub fn move_entities (
    mut q_movement: Query<(&Movement, &mut Transform)>,
) {
    for (Movement { input, speed_x, speed_y }, mut transform) in q_movement.iter_mut() {
        if let Some(input_vec) = input {
            let Vec2 { x, y } = input_vec.normalize_or_zero();
            let mut movement: Vec3 = Vec3 {
                x: x * speed_x,
                y: y * speed_y,
                z: 0.0
            };
            let abs_x = x.abs();
            let abs_y = y.abs();
            if abs_x == 1.0 && abs_y == 1.0 {
                let dist = abs_x / SQRT_2;
                movement.x = dist * movement.x.signum();
                movement.y = dist * movement.y.signum();
            }
            transform.translation += movement;
        }
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraFollow {
    pub lerp: f32
}

impl CameraFollow {
    pub fn default() -> Self {
        Self { lerp: 0.1 }
    }
}

#[derive(Component)]
pub struct Movement {
    pub speed_x: f32,
    pub speed_y: f32,
    
    pub input: Option<Vec2>
}

pub fn camera_follow(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<CameraFollow>)>,
    mut follow_query: Query<&Transform, (With<CameraFollow>, Without<MainCamera>)>
) {
    let mut cam_transform: Mut<'_, Transform> = camera_query.single_mut();
    let player_transform: &Transform = follow_query.single_mut();

    cam_transform.translation = cam_transform.translation.lerp(player_transform.translation, 0.1);
}

