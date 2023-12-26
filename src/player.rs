use std::f32::consts::{SQRT_2, E};

use bevy::input::mouse::MouseWheel;

use crate::*;

#[derive(Component)]
pub struct Player;

const MOVE_ANIMATION_FRAMES: usize = 6;

#[derive(Component, Clone, Debug)]
pub struct SpriteDirection {
    pub direction: usize,
    pub movement_frame: usize,
    pub animation_timer: Timer,
}
impl Default for SpriteDirection {
    fn default() -> Self {
        Self {
            animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            direction: 0,
            movement_frame: 0,
        }
    }
}
impl SpriteDirection {
    pub fn set_from_vec(&mut self, vec: Vec2) {
        let mut direction = 0;
        let mut moving = false;
        if vec.x > 0.0 {
            direction = 0;
            moving = true;
        } else if vec.x < 0.0 {
            direction = 2;
            moving = true;
        } else if vec.y > 0.0 {
            direction = 1;
            moving = true;
        } else if vec.y < 0.0 {
            direction = 3;
            moving = true;
        } else {
            if self.direction > 3 {
                direction = self.direction - 24;
                if direction >= 24 {
                    direction -= 24;
                }
                direction /= MOVE_ANIMATION_FRAMES;
            }
        }
        direction *= MOVE_ANIMATION_FRAMES;
        direction += self.movement_frame;
        direction += 24;
        if moving {
            direction += 24;
        }
        self.direction = direction;
    }
}

pub fn sprite_direction_system(
    mut query: Query<(&mut SpriteDirection, &mut TextureAtlasSprite)>,
) {
    for (mut direction, mut sprite) in query.iter_mut() {
        sprite.index = direction.direction;
    }
}

