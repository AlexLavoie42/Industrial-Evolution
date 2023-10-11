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
