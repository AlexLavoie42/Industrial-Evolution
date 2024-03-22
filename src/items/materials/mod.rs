use bevy::{ecs::system::EntityCommands, reflect::Enum};

use crate::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Reflect, Hash)]
pub enum MaterialItem {
    WoodPulp
}

#[derive(Bundle)]
pub struct WoodPulpBundle {
    pub item: Item,
    pub material: MaterialItem,
    pub sprite: SpriteBundle
}
impl Default for WoodPulpBundle {
    fn default() -> WoodPulpBundle {
        WoodPulpBundle {
            item: Item::Material(MaterialItem::WoodPulp),
            material: MaterialItem::WoodPulp,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::ANTIQUE_WHITE,
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 6.0),
                ..default()
            }
        }
    }
}

impl ItemType for MaterialItem {
    fn get_name(&self) -> &str {
            self.variant_name()
    }
}

impl<'a, 'w, 's> ItemSpawn<'a, 'w, 's> for MaterialItem {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>,
        sprites: &SpriteStorage,
    ) -> EntityCommands<'w, 's, 'a> {
        match self {
            MaterialItem::WoodPulp => {
                commands.spawn(WoodPulpBundle::default())
            }
        }
    }

    fn spawn_bundle_with_transform(
        &self,
        commands: &'a mut Commands<'w, 's>,
        transform: Transform,
        sprites: &SpriteStorage,
    ) -> EntityCommands<'w, 's, 'a> {
        match self {
            MaterialItem::WoodPulp => {
                commands.spawn(WoodPulpBundle {
                    sprite: SpriteBundle {
                        transform,
                        ..WoodPulpBundle::default().sprite
                    },
                    ..Default::default()
                })
            }
        }
    }
}
