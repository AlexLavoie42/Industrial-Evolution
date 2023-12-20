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
pub struct Pulp;

#[derive(Bundle)]
pub struct PulpBundle {
    pub item: Item,
    pub resource: ResourceItem,
    pub sprite: SpriteBundle,
    pub marker: Pulp
}
impl Default for PulpBundle {
    fn default() -> Self {
        PulpBundle {
            marker: Pulp,
            item: Item::Resource(ResourceItem::Pulp),
            resource: ResourceItem::Pulp,
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