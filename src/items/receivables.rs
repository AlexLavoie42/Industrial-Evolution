use bevy::sprite::Anchor;

use crate::*;

#[derive(Component)]
pub struct ItemReceivable;
#[derive(Bundle)]
pub struct ItemReceivableBundle {
    pub marker: ItemReceivable,
    pub container: ItemContainer,
    pub sprite: SpriteBundle,
    pub solid: SolidEntity,
    pub tile_size: EntityTileSize
}
impl GetGhostBundle for ItemReceivableBundle {
    fn get_sprite_bundle(&self) -> SpriteBundle {
        self.sprite.clone()
    }
    fn get_tile_size(&self) -> Option<EntityTileSize> {
        Some(self.tile_size)
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
            marker: ItemReceivable,
            container: ItemContainer {
                items: Vec::new(),
                item_type: None,
                max_items: 100
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(32.0, 64.0)),
                    ..default()
                },
                ..default()
            },
            solid: SolidEntity,
            tile_size: EntityTileSize(IVec2 { x: 2, y: 4 })
        }
    }
}

pub fn purchase_receivables(
    mut commands: Commands,
    mut q_receivables: Query<(Entity, &mut ItemContainer), With<ItemReceivable>>,
    mut selected_receivables: ResMut<ReceivableSelections>,
    mut economy: ResMut<Economy>,
    mut money: ResMut<PlayerMoney>,
) {
    for (receivable_entity, mut container) in q_receivables.iter_mut() {
        if container.items.len() < container.max_items {
            for selection in selected_receivables.selected.iter() {
                let mut item_command = match selection {
                    PurchasableItem::Good(item) => item.spawn_bundle(&mut commands),
                    PurchasableItem::Resource(item) => item.spawn_bundle(&mut commands)
                };
                let item_entity = item_command.id();

                let mut selected_item = match selection {
                    PurchasableItem::Good(item) => Item::Good(*item),
                    PurchasableItem::Resource(item) => Item::Resource(*item),
                };
                match container.add_item((Some(item_entity), Some(selected_item))) {
                    Ok(_) => {
                        commands.entity(receivable_entity).push_children(&[item_entity]);
                        
                        let Some(price) = selected_item.get_price(&economy) else { continue; };
                        let Ok(_) = money.try_remove_money(price) else { continue; };
                        let Ok(_) = selected_item.buy(&mut economy, 1) else { continue; };
                    },
                    Err(e) => {
                        println!("Error adding item to container: {:?}", e);
                        item_command.despawn_recursive();
                    }
                }
            }
        }
    }
    selected_receivables.selected.clear();
}

pub const STORAGE_FEE: f32 = 5.0;

pub fn receivables_storage_fee(
    mut money: ResMut<PlayerMoney>,
    mut upkeep_tracker: ResMut<UpkeepTracker>,
    mut q_receivables: Query<&mut ItemContainer, With<ItemReceivable>>
) {
    for mut container in q_receivables.iter_mut() {
        for mut item in container.items.iter_mut() {
            if let Some(item_entity) = item {
                if money.amount < STORAGE_FEE {
                    println!("Player does not have enough money to pay storage fee");
                    return;
                }
                money.amount -= STORAGE_FEE;
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
        if state.get() == &PlayerState::Receivables {
            next_state.set(PlayerState::None);
        } else {
            next_state.set(PlayerState::Receivables);
            
        }
    }
}

pub fn place_receivable(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    asset_server: Res<AssetServer>,
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

        let size = ItemReceivableBundle::default().tile_size.0;
        let pos = get_corner_tile_pos(get_tile_world_pos(&tile_pos, map_transform, grid_size, map_type), size);

        let mut output_bundle = ContainerOutputSelectorBundle::new(asset_server.clone());
        output_bundle.sprite.transform.translation = Vec3::new(0.0, -42.0, 1.0);
        let output_entity = commands.spawn(output_bundle).id();

        commands.spawn(ItemReceivableBundle::from_translation(Vec3 { x: pos.x, y: pos.y, z: 1.0 })).push_children(&[output_entity]);
    }
}
