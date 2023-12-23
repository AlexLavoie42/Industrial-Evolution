use crate::*;

mod good_types;
use bevy::{ecs::system::EntityCommands, reflect::Enum};
pub use good_types::*;

#[derive(Component, PartialEq, Debug, Reflect, Eq, Hash, Clone, Copy)]
pub enum GoodItem {
    Paper
}

impl ItemType for GoodItem {
    fn get_name (&self) -> &str {
        self.variant_name()
    }
}

impl<'a, 'w, 's> ItemSpawn<'a, 'w, 's> for GoodItem {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>
    ) -> EntityCommands<'w, 's, 'a> {
        match self {
            GoodItem::Paper => commands.spawn(PaperBundle::default())
        }
    }
}

#[derive(Bundle)]
pub struct GoodBundle {
    pub item: Item,
    pub good: GoodItem,
    pub sprite: SpriteBundle
}
