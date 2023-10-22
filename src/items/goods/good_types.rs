use crate::*;

#[derive(Component)]
pub struct Paper;

#[derive(Bundle)]
pub struct PaperBundle {
    pub item: Item,
    pub good: GoodItem,
    pub marker: Paper,
    pub sprite: SpriteBundle
}
impl Default for PaperBundle {
    fn default() -> Self {
        PaperBundle {
            marker: Paper,
            item: Item::Good(GoodItem::Paper),
            good: GoodItem::Paper,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                ..default()
            }
        }
    }
}
