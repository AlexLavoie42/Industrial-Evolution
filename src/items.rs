use bevy::ecs::system::EntityCommands;

use crate::*;

#[derive(Component)]
pub struct Item;

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub sprite: SpriteBundle
}

#[derive(Component, PartialEq)]
pub enum Resource {
    Wood,
    Pulp
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
    pub marker: Wood
}
impl Default for WoodBundle {
    fn default() -> Self {
        WoodBundle {
            marker: Wood,
            item: Item,
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
            item: Item,
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



#[derive(Component)]
pub enum Good {
    Paper
}
pub trait GoodBehavior<'a, 'w, 's> {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>
    ) -> Option<EntityCommands<'w, 's, 'a>>;
}

impl<'a, 'w, 's> GoodBehavior<'a, 'w, 's> for Good {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>
    ) -> Option<EntityCommands<'w, 's, 'a>> {
        match self {
            Good::Paper => Some(commands.spawn(PaperBundle::default())),
            _ => None,
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
            item: Item,
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
