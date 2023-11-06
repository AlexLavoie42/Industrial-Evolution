use crate::*;

#[derive(Component, Debug, Reflect)]
pub struct TradeDepot;

#[derive(Bundle)]
pub struct TradeDepotBundle {
    pub depot: TradeDepot,
    pub sprite: SpriteBundle,
    pub items: ItemContainer
}

impl TradeDepotBundle {
    pub fn from_translation(translation: Vec3) -> Self {
        let mut bundle = TradeDepotBundle::default();
        bundle.sprite.transform.translation = translation;
        return bundle;
    }
}

impl Default for TradeDepotBundle {
    fn default() -> Self {
        TradeDepotBundle {
            depot: TradeDepot,
            items: ItemContainer {
                items: Vec::new(),
                max_items: 25
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::ORANGE,
                    custom_size: Some(Vec2::new(25.0, 50.0)),
                    ..default()
                },
                ..default()
            }
        }
    }
}

pub fn sell_trade_depot_items(
    mut commands: Commands,
    mut economy: ResMut<Economy>,
    mut money: ResMut<PlayerMoney>,
    mut q_items: Query<&mut Item>,
    mut q_depot: Query<(&TradeDepot, &mut ItemContainer)>
) {
    for (depot, mut container) in q_depot.iter_mut() {
        let mut container_ref = container;
        container_ref.items.retain(|item_entity| {
            let Some(item_entity) = item_entity else { return true; };
            let Ok(mut item) = q_items.get_mut(*item_entity) else { return true; };
            
            let Some(price) = item.get_price(&economy) else { return true; };

            match item.sell(&mut economy, 1) {
                Ok(_) => {},
                Err(e) => {
                    println!("{:?}", e);
                    return true
                }
            }
            println!("Selling item: {:?}", item_entity);
            money.add_money(price);

            commands.entity(*item_entity).despawn_recursive();
            return false;
        });
    }
}

pub fn input_toggle_trade_depot_mode(
    input: Res<Input<KeyCode>>,
    state: Res<State<PlayerState>>,
    mut next_state: ResMut<NextState<PlayerState>>
) {
    if input.just_pressed(KeyCode::T) {
        if state.get() == &PlayerState::TradeDepot {
            next_state.set(PlayerState::None);
        } else {
            next_state.set(PlayerState::TradeDepot);
            
        }
    }
}

pub fn place_trade_depot(
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

        let Some(tile_pos) = get_mouse_tile(window, camera, camera_transform, tilemap_size, grid_size, map_type, map_transform) else { return };
        let pos = get_tile_world_pos(&tile_pos, map_transform, grid_size, map_type);

        let mut input_bundle = ContainerInputSelectorBundle::default();
        input_bundle.sprite.transform.translation = Vec3::new(0.0, 42.0, 1.0);
        let input_entity = commands.spawn(input_bundle).id();

        commands.spawn(TradeDepotBundle::from_translation(Vec3 { x: pos.x, y: pos.y, z: 1.0 })).push_children(&[input_entity]);
    }
}
