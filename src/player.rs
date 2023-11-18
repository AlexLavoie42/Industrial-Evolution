use std::f32::{consts::SQRT_2, INFINITY};

use bevy::input::mouse::{MouseWheel, MouseButtonInput};
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
    pub container: ItemContainer,
    pub production: PowerProduction
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
const PLAYER_REACH: f32 = 4.0 * TILE_SIZE.x;

// TODO: Refactor with one-shot systems once you can use 0.12
// TODO: Resource & Trait for closest interactable
pub fn player_pickup_item(
    mut commands: Commands,
    mut q_items: Query<(Entity, &GlobalTransform, &mut Transform), (With<Item>, Without<Player>, Without<ItemIOContainer>, Without<ItemContainer>)>,
    mut q_containers: Query<(&mut ItemContainer, &Transform, Entity), (Without<Player>, Without<ItemIOContainer>)>,
    mut q_io_containers: Query<(&mut ItemIOContainer, &Transform, Entity), (Without<Player>, Without<ItemContainer>)>,
    mut q_player: Query<(Entity, &Transform, &mut ItemContainer, Option<&Children>), (With<Player>, Without<Item>, Without<ItemIOContainer>)>,
    input: Res<Input<KeyCode>>,
    mouse_pos: Res<MousePos>,
) {
    if input.just_pressed(KeyCode::F) {
        let Ok((player, player_transform, mut player_container, children)) = q_player.get_single_mut() else { return };
        let near_item = q_items.iter_mut()
            .filter(|i| {
                let is_input = q_io_containers.iter_mut().any(|c| c.0.input.items.contains(&Some(i.0)));
                if is_input {
                    return false;
                }
                let is_held = player_container.items.contains(&Some(i.0));
                if is_held {
                    return false;
                }
                let distance = Vec3::distance(i.1.translation(), player_transform.translation);
                distance <= PLAYER_REACH
            })
            .min_by(|a, b| {
                let a_distance = Vec3::distance(a.1.translation(), vec3(mouse_pos.0.x, mouse_pos.0.y, 0.0));
                let b_distance: f32 = Vec3::distance(b.1.translation(), vec3(mouse_pos.0.x, mouse_pos.0.y, 0.0));
                a_distance.partial_cmp(&b_distance).unwrap()
            });
        let mut near_container = q_containers.iter_mut()
            .filter(|c| {
                let distance = Vec3::distance(c.1.translation, player_transform.translation);
                distance <= PLAYER_REACH
            })
            .min_by(|a, b| {
                let a_distance = Vec3::distance(a.1.translation, vec3(mouse_pos.0.x, mouse_pos.0.y, 0.0));
                let b_distance = Vec3::distance(b.1.translation, vec3(mouse_pos.0.x, mouse_pos.0.y, 0.0));
                a_distance.partial_cmp(&b_distance).unwrap()
            });
        let near_io_container = q_io_containers.iter_mut()
            .filter(|c| {
                let distance = Vec3::distance(c.1.translation, player_transform.translation);
                distance <= PLAYER_REACH
            })
            .min_by(|a, b| {
                let a_distance = Vec3::distance(a.1.translation, vec3(mouse_pos.0.x, mouse_pos.0.y, 0.0));
                let b_distance = Vec3::distance(b.1.translation, vec3(mouse_pos.0.x, mouse_pos.0.y, 0.0));
                a_distance.partial_cmp(&b_distance).unwrap()
            });


        let container_dist = near_container.as_ref().map(|c| Vec3::distance(c.1.translation, player_transform.translation));
        let io_container_dist = near_io_container.as_ref().map(|c| Vec3::distance(c.1.translation, player_transform.translation));
        let item_dist = near_item.as_ref().map(|i| Vec3::distance(i.1.translation(), player_transform.translation));

        if container_dist.unwrap_or(INFINITY) < item_dist.unwrap_or(INFINITY) && !player_container.items.is_empty() {
            println!("Dropping item in container");
            if let Some((container, _, container_entity)) = near_container.as_mut() {
                let Some(Some(child)) = children.map(|c| c.first()) else { return; };
                if let Ok(_) = container.add_item(Some(*child)) {
                    match player_container.remove_item(Some(*child)) {
                        Ok(_) => {
                            let Ok((item, _, mut item_transform)) = q_items.get_mut(*child) else {
                                if let Err(err) = container.remove_item(Some(*child)) {
                                    println!("Error picking item back up: {err}");
                                }
                                return;
                            };
                            commands.entity(player).remove_children(&[*child]);
                            commands.entity(*container_entity).push_children(&[*child]);
                            item_transform.translation = Vec3::new(0.0, 0.0, item_transform.translation.z);
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
        } else if io_container_dist.unwrap_or(INFINITY) < item_dist.unwrap_or(INFINITY) && !player_container.items.is_empty() {
            println!("Dropping item in IO container");
            if let Some((mut container, _, container_entity)) = near_io_container {
                let Some(Some(child)) = children.map(|c| c.first()) else { return; };
                if let Ok(_) = container.input.add_item(Some(*child)) {
                    match player_container.remove_item(Some(*child)) {
                        Ok(_) => {
                            let Ok((item, _, mut item_transform)) = q_items.get_mut(*child) else {
                                if let Err(err) = container.input.remove_item(Some(*child)) {
                                    println!("Error picking item back up: {err}");
                                }
                                return;
                            };
                            commands.entity(player).remove_children(&[*child]);
                            commands.entity(container_entity).push_children(&[*child]);
                            item_transform.translation = Vec3::new(0.0, 0.0, item_transform.translation.z);
                            return;
                        },
                        Err(_) => {
                            if let Err(err) = container.input.remove_item(Some(*child)) {
                                println!("Error picking item back up: {err}");
                            }
                        }
                    }
                }
            }
        } else if let Some((entity, _, mut transform)) = near_item {
            println!("Pick up item");
            let is_input = q_io_containers.iter_mut().any(|c| c.0.input.items.contains(&Some(entity)));
            if is_input {
                // TODO: Selectable inputs when hover select is setup?
                println!("Can't pick up input item");
                return;
            }
    
            let item_container = q_containers.iter_mut().find(|c| c.0.items.contains(&Some(entity)));
            let io_container = q_io_containers.iter_mut().find(|c| c.0.output.items.contains(&Some(entity)));
    
            if let Some((mut item_container, _, _)) = item_container {
                if let Err(err) = item_container.remove_item(Some(entity)) {
                    println!("Error removing item: {err}");
                }
            } else if let Some((mut io_container, _, _)) = io_container {
                if let Err(err) = io_container.output.remove_item(Some(entity)) {
                    println!("Error removing item: {err}");
                }
            }
    
            if let Ok(_) = player_container.add_item(Some(entity)) {
                transform.translation.x = 16.0;
                transform.translation.y = 8.0;
                commands.entity(player).push_children(&[entity]);
                return;
            }
        }
    }
}

pub fn player_drop_item(
    mut commands: Commands,
    mut q_player: Query<(Entity, &Transform, &mut ItemContainer, &Children), With<Player>>,
    input: Res<Input<KeyCode>>,
    mut q_containers: Query<(Entity, &Transform, &mut ItemContainer), Without<Player>>,
    mut q_io_containers: Query<(Entity, &Transform, &mut ItemIOContainer), Without<Player>>,
    mut item_transforms: Query<&mut Transform, (With<Item>, Without<Player>, Without<ItemContainer>, Without<ItemIOContainer>)>,
    mouse_pos: Res<MousePos>,
) {
    if input.just_pressed(KeyCode::Q) {
        let Ok((player, player_transform, mut player_container, children)) = q_player.get_single_mut() else { return };
        if player_container.items.is_empty() {
            return;
        };

        let Some(child) = children.first() else { return; };
        match player_container.remove_item(Some(*child)) {
            Ok(_) => {
                println!("dropping item {:?}", *child);
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

pub fn player_power_assembly(
    input: Res<Input<KeyCode>>,
    mouse_pos: Res<MousePos>,
    mut ev_power_input: EventWriter<AssemblyPowerInput>,
    q_assemblies: Query<(Entity, &Transform), With<AssemblyPower>>,
    q_player: Query<(Entity, &PowerProduction), With<Player>>,
) {
    if input.pressed(KeyCode::Space) {
        let Ok((player, power_prod)) = q_player.get_single() else { return };
        let closest_assembly = q_assemblies.iter()
            .min_by(|a, b| {
                let a_distance = Vec3::distance(a.1.translation, vec3(mouse_pos.0.x, mouse_pos.0.y, 0.0));
                let b_distance = Vec3::distance(b.1.translation, vec3(mouse_pos.0.x, mouse_pos.0.y, 0.0));
                a_distance.partial_cmp(&b_distance).unwrap()
            });

        if let Some((entity, _)) = closest_assembly {
            ev_power_input.send(AssemblyPowerInput {
                assembly: entity,
                power: power_prod.power,
                source: player,
            });
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

const MAX_CAMERA_ZOOM: f32 = 1.5;
const MIN_CAMERA_ZOOM: f32 = 0.25;

pub fn camera_scroll_zoom(
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut ev_scroll: EventReader<MouseWheel>,
) {
    let mut cam = camera_query.single_mut();
    for ev in ev_scroll.iter() {
        if ev.y < 0.0 && cam.scale < MAX_CAMERA_ZOOM {
            cam.scale += 0.1;
        }
        if ev.y > 0.0 && cam.scale > MIN_CAMERA_ZOOM {
            cam.scale -= 0.1;
        }
    }
}
