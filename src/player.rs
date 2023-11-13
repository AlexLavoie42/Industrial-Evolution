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
    pub camera_follow: CameraFollow,
    pub container: ItemContainer
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
    for (Movement { mut input, speed_x, speed_y }, mut transform) in q_movement.iter_mut() {
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
const PLAYER_REACH: f32 = 1.0 * TILE_SIZE.x;

// TODO: Check for item container
pub fn player_pickup_item(
    mut commands: Commands,
    mut q_items: Query<(Entity, &GlobalTransform, &mut Transform), (With<Item>, Without<Player>)>,
    mut q_player: Query<(Entity, &Transform, &mut ItemContainer), (With<Player>, Without<Item>)>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F) {
        let Ok((player, player_transform, mut container)) = q_player.get_single_mut() else { return };
        for (entity, gtransform, mut transform) in q_items.iter_mut() {
            let distance = Vec3::distance(gtransform.translation(), player_transform.translation);
            if distance > PLAYER_REACH {
                continue;
            }

            if let Ok(_) = container.add_item(Some(entity)) {
                transform.translation.x = 16.0;
                transform.translation.y = 8.0;
                commands.entity(player).push_children(&[entity]);
                return;
            }
        }
    }
}

// TODO: Same key as pickup & do one based on closest to cursor & currently picked up or not
pub fn player_drop_item(
    mut commands: Commands,
    mut q_player: Query<(Entity, &Transform, &mut ItemContainer, &Children), With<Player>>,
    input: Res<Input<KeyCode>>,
    mut q_containers: Query<(Entity, &Transform, &mut ItemContainer), Without<Player>>,
    mut q_io_containers: Query<(Entity, &Transform, &mut ItemIOContainer), Without<Player>>,
    mut item_transforms: Query<&mut Transform, (With<Item>, Without<Player>, Without<ItemContainer>, Without<ItemIOContainer>)>,
) {
    if input.just_pressed(KeyCode::G) {
        let Ok((player, player_transform, mut player_container, children)) = q_player.get_single_mut() else { return };
        if player_container.items.is_empty() {
            return;
        }

        for (container_entity, container_transform, mut container) in q_containers.iter_mut() {
            let distance = Vec3::distance(container_transform.translation, player_transform.translation);
            if distance > PLAYER_REACH {
                continue;
            }

            for child in children.iter() {
                if let Ok(_) = container.add_item(Some(*child)) {
                    match player_container.remove_item(Some(*child)) {
                        Ok(_) => {
                            commands.entity(player).remove_children(&[*child]);
                            commands.entity(container_entity).push_children(&[*child]);
                            if let Ok(mut transform) = item_transforms.get_mut(*child) {
                                transform.translation = Vec3::new(0.0, 0.0, transform.translation.z);
                            }
                            return;
                        },
                        Err(_) => {
                            if let Err(err) = container.remove_item(Some(*child)) {
                                println!("Error picking item back up: {err}");
                            }
                        }
                    }
                }
            }
        }

        for (container_entity, container_transform, mut container) in q_io_containers.iter_mut() {
            let distance = Vec3::distance(container_transform.translation, player_transform.translation);
            if distance > PLAYER_REACH {
                continue;
            }

            for child in children.iter() {
                if let Ok(_) = container.input.add_item(Some(*child)) {
                    match player_container.remove_item(Some(*child)) {
                        Ok(_) => {
                            commands.entity(player).remove_children(&[*child]);
                            commands.entity(container_entity).push_children(&[*child]);
                            if let Ok(mut transform) = item_transforms.get_mut(*child) {
                                transform.translation = Vec3::new(0.0, 0.0, transform.translation.z);
                            }
                            return;
                        },
                        Err(_) => {
                            if let Err(err) = container.input.remove_item(Some(*child)) {
                                println!("Error removing item: {err}");
                            }
                        }
                    }
                }
            }
        }

        for child in children.iter() {
            match player_container.remove_item(Some(*child)) {
                Ok(_) => {
                    commands.entity(player).remove_children(&[*child]);
                    if let Ok(mut transform) = item_transforms.get_mut(*child) {
                        transform.translation = Vec3::new(player_transform.translation.x, player_transform.translation.y, transform.translation.z);
                    }
                    return;
                },
                Err(err) => {
                    println!("Error dropping item: {err}");
                }
            }
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

