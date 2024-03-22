use crate::*;

mod resource_types;
use bevy::{ecs::system::EntityCommands, reflect::Enum};
pub use resource_types::*;

// TODO: Macro
#[derive(Component, PartialEq, Debug, Reflect, Eq, Hash, Clone, Copy)]
pub enum ResourceItem {
    Wood,
    WoodChips,
    Lumber
}

impl ItemType for ResourceItem {
    fn get_name(&self) -> &str {
        self.variant_name()
    }
}

impl<'a, 'w, 's> ItemSpawn<'a, 'w, 's> for ResourceItem {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>,
        sprite_storage: &SpriteStorage
    ) -> EntityCommands<'w, 's, 'a> {
        match self {
            ResourceItem::Wood => commands.spawn(WoodBundle::default_with_sprites(sprite_storage)),
            ResourceItem::WoodChips => commands.spawn(WoodChipsBundle::default()),
            ResourceItem::Lumber => commands.spawn(LumberBundle::default_with_sprites(sprite_storage))
        }
    }

    fn spawn_bundle_with_transform(
        &self,
        commands: &'a mut Commands<'w, 's>,
        transform: Transform,
        sprite_storage: &SpriteStorage
    ) -> EntityCommands<'w, 's, 'a> {
        match self {
            ResourceItem::Wood => commands.spawn(WoodBundle {
                sprite: SpriteBundle {
                    transform,
                    ..WoodBundle::default_with_sprites(sprite_storage).sprite
                },
                ..WoodBundle::default_with_sprites(sprite_storage)
            }),
            ResourceItem::WoodChips => commands.spawn(WoodChipsBundle {
                sprite: SpriteBundle {
                    transform,
                    ..WoodChipsBundle::default().sprite
                },
                ..default()
            }),
            ResourceItem::Lumber => commands.spawn(LumberBundle {
                sprite: SpriteBundle {
                    transform,
                    ..LumberBundle::default_with_sprites(sprite_storage).sprite
                },
                ..LumberBundle::default_with_sprites(sprite_storage)
            })
        }
    }
}

#[derive(Bundle)]
pub struct ResourceBundle {
    pub item: Item,
    pub resource: ResourceItem,
    pub sprite: SpriteBundle
}
