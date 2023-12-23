use crate::*;

mod resource_types;
use bevy::{ecs::system::EntityCommands, reflect::Enum};
pub use resource_types::*;

#[derive(Component, PartialEq, Debug, Reflect, Eq, Hash, Clone, Copy)]
pub enum ResourceItem {
    Wood,
    Pulp
}

impl ItemType for ResourceItem {
    fn get_name(&self) -> &str {
        self.variant_name()
    }
}

impl<'a, 'w, 's> ItemSpawn<'a, 'w, 's> for ResourceItem {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>
    ) -> EntityCommands<'w, 's, 'a> {
        match self {
            ResourceItem::Wood => commands.spawn(WoodBundle::default()),
            ResourceItem::Pulp => commands.spawn(PulpBundle::default())
        }
    }
}

#[derive(Bundle)]
pub struct ResourceBundle {
    pub item: Item,
    pub resource: ResourceItem,
    pub sprite: SpriteBundle
}
