use crate::*;

#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq)]
pub enum Power {
    Mechanical(f32),
    Thermal(f32),
    Electrical(f32)
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
) {
    for ev in ev_power_input.iter() {
        if let Ok(mut assembly) = q_assembly_power.get_mut(ev.assembly) {
            match ev.power {
                Power::Electrical(input_amount) => {
                    let input_power = assembly.current_power;
                    if let Power::Electrical(existing) = input_power {
                        if existing + input_amount > assembly.max_power {
                            assembly.current_power = Power::Electrical(assembly.max_power);
                            continue;
                        }
                        assembly.current_power = Power::Electrical(existing + input_amount);
                        assembly.powering_entities.push(ev.source);
                    }
                },
                Power::Thermal(input_amount) => {
                    let input_power = assembly.current_power;
                    if let Power::Thermal(existing) = input_power {
                        if existing + input_amount > assembly.max_power {
                            assembly.current_power = Power::Thermal(assembly.max_power);
                            continue;
                        }
                        assembly.current_power = Power::Thermal(existing + input_amount);
                        assembly.powering_entities.push(ev.source);
                    }
                },
                Power::Mechanical(input_amount) => {
                    let input_power = assembly.current_power;
                    if let Power::Mechanical(existing) = input_power {
                        if existing + input_amount > assembly.max_power {
                            assembly.current_power = Power::Mechanical(assembly.max_power);
                            continue;
                        }
                        assembly.current_power = Power::Mechanical(existing + input_amount);
                        assembly.powering_entities.push(ev.source);
                    }
                },
            }
        }
    }
}
