use crate::*;

#[derive(Component, Debug, Reflect)]
pub struct ItemExport;

#[derive(Bundle)]
pub struct ItemExportBundle {
    pub depot: ItemExport,
    pub sprite: SpriteBundle,
    pub items: ItemContainer
}
impl GetGhostBundle for ItemExportBundle {
    fn get_sprite_bundle(&self) -> Option<SpriteBundle> {
        Some(self.sprite.clone())
    }
    fn get_tile_size(&self) -> Option<EntityTileSize> {
        None
    }
}
impl ItemExportBundle {
    pub fn from_translation(translation: Vec3, sprites: &SpriteStorage) -> Self {
        let mut bundle = ItemExportBundle::default_with_sprites(sprites);
        bundle.sprite.transform.translation = translation;
        return bundle;
    }
}

#[derive(Resource, Default)]
pub struct SoldItems {
    pub items: Vec<(Item, f32)>
}

impl DefaultWithSprites for ItemExportBundle {
    fn default_with_sprites(sprites: &SpriteStorage) -> Self {
        ItemExportBundle {
            depot: ItemExport,
            items: ItemContainer {
                items: Vec::new(),
                item_type: None,
                max_items: 100
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 64.0)),
                    ..default()
                },
                texture: sprites.exports.clone(),
                ..default()
            }
        }
    }
}

pub fn calculate_sold_items(
    mut q_items: Query<&mut Item>,
    mut economy: ResMut<Economy>,
    mut q_depot: Query<(&ItemExport, &mut ItemContainer)>,
    mut sold_items: ResMut<SoldItems>
) {
    for (depot, mut container) in q_depot.iter_mut() {
        let mut container_ref = container;
        for item in container_ref.items.iter() {
            let Some(item_entity) = item else { continue; };
            let Ok(mut item) = q_items.get_mut(*item_entity) else { continue; };
            
            let Some(price) = item.get_price(&economy) else { continue; };
            sold_items.items.push((item.clone(), price));
        }
    }
}

pub fn sell_export_items(
    mut commands: Commands,
    mut economy: ResMut<Economy>,
    mut money: ResMut<PlayerMoney>,
    mut q_items: Query<&mut Item>,
    mut q_depot: Query<(&ItemExport, &mut ItemContainer)>,
    mut sold_items: ResMut<SoldItems>
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
            sold_items.items.push((item.clone(), price));
            println!("Selling item: {:?}", item_entity);
            money.add_money(price);

            commands.entity(*item_entity).insert(DespawnLater);
            return false;
        });
        // sold_items.items.clear();
    }
}

pub fn input_toggle_place_export_mode(
    input: Res<Input<KeyCode>>,
    state: Res<State<PlayerState>>,
    mut next_state: ResMut<NextState<PlayerState>>
) {
    if input.just_pressed(KeyCode::T) {
        if state.get() == &PlayerState::Export {
            next_state.set(PlayerState::None);
        } else {
            next_state.set(PlayerState::Export);
            
        }
    }
}

pub fn place_item_export(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    asset_server: Res<AssetServer>,
    sprites: Res<SpriteStorage>,
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

        let mut input_bundle = ContainerInputSelectorBundle::new(asset_server.clone());
        input_bundle.sprite.transform.translation = Vec3::new(0.0, 42.0, 1.0);
        let input_entity = commands.spawn(input_bundle).id();

        commands.spawn(ItemExportBundle::from_translation(Vec3 { x: pos.x, y: pos.y, z: 1.0 }, &sprites))
            .push_children(&[input_entity]);
    }
}
