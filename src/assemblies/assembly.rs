use crate::*;

#[derive(Component, Debug)]
pub struct Assembly;
impl Clickable for Assembly {}

#[derive(Resource)]
pub struct SelectedAssembly {
    pub selected: AssemblyType
}

#[derive(Component, Reflect, Debug)]
pub struct AssemblyPower(pub Option<Power>);

pub fn input_toggle_assembly_mode(
    input: Res<Input<KeyCode>>,
    state: Res<State<PlayerState>>,
    mut next_state: ResMut<NextState<PlayerState>>
) {
    if input.just_pressed(KeyCode::R) {
        if state.get() == &PlayerState::Assemblies {
            next_state.set(PlayerState::None);
        } else {
            next_state.set(PlayerState::Assemblies);
            
        }
    }
}

pub fn place_assembly(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    selected_assembly: Res<SelectedAssembly>,
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
            let input_entity = commands.spawn(AssemblyOutputBundle {
                marker: AssemblyOutput(None),
                sprite: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(0.0, 16.0, 1.0),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(16.0, 8.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            }).id();

            let output_entity: Entity = commands.spawn(AssemblyInputBundle {
                marker: AssemblyInput(None),
                sprite: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(0.0, -16.0 as f32, 1.0),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::new(16.0, 8.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            }).id();
            selected_assembly.selected.spawn_bundle(&mut commands, pos).push_children(&[input_entity, output_entity]);
        }
    }
}

pub fn produce_goods(
    mut commands: Commands,
    mut q_assembly: Query<(&AssemblyPower, &mut AssemblyItemContainer, &Children)>,
    mut q_assembly_input: Query<&AssemblyInput>,
    mut q_assembly_output: Query<&AssemblyOutput>,
    q_items: Query<&Item>
) {
    for (
        assembly_power,
        mut assembly_items,
        children
    ) in q_assembly.iter_mut() {
        if !assembly_items.input.items.is_empty() &&
        assembly_items.output.max_items > assembly_items.output.items.len() &&
        assembly_power.0.is_some() {
            let assembly_inputs: Vec<&AssemblyInput> = children.iter().map(|child| {
                q_assembly_input.get(*child)
            }).filter(|child| {
                child.is_ok() && child.unwrap().0.is_some()
            }).map(|child| {
                child.unwrap()
            }).collect();
            let assembly_input = assembly_inputs.first();

            let assembly_outputs: Vec<&AssemblyOutput> = children.iter().map(|child| {
                q_assembly_output.get(*child)
            }).filter(|child| {
                child.is_ok() && child.unwrap().0.is_some()
            }).map(|child| {
                child.unwrap()
            }).collect();
            let assembly_output: Option<&&AssemblyOutput> = assembly_outputs.first();
            println!("Input: {:?}", assembly_input);
            // TODO: Production timer
            if let (Some(Some(mut input_entity)), Some(assembly_input)) = (assembly_items.input.items.last_mut(), assembly_input) {
                println!("Input: {:?}", assembly_input);
                println!("input entity: {:?}", input_entity);
                if let Ok(item) = q_items.get(input_entity) {
                    println!("Item: {:?}", item);
                    if let Some(input) = &assembly_input.0 {
                        if input != item {
                            continue;
                        }
                    }
                    if let Some(assembly_output) = assembly_output {
                        if let Some(output) = &assembly_output.0 {
                            let mut output_entity: bevy::ecs::system::EntityCommands<'_, '_, '_> = output.spawn_bundle(&mut commands);
                            if let Ok(_) = assembly_items.output.add_item(Some(output_entity.id())) {
                                if let Ok(_) = assembly_items.input.remove_item(Some(input_entity)) {
                                    commands.entity(input_entity).despawn();
                                }
                            } else {
                                output_entity.despawn();
                            }
                        }
                    } else {
                        if let Ok(_) = assembly_items.input.remove_item(Some(input_entity)) {
                            commands.entity(input_entity).despawn();
                        }
                    }
                }
            }
        }
    }
}
