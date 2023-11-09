use crate::*;

#[derive(Bundle)]
pub struct PulpMillBundle {
    pub assembly_type: AssemblyType,
    pub assembly: Assembly,
    pub input: AssemblyInput,
    pub output: AssemblyOutput,
    pub timer: AssemblyTimer,
    pub power: AssemblyPower,
    pub solid: SolidEntity,
    pub assembly_items: ItemIOContainer,
    pub sprite: SpriteBundle
}
impl Default for PulpMillBundle {
    fn default() -> PulpMillBundle {
        PulpMillBundle {
            assembly_type: AssemblyType::PulpMill,
            assembly: Assembly,
            input: AssemblyInput(Some(Item::Resource(ResourceItem::Wood))),
            output: AssemblyOutput(Some(Item::Resource(ResourceItem::Pulp))),
            timer: AssemblyTimer(Timer::from_seconds(15.0, TimerMode::Repeating)),
            power: AssemblyPower {
                current_power: Power::Mechanical(0.0),
                max_power: 100.0,
                power_cost: 10.0,
                powering_entities: Vec::new()
            },
            assembly_items: ItemIOContainer {
                input: ItemContainer {
                    items: Vec::new(),
                    max_items: 5
                },
                output: ItemContainer {
                    items: Vec::new(),
                    max_items: 3
                }
            },
            solid: SolidEntity,
            sprite: SpriteBundle {
                ..AssemblyBundle::default().sprite
            }
        }
    }
}

#[derive(Bundle)]
pub struct PaperPressBundle {
    pub assembly_type: AssemblyType,
    pub assembly: Assembly,
    pub input: AssemblyInput,
    pub output: AssemblyOutput,
    pub timer: AssemblyTimer,
    pub power: AssemblyPower,
    pub solid: SolidEntity,
    pub assembly_items: ItemIOContainer,
    pub sprite: SpriteBundle
}
impl Default for PaperPressBundle {
    fn default() -> PaperPressBundle {
        PaperPressBundle {
            assembly_type: AssemblyType::PaperPress,
            assembly: Assembly,
            input: AssemblyInput(Some(Item::Resource(ResourceItem::Pulp))),
            output: AssemblyOutput(Some(Item::Material(MaterialItem::WetPaper))),
            timer: AssemblyTimer(Timer::from_seconds(5.0, TimerMode::Repeating)),
            power: AssemblyPower {
                current_power: Power::Mechanical(0.0),
                max_power: 100.0,
                power_cost: 50.0,
                powering_entities: Vec::new()
            },
            assembly_items: ItemIOContainer {
                input: ItemContainer {
                    items: Vec::new(),
                    max_items: 5
                },
                output: ItemContainer {
                    items: Vec::new(),
                    max_items: 3
                }
            },
            solid: SolidEntity,
            sprite: SpriteBundle {
                ..AssemblyBundle::default().sprite
            }
        }
    }
}

#[derive(Bundle)]
pub struct PaperDrierBundle {
    pub assembly_type: AssemblyType,
    pub assembly: Assembly,
    pub input: AssemblyInput,
    pub output: AssemblyOutput,
    pub timer: AssemblyTimer,
    pub solid: SolidEntity,
    pub assembly_items: ItemIOContainer,
    pub sprite: SpriteBundle
}

impl Default for PaperDrierBundle {
    fn default() -> Self {
        PaperDrierBundle {
            assembly_type: AssemblyType::PaperDrier,
            assembly: Assembly,
            input: AssemblyInput(Some(Item::Material(MaterialItem::WetPaper))),
            output: AssemblyOutput(Some(Item::Good(GoodItem::Paper))),
            timer: AssemblyTimer(Timer::from_seconds(45.0, TimerMode::Repeating)),
            assembly_items: ItemIOContainer {
                input: ItemContainer {
                    items: Vec::new(),
                    max_items: 25
                },
                output: ItemContainer {
                    items: Vec::new(),
                    max_items: 2
                }
            },
            solid: SolidEntity,
            sprite: SpriteBundle {
                ..AssemblyBundle::default().sprite
            }
        }
    }
}
