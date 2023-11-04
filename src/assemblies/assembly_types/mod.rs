use bevy::ecs::system::EntityCommands;

use crate::*;

pub mod assembly_templates;
use assembly_templates::*;

#[derive(Resource)]
pub enum AssemblyType {
    PulpMill
}

impl Default for AssemblyType {
    fn default() -> Self {
        AssemblyType::PulpMill
    }
}

pub trait AssemblySpawn<'a, 'w, 's> {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>,
        position: Vec2
    ) -> EntityCommands<'w, 's, 'a>;
}

impl<'a, 'w, 's> AssemblySpawn<'a, 'w, 's> for AssemblyType {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>,
        position: Vec2
    ) -> EntityCommands<'w, 's, 'a> {
        match self {
            AssemblyType::PulpMill => commands.spawn(PulpMillBundle {
                sprite: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(position.x, position.y, 1.0),
                        ..PulpMillBundle::default().sprite.transform
                    },
                    ..PulpMillBundle::default().sprite
                },
                ..default()
            })
        }
    }
}

#[derive(Bundle)]
pub struct AssemblyBundle {
    pub marker: Assembly,
    pub power: AssemblyPower,
    pub solid: SolidEntity,
    pub assembly_items: AssemblyItemContainer,
    pub sprite: SpriteBundle
}
impl Default for AssemblyBundle {
    fn default() -> AssemblyBundle {
        AssemblyBundle {
            marker: Assembly,
            solid: SolidEntity,
            power: AssemblyPower(Some(Power::Mechanical(0.0))),
            assembly_items: AssemblyItemContainer {
                input: ItemContainer {
                    items: Vec::new(),
                    max_items: 5
                },
                output: ItemContainer {
                    items: Vec::new(),
                    max_items: 3
                }
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            }
        }
    }
}
