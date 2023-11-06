use bevy::ecs::system::EntityCommands;

use crate::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Reflect, Hash)]
pub enum MaterialItem {
    WetPaper
}

#[derive(Bundle)]
pub struct WetPaperBundle {
    pub item: Item,
    pub material: MaterialItem,
    pub sprite: SpriteBundle
}
impl Default for WetPaperBundle {
    fn default() -> WetPaperBundle {
        WetPaperBundle {
            item: Item::Material(MaterialItem::WetPaper),
            material: MaterialItem::WetPaper,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::ANTIQUE_WHITE,
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                ..default()
            }
        }
    }
}

impl ItemType for MaterialItem {}

impl<'a, 'w, 's> ItemSpawn<'a, 'w, 's> for MaterialItem {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>
    ) -> EntityCommands<'w, 's, 'a> {
        match self {
            MaterialItem::WetPaper => {
                commands.spawn(WetPaperBundle::default())
            }
        }
    }
}
