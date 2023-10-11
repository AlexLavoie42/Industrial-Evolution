use crate::*;

#[derive(Component)]
pub struct Item;

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub sprite: SpriteBundle
}

#[derive(Component)]
pub enum Material {
    Wood
}

#[derive(Bundle)]
pub struct MaterialBundle {
    pub item: Item,
    pub material: Material,
    pub sprite: SpriteBundle
}

#[derive(Component)]
pub struct Wood;

#[derive(Bundle)]
pub struct WoodBundle {
    pub material: MaterialBundle,
    pub marker: Wood
}
impl Default for WoodBundle {
    fn default() -> Self {
        WoodBundle {
            marker: Wood,
            material: MaterialBundle {
                item: Item,
                material: Material::Wood,
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(15.0, 15.0)),
                        ..default()
                    },
                    ..default()
                },
            },
        }
    }
}



#[derive(Component)]
pub struct Good;

#[derive(Bundle)]
pub struct GoodBundle {
    pub item: Item,
    pub good: Good,
}

#[derive(Component)]
pub struct Paper;

#[derive(Bundle)]
pub struct PaperBundle {
    pub good: GoodBundle,
    pub marker: Paper,
}
