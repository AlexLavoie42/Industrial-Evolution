use crate::*;

#[derive(Component, Debug)]
pub struct Assembly;
impl Clickable for Assembly {}

#[derive(Resource, Reflect, Clone, Copy)]
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
    pub power_cost: f32,
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
    q_assembly: Query<&AssemblyType>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<PlayerMoney>,
    assembly_prices: Res<AssemblyPrices>
) {
    if input.just_pressed(KeyCode::Delete) {
        let Some(ev) = ev_assembly_mouse.iter().next() else { return };
        let Some((_, assembly)) = ev.collision else { return };
        let Ok(assembly_type) = q_assembly.get(assembly) else { return };
        if let Some(price) = assembly_prices.prices.get(assembly_type) {
            money.add_money(*price);
        }
        commands.entity(assembly).despawn_recursive();
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
    asset_server: Res<AssetServer>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &Transform
    )>,
    q_collision_tiles: Query<&TilePos, With<TileMapCollision>>,
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
        let size = selected_assembly.selected.get_tile_size().0;
        let pos = get_corner_tile_pos(get_tile_world_pos(&tile_pos, map_transform, grid_size, map_type), size);
        if q_collision_tiles.iter().any(|p| *p == tile_pos) {
            println!("Can't place assembly here");
            return;
        }
        let mut output_bundle = ContainerOutputSelectorBundle::new(asset_server.clone());

        output_bundle.sprite.transform.translation = Vec3::new(0.0, (size.y as f32) * TILE_SIZE.y, 1.0);
        let output_entity = commands.spawn(output_bundle).id();

        let mut input_bundle = ContainerInputSelectorBundle::new(asset_server.clone());
        input_bundle.sprite.transform.translation = Vec3::new(0.0, -(size.y as f32) * TILE_SIZE.y, 1.0);
        let input_entity: Entity = commands.spawn(input_bundle).id();
        selected_assembly.selected.spawn_bundle(&mut commands, pos).push_children(&[input_entity, output_entity]);
    }
}
