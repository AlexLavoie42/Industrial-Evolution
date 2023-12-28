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
impl Default for WoodBundle {
    fn default() -> Self {
        WoodBundle {
            marker: Wood,
            item: Item::Resource(ResourceItem::Wood),
            resource: ResourceItem::Wood,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(83, 53, 10),
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
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
impl Default for LumberBundle {
    fn default() -> Self {
        LumberBundle {
            marker: Lumber,
            item: Item::Resource(ResourceItem::Lumber),
            resource: ResourceItem::Lumber,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(161, 159, 124),
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 6.0),
                ..default()
            }
        }
    }
}