pub fn movement_animation_system(
    mut query: Query<&mut SpriteDirection>,
    time: Res<Time>
) {
    for mut direction in query.iter_mut() {
        if direction.animation_timer.tick(time.delta()).just_finished() {
            direction.movement_frame = (direction.movement_frame + 1) % MOVE_ANIMATION_FRAMES;
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub sprite_sheet: SpriteSheetBundle,
    pub movement: Movement,
    pub direction: SpriteDirection,
    pub camera_follow: CameraFollow,
    pub container: ItemContainer,
    pub production: PlayerPowerProduction
}

pub fn player_movement(
    mut query: Query<&mut Movement, With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    let mut movement = query.single_mut();
    let input = Vec2 {
        x: if keys.pressed(KeyCode::A) { -1.0 } else if keys.pressed(KeyCode::D) { 1.0 } else { 0.0 },
        y: if keys.pressed(KeyCode::S) { -1.0 } else if keys.pressed(KeyCode::W) { 1.0 } else { 0.0 }
    };
    movement.input = Some(input);
}

pub fn move_entities (
    mut q_movement: Query<(&Movement, &mut Transform, Option<&mut SpriteDirection>)>,
) {
    for (Movement { mut input, speed_x, speed_y }, mut transform, sprite_direction) in q_movement.iter_mut() {
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

            if let Some(mut direction) = sprite_direction {
                direction.set_from_vec(input_vec);
            }
        } else {
            if let Some(mut direction) = sprite_direction {
                direction.set_from_vec(Vec2::ZERO);
            }
        }
    }
}
const PLAYER_REACH: f32 = 4.0 * TILE_SIZE.x;

// TODO: Refactor with one-shot systems once you can use 0.12
// TODO: Resource & Trait for closest interactable
pub fn player_pickup_item(
    mut commands: Commands,
    mut q_items: Query<(Entity, &GlobalTransform, &mut Transform, &Item), (With<Item>, Without<Player>, Without<ItemIOContainer>, Without<ItemContainer>)>,
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

        if near_container.is_some() && !player_container.items.is_empty() {
            println!("Dropping item in container");
            if let Some((container, _, container_entity)) = near_container.as_mut() {
                let Some(Some(child)) = children.map(|c| c.first()) else { return; };
                let Ok((_, _, _, item_type)) = q_items.get(*child) else { return; };
                if let Ok(_) = container.add_item((Some(*child), Some(*item_type))) {
                    match player_container.remove_item(Some(*child)) {
                        Ok(_) => {
                            let Ok((item, _, mut item_transform, item_type)) = q_items.get_mut(*child) else {
                                if let Err(err) = container.remove_item(Some(*child)) {
                                    println!("Error picking item back up: {err}");
                                }
                                return;
                            };
                            commands.entity(player).remove_children(&[*child]);
                            commands.entity(*container_entity).push_children(&[*child]);
                            *item_transform = container.get_transform();
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
        } else if near_io_container.is_some() && !player_container.items.is_empty() {
            println!("Dropping item in IO container");
            if let Some((mut container, _, container_entity)) = near_io_container {
                let Some(Some(child)) = children.map(|c| c.first()) else { return; };
                let Ok((_, _, _, item_type)) = q_items.get(*child) else { return; };
                if let Ok(_) = container.input.add_item((Some(*child), Some(*item_type))) {
                    match player_container.remove_item(Some(*child)) {
                        Ok(_) => {
                            let Ok((item, _, mut item_transform, item_type)) = q_items.get_mut(*child) else {
                                if let Err(err) = container.input.remove_item(Some(*child)) {
                                    println!("Error picking item back up: {err}");
                                }
                                return;
                            };
                            commands.entity(player).remove_children(&[*child]);
                            commands.entity(container_entity).push_children(&[*child]);
                            *item_transform = container.input.get_transform();
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
        } else if let Some((entity, _, mut transform, item_type)) = near_item {
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
    
            if let Ok(_) = player_container.add_item((Some(entity), Some(*item_type))) {
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
    q_io_containers: Query<(Entity, &Transform, &mut ItemIOContainer), Without<Player>>,
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

#[derive(Resource, Default)]
pub struct AssemblyPowerSelection {
    pub selected: Option<Entity>
}

#[derive(Component)]
pub struct PlayerPowerProduction {
    pub max_output: Power,
    pub min_output: Power,
    pub input_count: usize,
    pub count_timer: Timer,
    pub no_input_count: usize
}

pub fn activate_power_mode_on_click(
    q_assemblies: Query<(Entity, &Transform), (With<AssemblyPower>, Without<Player>)>,
    q_player: Query<&Transform, (With<Player>, Without<AssemblyPower>)>,
    mut mouse_collision: EventReader<GenericMouseCollisionEvent<Assembly>>,
    input: Res<Input<MouseButton>>,
    player_state: Res<State<PlayerState>>,
    mut power_selection: ResMut<AssemblyPowerSelection>,
    mut next_state: ResMut<NextState<PlayerState>>
    
) {
    if player_state.get() == &PlayerState::None {
        if input.just_pressed(MouseButton::Left) {
            for ev in mouse_collision.iter() {
                if let Some((_, entity)) = ev.collision {
                    if let Ok((assembly_entity, transform)) = q_assemblies.get(entity) {
                        let Ok(player_transform) = q_player.get_single() else { return; };
                        let distance = Vec3::distance(player_transform.translation, transform.translation);
                        if distance > PLAYER_REACH {
                            continue;
                        }

                        power_selection.selected = Some(assembly_entity);
                        next_state.set(PlayerState::Power);
                        println!("Assembly {} selected", assembly_entity.index());
                    }
                }
            }
        }
    }
}

const POWER_DECREASE_MULT: f32 = 0.40;
const POWER_DECREASE_BASE: usize = 7; 
const INCREASE_CURVE_MULT: f32 = 0.07;
const NO_INPUT_MULT: f32 = 3.0;

pub fn player_power_assembly(
    input: Res<Input<KeyCode>>,
    mouse_pos: Res<MousePos>,
    mut ev_power_input: EventWriter<AssemblyPowerInput>,
    q_assemblies: Query<Entity, (With<AssemblyPower>, Without<Player>)>,
    mut q_player: Query<(Entity, &mut PlayerPowerProduction, &Transform), (With<Player>, Without<AssemblyPower>)>,
    selected_assembly: Res<AssemblyPowerSelection>,
    player_state: Res<State<PlayerState>>,
    time: Res<Time>,
) {
    let sigmoid = |x: f32, mult: f32| -> f32 {
        2.0 * ((1.0) / (1.0 + E.powf(-mult * x))) - 1.0
    };
    if player_state.get() == &PlayerState::Power {
        let Ok((player, mut power_prod, player_transform)) = q_player.get_single_mut() else { return };
        
        if power_prod.count_timer.tick(time.delta()).just_finished() {
            if power_prod.input_count > 0 {
                let mut decrease = (POWER_DECREASE_BASE + (power_prod.input_count as f32 * (POWER_DECREASE_MULT + (sigmoid(power_prod.no_input_count as f32, 1.0) * NO_INPUT_MULT))) as usize) / 10;
                decrease = decrease.min(power_prod.input_count);
                power_prod.input_count = power_prod.input_count - decrease;
            }
            power_prod.no_input_count += 1;
        }

        let Some(selected_assembly) = selected_assembly.selected else { return; };
        if input.just_pressed(KeyCode::Space) {
            power_prod.input_count += 10;
            power_prod.no_input_count = 0;
        }

        let output_mult = sigmoid(power_prod.input_count as f32 / 10.0, INCREASE_CURVE_MULT);
        let power_output = ((power_prod.max_output - power_prod.min_output) * output_mult) + power_prod.min_output;

        if let Ok(entity) = q_assemblies.get(selected_assembly) {
            ev_power_input.send(AssemblyPowerInput {
                assembly: entity,
                power: power_output,
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

const MAX_CAMERA_ZOOM: f32 = 1.4;
const MIN_CAMERA_ZOOM: f32 = 0.4;

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
