use crate::*;

#[derive(Component)]
pub struct PulpMill;

#[derive(Bundle)]
pub struct PulpMillBundle {
    pub marker: PulpMill,
    pub assembly: Assembly,
    pub input: AssemblyInput,
    pub output: AssemblyOutput,
    pub power: AssemblyPower,
    pub solid: SolidEntity,
    pub assembly_items: AssemblyItemContainer,
    pub sprite: SpriteBundle
}
impl Default for PulpMillBundle {
    fn default() -> PulpMillBundle {
        PulpMillBundle {
            marker: PulpMill,
            assembly: Assembly,
            input: AssemblyInput(Some(Item::Resource(ResourceItem::Wood))),
            output: AssemblyOutput(Some(Item::Resource(ResourceItem::Pulp))),
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
            solid: SolidEntity,
            sprite: SpriteBundle {
                ..AssemblyBundle::default().sprite
            }
        }
    }
}