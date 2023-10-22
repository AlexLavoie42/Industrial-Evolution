use bevy::ecs::system::EntityCommands;

use crate::*;

mod resources;
pub use resources::*;

mod goods;
pub use goods::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, mouse_collision_system::<Item>)
            .add_event::<GenericMouseCollisionEvent<Item>>()
        ;
    }
}

#[derive(Component, PartialEq)]
pub enum Item {
    Good(GoodItem),
    Resource(ResourceItem)
}

impl Clickable for Item {}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub sprite: SpriteBundle
}

pub trait ItemType {}

pub trait ItemSpawn<'a, 'w, 's>: Component {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>
    ) -> EntityCommands<'w, 's, 'a>;
}

#[derive(Component)]
pub struct ItemContainer {
    pub items: Vec<Option<Entity>>,
    pub max_items: usize,
}

impl ItemContainer {
    pub fn add_item(&mut self, item: Option<Entity>) -> Result<(), &'static str> {
        if self.items.len() >= self.max_items {
            return Err("Maximum number of items reached");
        }

        Ok(self.items.push(item))
    }

    pub fn remove_item(&mut self, item: Option<Entity>) -> Result<Option<Entity>, &'static str> {
        let item_i = self.items.iter().position(|&x| x == item);
        if let Some(index) = item_i {
            if index >= self.items.len() {
                return Err("Invalid index");
            }

            return Ok(self.items.remove(index));
        } else {
            return Err("Item not found");
        }
    }

    pub fn remove_index(&mut self, index: usize) -> Result<Option<Entity>, &'static str> {
        if index >= self.items.len() {
            return Err("Invalid index");
        }

        Ok(self.items.remove(index))
    }

    pub fn get_items(&self) -> &[Option<Entity>] {
        &self.items
    }
}
