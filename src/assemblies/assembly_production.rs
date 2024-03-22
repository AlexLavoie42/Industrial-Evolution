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
                transform: Transform::from_xyz(0.0, 47.0, 25.0),
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

pub fn produce_goods(
    mut commands: Commands,
    mut q_assembly: Query<(Entity, &mut ItemIOContainer, &AssemblyInput, &AssemblyOutput)>,
    mut q_assembly_timer: Query<&mut AssemblyTimer>,
    mut q_assembly_power: Query<&mut AssemblyPower>,
    mut q_jobs: Query<&mut Job>,
    q_items: Query<&Item>,
    time: Res<Time>,
    sprites: Res<SpriteStorage>,
) {
    for (
        assembly_entity,
        mut assembly_items,
        assembly_input,
        assembly_output
    ) in q_assembly.iter_mut() {
        let mut timer_item = q_assembly_timer.get_mut(assembly_entity).as_ref().map(|t| t.item).unwrap_or(None);
        if !assembly_items.input.items.contains(&timer_item) {
            timer_item = None;
        }

        if let Ok(mut timer) = q_assembly_timer.get_mut(assembly_entity) {
            if timer_item.is_none() {
                timer.timer.reset();
                let next_item = assembly_items.input.items.get(0);
                if let Some(next_item) = next_item {
                    timer.item = *next_item;
                    timer_item = *next_item;
                }
            }
        }

        let (Some(Some(mut input_entity)), Some(assembly_input)) = (assembly_items.input.items.last_mut(), &assembly_input.0) else { continue; };
        let Ok(item) = q_items.get(input_entity) else { continue; };
        if assembly_input != item {
            continue;
        }

        if assembly_items.input.items.is_empty() ||
        assembly_items.output.max_items == assembly_items.output.items.len() {
            continue;
        }
        let mut power_mult = 1.0;
        if let Ok(power) = q_assembly_power.get(assembly_entity) {
            match power.current_power {
                Power::Electrical(existing) | Power::Thermal(existing) | Power::Mechanical(existing) => {
                    if existing < power.power_cost { continue; };
                    power_mult = existing / power.power_cost;
                },
            }
        }

        if let Ok(mut timer) = q_assembly_timer.get_mut(assembly_entity) {
            if timer_item.is_none() || !timer.timer.tick(time.delta().mul_f32(power_mult)).just_finished() {
                continue;
            }
        }

        let mut finish_production = || {
            if let Ok(mut power) = q_assembly_power.get_mut(assembly_entity) {
                power.current_power = match power.current_power {
                    Power::Electrical(_) => {
                        Power::Electrical(0.0)
                    },
                    Power::Thermal(_) => {
                        Power::Thermal(0.0)
                    },
                    Power::Mechanical(_) => {
                        Power::Mechanical(0.0)
                    }
                };
                for entity in power.powering_entities.drain(..) {
                    let Ok(mut job) = q_jobs.get_mut(entity) else { continue };
                    let Some(current_job_i) = job.current_job else { continue };
                    let Some(current_job) = job.path.get_mut(current_job_i) else { continue };
                    
                    current_job.job_status = JobStatus::Completed;
                }
            }
        };

        if let Some(assembly_output) = &assembly_output.0 {
            let mut output_entity_commands: bevy::ecs::system::EntityCommands<'_, '_, '_> =
                assembly_output.spawn_bundle_with_transform(&mut commands, assembly_items.output.get_transform(), sprites.as_ref());

            let output_entity = output_entity_commands.id();
            if let Ok(_) = assembly_items.output.add_item((Some(output_entity), Some(*assembly_output))) {
                if let Ok(_) = assembly_items.input.remove_item(Some(input_entity)) {
                    commands.entity(assembly_entity).remove_children(&[input_entity]);
                    commands.entity(input_entity).insert(DespawnLater);
                    commands.entity(assembly_entity).push_children(&[output_entity]);

                    finish_production();
                } else {
                    output_entity_commands.despawn();
                    if let Err(err) = assembly_items.output.remove_item(Some(output_entity)) {}
                }
            } else {
                output_entity_commands.despawn();
                if let Err(err) = assembly_items.output.remove_item(Some(output_entity)) {}
            }
        } else {
            if let Ok(_) = assembly_items.input.remove_item(Some(input_entity)) {
                commands.entity(assembly_entity).remove_children(&[input_entity]);
                commands.entity(input_entity).insert(DespawnLater);

                finish_production();
            }
        }
    }
}
