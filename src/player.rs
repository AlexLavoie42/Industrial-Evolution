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

pub fn player_movement(mut query: Query<(&mut Transform, &Movement), With<Player>>, keys: Res<Input<KeyCode>>) {
    let (mut transform, movement) = query.single_mut();
    let input: Vec3 = Vec3 {
        x: if keys.pressed(KeyCode::A) { -1.0 } else if keys.pressed(KeyCode::D) { 1.0 } else { 0.0 },
        y: if keys.pressed(KeyCode::S) { -1.0 } else if keys.pressed(KeyCode::W) { 1.0 } else { 0.0 },
        z: transform.translation.z
    };
    let mut movement: Vec3 = Vec3 {
        x: input.x * movement.speed_x,
        y: input.y * movement.speed_y,
        z: transform.translation.z
    };
    let abs_x = input.x.abs();
    let abs_y = input.y.abs();
    if abs_x == 1.0 && abs_y == 1.0 {
        movement.x = if movement.x > 0.0 { movement.x.abs().sqrt() } else { -movement.x.abs().sqrt() };
        movement.y = if movement.y > 0.0 { movement.y.abs().sqrt() } else { -movement.y.abs().sqrt() };
    }
    transform.translation += movement;
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
    pub speed_y: f32
}

pub fn camera_follow(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<CameraFollow>)>,
    mut follow_query: Query<&Transform, (With<CameraFollow>, Without<MainCamera>)>
) {
    let mut cam_transform: Mut<'_, Transform> = camera_query.single_mut();
    let player_transform: &Transform = follow_query.single_mut();

    cam_transform.translation = cam_transform.translation.lerp(player_transform.translation, 0.1);
}

