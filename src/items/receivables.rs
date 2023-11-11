use crate::*;

#[derive(Component, Debug, Reflect)]
pub struct ItemReceivable {
    pub item: ResourceItem,
    pub refill_quantity: i32,
    pub refill_limit: usize
}

#[derive(Bundle)]
pub struct ItemReceivableBundle {
    pub item: ItemReceivable,
    pub container: ItemContainer,
    pub sprite: SpriteBundle
}
impl GetSpriteBundle for ItemReceivableBundle {
    fn get_sprite_bundle(&self) -> SpriteBundle {
        self.sprite.clone()
    }
}
impl ItemReceivableBundle {
    pub fn from_translation(translation: Vec3) -> Self {
        let mut bundle = ItemReceivableBundle::default();
        bundle.sprite.transform.translation = translation;
        return bundle;
    }
}

impl Default for ItemReceivableBundle {
    fn default() -> Self {
        ItemReceivableBundle {
            item: ItemReceivable {
                item: ResourceItem::Wood,
                refill_quantity: 100,
                refill_limit: 20
            },
            container: ItemContainer {
                items: Vec::new(),
                max_items: 100
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

pub fn purchase_receivables(
    mut commands: Commands,
    mut q_receivables: Query<(Entity, &mut ItemReceivable, &mut ItemContainer)>,
    mut economy: ResMut<Economy>,
    mut money: ResMut<PlayerMoney>,
) {
    for (receivable_entity, receivable, mut container) in q_receivables.iter_mut() {
        if container.items.len() < receivable.refill_limit {
            // TODO: Timer
            for _ in 0..receivable.refill_quantity {
                let mut item = receivable.item.spawn_bundle(&mut commands);
                let item_entity = item.id();
                match container.add_item(Some(item_entity)) {
                    Ok(_) => {
                        commands.entity(receivable_entity).push_children(&[item_entity]);
                        
                        let Some(price) = Item::Resource(receivable.item).get_price(&economy) else { continue; };
                        let Ok(_) = money.try_remove_money(price) else { continue; };
                        let Ok(_) = Item::Resource(receivable.item).buy(&mut economy, 1) else { continue; };
                    },
                    Err(e) => {
                        println!("Error adding item to container: {:?}", e);
                        item.despawn_recursive();
                    }
                }
            }
        }
    }
}

pub fn input_toggle_receivable_mode(
    input: Res<Input<KeyCode>>,
    state: Res<State<PlayerState>>,
    mut next_state: ResMut<NextState<PlayerState>>
) {
    if input.just_pressed(KeyCode::R) {
        if state.get() == &PlayerState::Recievables {
            next_state.set(PlayerState::None);
        } else {
            next_state.set(PlayerState::Recievables);
            
        }
    }
}

pub fn place_receivable(
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

        let mut output_bundle = ContainerOutputSelectorBundle::default();
        output_bundle.sprite.transform.translation = Vec3::new(0.0, -42.0, 1.0);
        let output_entity = commands.spawn(output_bundle).id();

        commands.spawn(ItemReceivableBundle::from_translation(Vec3 { x: pos.x, y: pos.y, z: 1.0 })).push_children(&[output_entity]);
    }
}
