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
    pub power: Power
}

pub fn add_assembly_power_input(
    mut ev_power_input: EventReader<AssemblyPowerInput>,
    mut q_assembly_power: Query<&mut AssemblyPower>,
) {
    for ev in ev_power_input.iter() {
        if let Ok(mut assembly) = q_assembly_power.get_mut(ev.assembly) {
            if assembly.0.is_none() {
                assembly.0 = Some(ev.power);
            } else {
                match ev.power {
                    Power::Electrical(input_amount) => {
                        let input_power = assembly.0.unwrap();
                        match input_power {
                            Power::Electrical(existing) => {
                                assembly.0 = Some(Power::Electrical(existing + input_amount));
                            },
                            _ => { }
                        }
                    },
                    Power::Mechanical(input_amount) => {
                        let input_power = assembly.0.unwrap();
                        match input_power {
                            Power::Mechanical(existing) => {
                                assembly.0 = Some(Power::Mechanical(existing + input_amount));
                            },
                            _ => { }
                        }
                    },
                    Power::Thermal(input_amount) => {
                        let input_power = assembly.0.unwrap();
                        match input_power {
                            Power::Thermal(existing) => {
                                assembly.0 = Some(Power::Thermal(existing + input_amount));
                            },
                            _ => { }
                        }
                    }
                }
            }
        }
    }
}
