use crate::*;

#[derive(Component, Debug)]
pub struct Assembly;
impl Clickable for Assembly {}

#[derive(Resource, Reflect)]
pub struct SelectedAssembly {
    pub selected: AssemblyType
}
impl Default for SelectedAssembly {
    fn default() -> Self {
        SelectedAssembly {
            selected: AssemblyType::PulpMill
        }
    }
}

#[derive(Component, Reflect, Debug)]
pub struct AssemblyPower {
    pub current_power: Power,
    pub max_power: f32,
    pub powering_entities: Vec<Entity>,
    pub power_cost: f32
}

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

pub fn refund_assembly(
    mut commands: Commands,
    mut ev_assembly_mouse: EventReader<GenericMouseCollisionEvent<Assembly>>,
    mut q_assembly: Query<(Entity, &AssemblyType)>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<PlayerMoney>,
    assembly_prices: Res<AssemblyPrices>
) {
    if input.just_pressed(KeyCode::Delete) && ev_assembly_mouse.iter().last().is_some() {
        for (assembly, assembly_type) in q_assembly.iter_mut() {
            if let Some(price) = assembly_prices.prices.get(assembly_type) {
                money.add_money(*price);
            }
            commands.entity(assembly).despawn_recursive();
        }
    }
}

pub fn place_assembly(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    selected_assembly: Res<SelectedAssembly>,
    assembly_prices: Res<AssemblyPrices>,
    mut money: ResMut<PlayerMoney>,
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
        let price = assembly_prices.prices.get(&selected_assembly.selected);
        if let Some(price) = price { 
            let Ok (_) = money.try_remove_money(*price) else { 
                println!("Not enough money to place assembly"); 
                return
            };
        }

        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();
        let (tilemap_size, grid_size, map_type, map_transform) = tilemap_q.single();

        let Some(tile_pos) = get_mouse_tile(window, camera, camera_transform, tilemap_size, grid_size, map_type, map_transform) else { return };
        let pos = get_tile_world_pos(&tile_pos, map_transform, grid_size, map_type);

        let mut output_bundle = ContainerOutputSelectorBundle::default();
        output_bundle.sprite.transform.translation = Vec3::new(0.0, 16.0, 1.0);
        let output_entity = commands.spawn(output_bundle).id();

        let mut input_bundle = ContainerInputSelectorBundle::default();
        input_bundle.sprite.transform.translation = Vec3::new(0.0, -16.0, 1.0);
        let input_entity: Entity = commands.spawn(input_bundle).id();
        selected_assembly.selected.spawn_bundle(&mut commands, pos).push_children(&[input_entity, output_entity]);
    }
}

pub fn produce_goods(
    mut commands: Commands,
    mut q_assembly: Query<(Entity, &mut ItemIOContainer, &AssemblyInput, &AssemblyOutput)>,
    mut q_assembly_timer: Query<&mut AssemblyTimer>,
    mut q_assembly_power: Query<&mut AssemblyPower>,
    mut q_jobs: Query<&mut Job>,
    q_items: Query<&Item>,
    time: Res<Time>
) {
    for (
        assembly_entity,
        mut assembly_items,
        assembly_input,
        assembly_output
    ) in q_assembly.iter_mut() {
        if assembly_items.input.items.is_empty() ||
        assembly_items.output.max_items == assembly_items.output.items.len() {
            continue;
        }

        let timer = q_assembly_timer.get_mut(assembly_entity);
        if let Ok(mut timer) = timer {
            if !timer.0.tick(time.delta()).just_finished() {
                continue;
            }
        }
        
        if let Ok(power) = q_assembly_power.get(assembly_entity) {
            match power.current_power {
                Power::Electrical(existing) | Power::Thermal(existing) | Power::Mechanical(existing) => {
                    if existing < power.power_cost { continue; };
                },
            }
        }

        let (Some(Some(mut input_entity)), Some(assembly_input)) = (assembly_items.input.items.last_mut(), &assembly_input.0) else { continue; };
        let Ok(item) = q_items.get(input_entity) else { continue; };
        if assembly_input != item {
            continue;
        }

        let mut finish_production = || {
            if let Ok(mut power) = q_assembly_power.get_mut(assembly_entity) {
                power.current_power = match power.current_power {
                    Power::Electrical(_) => {
                        Power::Electrical(0.0)
                    },
                    Power::Thermal(_) => {
                        Power::Thermal(0.0)
                    },
                    Power::Mechanical(_) => {
                        Power::Mechanical(0.0)
                    }
                };
                for entity in power.powering_entities.drain(..) {
                    let Ok(mut job) = q_jobs.get_mut(entity) else { continue };
                    let Some(current_job_i) = job.current_job else { continue };
                    let Some(current_job) = job.path.get_mut(current_job_i) else { continue };

                    current_job.job_status = JobStatus::Completed;
                }
            }
        };

        if let Some(assembly_output) = &assembly_output.0 {
            let mut output_entity_commands: bevy::ecs::system::EntityCommands<'_, '_, '_> = assembly_output.spawn_bundle(&mut commands);
            let output_entity = output_entity_commands.id();
            if let Ok(_) = assembly_items.output.add_item(Some(output_entity)) {
                if let Ok(_) = assembly_items.input.remove_item(Some(input_entity)) {
                    commands.entity(assembly_entity).remove_children(&[input_entity]);
                    commands.entity(input_entity).insert(DespawnLater);
                    commands.entity(assembly_entity).push_children(&[output_entity]);

                    finish_production();
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

                finish_production();
            }
        }
    }
}
