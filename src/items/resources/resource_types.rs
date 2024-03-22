use crate::*;

#[derive(Component)]
pub struct Wood;

#[derive(Bundle)]
pub struct WoodBundle {
    pub item: Item,
    pub resource: ResourceItem,
    pub sprite: SpriteBundle,
    pub marker: Wood,
}

impl DefaultWithSprites for WoodBundle {
    fn default_with_sprites(sprite_sheets: &SpriteStorage) -> Self {
        WoodBundle {
            marker: Wood,
            item: Item::Resource(ResourceItem::Wood),
            resource: ResourceItem::Wood,
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                texture: sprite_sheets.wood.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 6.0),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct WoodChips;

#[derive(Bundle)]
pub struct WoodChipsBundle {
    pub item: Item,
    pub resource: ResourceItem,
    pub sprite: SpriteBundle,
    pub marker: WoodChips
}
impl Default for WoodChipsBundle {
    fn default() -> Self {
        WoodChipsBundle {
            marker: WoodChips,
            item: Item::Resource(ResourceItem::WoodChips),
            resource: ResourceItem::WoodChips,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::OLIVE,
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 6.0),
                ..default()
            }
        }
    }
}

#[derive(Component)]
pub struct Lumber;

#[derive(Bundle)]
pub struct LumberBundle {
    pub item: Item,
    pub resource: ResourceItem,
    pub sprite: SpriteBundle,
    pub marker: Lumber
}

impl DefaultWithSprites for LumberBundle {
    fn default_with_sprites(sprite_sheets: &SpriteStorage) -> Self {
        LumberBundle {
            marker: Lumber,
            item: Item::Resource(ResourceItem::Lumber),
            resource: ResourceItem::Lumber,
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                texture: sprite_sheets.lumber.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 6.0),
                ..default()
            },
        }
    }   
}