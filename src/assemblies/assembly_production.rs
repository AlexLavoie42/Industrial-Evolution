use bevy::sprite::Anchor;

use crate::*;

#[derive(Component, Debug)]
pub struct AssemblyInput(pub Option<Item>);

#[derive(Component, Debug)]
pub struct AssemblyOutput(pub Option<Item>);

#[derive(Component, Debug)]
pub struct AssemblyTimer {
    pub timer: Timer,
    pub item: Option<Entity>
}

#[derive(Component)]
pub struct AssemblyProgressBarBase;

#[derive(Bundle)]
pub struct AssemblyProgressBarBaseBundle {
    pub base: AssemblyProgressBarBase,
    pub sprite: SpriteBundle,
}
impl Default for AssemblyProgressBarBaseBundle {
    fn default() -> Self {
        Self {
            base: AssemblyProgressBarBase,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2::new(100.0, 15.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 25.0),
                ..default()
            }
        }
    }
}

#[derive(Component)]
pub struct AssemblyProgressBarSection {
    pub progress: f32
}

#[derive(Bundle)]
pub struct AssemblyProgressBarSectionBundle {
    pub section: AssemblyProgressBarSection,
    pub sprite: SpriteBundle,
}
impl Default for AssemblyProgressBarSectionBundle {
    fn default() -> Self {
        Self {
            section: AssemblyProgressBarSection { progress: 0.0 },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(0.0, 13.0)),
                    anchor: Anchor::CenterLeft,
                    ..default()
                },
                transform: Transform::from_xyz(-50.0, 0.0, 30.0),
                ..default()
            }
        }
    }
}

pub fn show_assembly_progress_bars(
    mut commands: Commands,
    q_assemblies: Query<(Entity, &Transform, &AssemblyTimer, &Children)>,
    q_assembly_progress_bars: Query<Entity, With<AssemblyProgressBarBase>>,
) {
    for (assembly_entity, transform, timer, children) in q_assemblies.iter() {
        let existing = children.iter().find(|child| q_assembly_progress_bars.get(**child).is_ok());
        if timer.item.is_none() || timer.timer.percent() <= 0.0 {
            if let Some(existing) = existing {
                commands.entity(*existing).despawn_recursive();
            }
            continue;
        } else if existing.is_some() {
            continue;
        }

        let progress_bar_entity = commands
            .spawn(AssemblyProgressBarSectionBundle::default())
            .id();

        let mut progress_base = AssemblyProgressBarBaseBundle::default();

        let base_entity = commands
            .spawn(progress_base)
            .add_child(progress_bar_entity)
            .id();

        commands.entity(assembly_entity).add_child(base_entity);
    }
}

pub fn update_assembly_progress_bars(
    q_assemblies: Query<(&Children, &AssemblyTimer)>,
    q_assembly_progress_bases: Query<&Children, With<AssemblyProgressBarBase>>,
    mut q_assembly_progress_bars: Query<(&mut AssemblyProgressBarSection, &mut Sprite)>,
) {
    for (children, timer) in q_assemblies.iter() {
        for child in children.iter() {
            if let Ok(base) = q_assembly_progress_bases.get(*child) {
                for base_child in base.iter() {
                    if let Ok((mut section, mut sprite)) = q_assembly_progress_bars.get_mut(*base_child) {
                        section.progress = timer.timer.percent();
                        
                        sprite.custom_size = Some(Vec2::new(
                            100.0 * section.progress,
                            13.0
                        ))
                    }
                }
            }
        }
    }
}
