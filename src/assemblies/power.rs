use crate::*;

#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
pub enum Power {
    Mechanical(f32),
    Thermal(f32),
    Electrical(f32)
}

impl std::ops::Sub for Power {
    type Output = Power;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Power::Mechanical(a), Power::Mechanical(b)) => Power::Mechanical(a - b),
            (Power::Thermal(a), Power::Thermal(b)) => Power::Thermal(a - b),
            (Power::Electrical(a), Power::Electrical(b)) => Power::Electrical(a - b),
            (Power::Mechanical(a), _) => Power::Mechanical(a),
            (Power::Thermal(a), _) => Power::Thermal(a),
            (Power::Electrical(a), _) => Power::Electrical(a),
        }
    }
}

impl std::ops::Add for Power {
    type Output = Power;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Power::Mechanical(a), Power::Mechanical(b)) => Power::Mechanical(a + b),
            (Power::Thermal(a), Power::Thermal(b)) => Power::Thermal(a + b),
            (Power::Electrical(a), Power::Electrical(b)) => Power::Electrical(a + b),
            (Power::Mechanical(a), _) => Power::Mechanical(a),
            (Power::Thermal(a), _) => Power::Thermal(a),
            (Power::Electrical(a), _) => Power::Electrical(a),
        }
    }
}

impl std::ops::Mul<f32> for Power {
    type Output = Power;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Power::Mechanical(a) => Power::Mechanical(a * rhs),
            Power::Thermal(a) => Power::Thermal(a * rhs),
            Power::Electrical(a) => Power::Electrical(a * rhs),
        }
    }
}

impl std::ops::Div<f32> for Power {
    type Output = Power;

    fn div(self, rhs: f32) -> Self::Output {
        match self {
            Power::Mechanical(a) => Power::Mechanical(a / rhs),
            Power::Thermal(a) => Power::Thermal(a / rhs),
            Power::Electrical(a) => Power::Electrical(a / rhs),
        }
    }
}

#[derive(Event)]
pub struct AssemblyPowerInput {
    pub assembly: Entity,
    pub source: Entity,
    pub power: Power,
}

pub fn add_assembly_power_input(
    mut ev_power_input: EventReader<AssemblyPowerInput>,
    mut q_assembly_power: Query<&mut AssemblyPower>,
    mut q_job_error: Query<&mut JobError>
) {
    for mut assembly in q_assembly_power.iter_mut() {
        assembly.current_power = match assembly.current_power {
            Power::Mechanical(_) => Power::Mechanical(0.0),
            Power::Thermal(_) => Power::Thermal(0.0),
            Power::Electrical(_) => Power::Electrical(0.0)
        };
        assembly.powering_entities.clear();
    }
    for ev in ev_power_input.iter() {
        if let Ok(mut assembly) = q_assembly_power.get_mut(ev.assembly) {
            fn handle_power(input_power: Power, input_amount: f32, assembly: &mut AssemblyPower, source: Entity) {
                if let Some(existing) = match (input_power, &assembly.current_power) {
                    (Power::Electrical(_), Power::Electrical(e)) => Some(*e),
                    (Power::Thermal(_), Power::Thermal(e)) => Some(*e),
                    (Power::Mechanical(_), Power::Mechanical(e)) => Some(*e),
                    _ => None,
                } {
                    if assembly.powering_entities.contains(&source) {
                        return;
                    }
                    if existing + input_amount > assembly.max_power {
                        assembly.current_power = input_power;
                        return;
                    }
                    assembly.current_power = match input_power {
                        Power::Electrical(_) => Power::Electrical(existing + input_amount),
                        Power::Thermal(_) => Power::Thermal(existing + input_amount),
                        Power::Mechanical(_) => Power::Mechanical(existing + input_amount),
                    };
                    assembly.powering_entities.push(source);
                }
            }

            match ev.power {
                Power::Electrical(input_amount) => handle_power(Power::Electrical(input_amount), input_amount, &mut assembly, ev.source),
                Power::Thermal(input_amount) => handle_power(Power::Thermal(input_amount), input_amount, &mut assembly, ev.source),
                Power::Mechanical(input_amount) => handle_power(Power::Mechanical(input_amount), input_amount, &mut assembly, ev.source),
            }

            if let Ok(mut job_error) = q_job_error.get_mut(ev.source) {
                job_error.set_error("Wrong power type");
            }
        }
    }
}

pub fn assembly_power_display(
    mut commands: Commands,
    q_assembly_power: Query<(Entity, &AssemblyPower, &Children)>,
    mut q_text: Query<&mut Text>
) {
    for (entity, assembly, children) in q_assembly_power.iter() {
        if let Some(power) = match assembly.current_power {
            Power::Mechanical(power) => if power > 0.0 { Some(power) } else { None },
            Power::Thermal(power) => if power > 0.0 { Some(power) } else { None },
            Power::Electrical(power) => if power > 0.0 { Some(power) } else { None },
        } {
            match children.iter().find(|&child| q_text.get_mut(*child).is_ok()) {
                Some(mut text_entity) => {
                    let mut text = q_text.get_mut(*text_entity).unwrap();
                    text.sections[0].value = format!("{:.2} / {:.2} Power", power, assembly.power_cost);
                },
                None => {
                    let text = commands.spawn(Text2dBundle {
                        text: Text {
                            sections: vec![
                                TextSection {
                                    value: format!("{:.2} / {:.2} Power", power, assembly.power_cost),
                                    style: TextStyle {
                                        font_size: 32.0,
                                        color: Color::BLACK,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }
                            ],
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(0.0, 15.0, 50.0),
                        ..Default::default()
                    }).id();
                
                    commands.entity(entity).push_children(&[
                        text
                    ]);
                }
            }
        } else {
            if let Some(mut text_entity) = children.iter().find(|&child| q_text.get_mut(*child).is_ok()) {
                commands.entity(*text_entity).despawn_recursive();
            }
        }
    }
}
