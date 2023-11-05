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
    if input.just_pressed(KeyCode::E) {
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

        let Some(tile_pos) = get_mouse_tile(window, camera, camera_transform, tilemap_size, grid_size, map_type, map_transform) else { return };
        let pos = get_tile_world_pos(&tile_pos, map_transform, grid_size, map_type);
        let output_entity = commands.spawn(ContainerOutputSelectorBundle {
            marker: ContainerOutputSelector,
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

        let input_entity: Entity = commands.spawn(ContainerInputSelectorBundle {
            marker: ContainerInputSelector,
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

pub fn produce_goods(
    mut commands: Commands,
    mut q_assembly: Query<(Entity, &AssemblyPower, &mut ItemIOContainer, &AssemblyInput, &AssemblyOutput)>,
    q_items: Query<&Item>
) {
    for (
        assembly_entity,
        assembly_power,
        mut assembly_items,
        assembly_input,
        assembly_output
    ) in q_assembly.iter_mut() {
        if !assembly_items.input.items.is_empty() &&
        assembly_items.output.max_items > assembly_items.output.items.len() &&
        assembly_power.0.is_some() {
            // TODO: Production timer
            let (Some(Some(mut input_entity)), Some(assembly_input)) = (assembly_items.input.items.last_mut(), &assembly_input.0) else { continue; };
            let Ok(item) = q_items.get(input_entity) else { continue; };
            if assembly_input != item {
                continue;
            }
            if let Some(assembly_output) = &assembly_output.0 {
                let mut output_entity_commands: bevy::ecs::system::EntityCommands<'_, '_, '_> = assembly_output.spawn_bundle(&mut commands);
                let output_entity = output_entity_commands.id();
                if let Ok(_) = assembly_items.output.add_item(Some(output_entity)) {
                    if let Ok(_) = assembly_items.input.remove_item(Some(input_entity)) {
                        commands.entity(assembly_entity).remove_children(&[input_entity]);
                        commands.entity(input_entity).insert(DespawnLater);
                        commands.entity(assembly_entity).push_children(&[output_entity]);
                    } else {
                        output_entity_commands.despawn();
                        if let Err(err) = assembly_items.output.remove_item(Some(output_entity)) {}
                    }
                } else {
                    output_entity_commands.despawn();
                    if let Err(err) = assembly_items.output.remove_item(Some(output_entity)) {}
                }
            } else {
                if let Ok(_) = assembly_items.input.remove_item(Some(input_entity)) {
                    commands.entity(assembly_entity).remove_children(&[input_entity]);
                    commands.entity(input_entity).insert(DespawnLater);
                }
            }
        }
    }
}
