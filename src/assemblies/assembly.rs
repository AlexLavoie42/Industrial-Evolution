use crate::*;

#[derive(Component, Debug)]
pub struct Assembly;
impl Clickable for Assembly {}

#[derive(Component)]
pub struct AssemblyItemContainer {
    pub input: ItemContainer,
    pub output: ItemContainer,
}

#[derive(Component)]
pub struct AssemblyInput(pub Option<Item>);
#[derive(Component)]
pub struct AssemblyOutput(pub Option<Good>);
#[derive(Component)]
pub struct AssemblyPower(pub Option<Power>);

#[derive(Bundle)]
pub struct AssemblyBundle {
    pub marker: Assembly,
    pub solid: SolidEntity,
    pub assembly_items: AssemblyItemContainer,
    pub sprite: SpriteBundle
}
impl Default for AssemblyBundle {
    fn default() -> AssemblyBundle {
        AssemblyBundle {
            marker: Assembly,
            solid: SolidEntity,
            assembly_items: AssemblyItemContainer {
                input: ItemContainer {
                    items: Vec::new(),
                    max_items: 5
                },
                output: ItemContainer {
                    items: Vec::new(),
                    max_items: 3
                }
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            }
        }
    }
}

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
            commands.spawn(AssemblyBundle {
                sprite: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(pos.x, pos.y, -1.0),
                        ..AssemblyBundle::default().sprite.transform
                    },
                    sprite: Sprite {
                        color: Color::YELLOW,
                        ..AssemblyBundle::default().sprite.sprite
                    },
                    ..default()
                },
                ..default()
            });
        }
    }
}

pub fn produce_goods(
    mut commands: Commands,
    mut q_assembly: Query<(&AssemblyPower, &AssemblyInput, &mut AssemblyOutput, &mut AssemblyItemContainer)>,
    q_items: Query<&Item>,

) {
    for (
        assembly_power,
        assembly_input,
        assembly_output,
        mut assembly_items
    ) in q_assembly.iter_mut() {
        if !assembly_items.input.items.is_empty() &&
        assembly_items.output.max_items < assembly_items.output.items.len() &&
        assembly_power.0.is_some() {
            // TODO: Check requirements function
            // TODO: Production timer
            if let (Some(Some(entity)), Some(assembly_input)) = (assembly_items.output.items.pop(), &assembly_input.0) {
                if let Ok(item) = q_items.get(entity) {
                    if assembly_input != item {
                        return;
                    }
                    if let Some(output) = &assembly_output.0 {
                        let mut output_entity = output.spawn_bundle(&mut commands);
                        if let Ok(()) = assembly_items.output.add_item(Some(output_entity.id())) {
                            commands.entity(entity).despawn();
                        } else {
                            output_entity.despawn();
                        }
                    } else {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}
