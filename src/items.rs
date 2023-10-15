use bevy::ecs::system::EntityCommands;

use crate::*;

#[derive(Component, PartialEq)]
pub enum Item {
    Good(Good),
    Resource(Resource)
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub sprite: SpriteBundle
}

pub trait ItemType<'a, 'w, 's>: Component {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>
    ) -> EntityCommands<'w, 's, 'a>;
}

pub struct ItemContainer {
    pub items: Vec<Option<Entity>>,
    pub max_items: usize,
}

impl ItemContainer {
    pub fn new(max_items: usize) -> Self {
        ItemContainer {
            items: Vec::new(),
            max_items,
        }
    }

    pub fn add_item(&mut self, item: Option<Entity>) -> Result<(), &'static str> {
        if self.items.len() >= self.max_items {
            return Err("Maximum number of items reached");
        }

        Ok(self.items.push(item))
    }

    pub fn remove_item(&mut self, index: usize) -> Result<Option<Entity>, &'static str> {
        if index >= self.items.len() {
            return Err("Invalid index");
        }

        Ok(self.items.remove(index))
    }

    pub fn get_items(&self) -> &[Option<Entity>] {
        &self.items
    }
}

#[derive(Component, PartialEq)]
pub enum Resource {
    Wood,
    Pulp
}

impl<'a, 'w, 's> ItemType<'a, 'w, 's> for Resource {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>
    ) -> EntityCommands<'w, 's, 'a> {
        match self {
            Resource::Wood => commands.spawn(WoodBundle::default()),
            Resource::Pulp => commands.spawn(PulpBundle::default())
        }
    }
}

#[derive(Bundle)]
pub struct ResourceBundle {
    pub item: Item,
    pub resource: Resource,
    pub sprite: SpriteBundle
}

#[derive(Component)]
pub struct Wood;

#[derive(Bundle)]
pub struct WoodBundle {
    pub item: Item,
    pub resource: Resource,
    pub sprite: SpriteBundle,
    pub marker: Wood,
}
impl Default for WoodBundle {
    fn default() -> Self {
        WoodBundle {
            marker: Wood,
            item: Item::Resource(Resource::Wood),
            resource: Resource::Wood,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::ORANGE_RED,
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                ..default()
            }
        }
    }
}

#[derive(Component)]
pub struct Pulp;

#[derive(Bundle)]
pub struct PulpBundle {
    pub item: Item,
    pub resource: Resource,
    pub sprite: SpriteBundle,
    pub marker: Pulp
}
impl Default for PulpBundle {
    fn default() -> Self {
        PulpBundle {
            marker: Pulp,
            item: Item::Resource(Resource::Pulp),
            resource: Resource::Pulp,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::OLIVE,
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                ..default()
            }
        }
    }
}



#[derive(Component, PartialEq)]
pub enum Good {
    Paper
}

impl<'a, 'w, 's> ItemType<'a, 'w, 's> for Good {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>
    ) -> EntityCommands<'w, 's, 'a> {
        match self {
            Good::Paper => commands.spawn(PaperBundle::default())
        }
    }
}

#[derive(Bundle)]
pub struct GoodBundle {
    pub item: Item,
    pub good: Good,
    pub sprite: SpriteBundle
}

#[derive(Component)]
pub struct Paper;

#[derive(Bundle)]
pub struct PaperBundle {
    pub item: Item,
    pub good: Good,
    pub marker: Paper,
    pub sprite: SpriteBundle
}
impl Default for PaperBundle {
    fn default() -> Self {
        PaperBundle {
            marker: Paper,
            item: Item::Good(Good::Paper),
            good: Good::Paper,
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
