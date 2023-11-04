use crate::*;

#[derive(Component, Debug, Reflect)]
pub struct AssemblyItemContainer {
    pub input: ItemContainer,
    pub output: ItemContainer,
}

#[derive(Component, Debug)]
pub struct AssemblyInput(pub Option<Item>);

#[derive(Component, Debug)]
pub struct AssemblyInputSelector;
impl Clickable for AssemblyInputSelector {}

#[derive(Component, Debug)]
pub struct AssemblyOutput(pub Option<Item>);

#[derive(Component, Debug)]
pub struct AssemblyOutputSelector;
impl Clickable for AssemblyOutputSelector {}

#[derive(Bundle)]
pub struct AssemblyInputSelectorBundle {
    pub marker: AssemblyInputSelector,
    pub sprite: SpriteBundle
}

#[derive(Bundle)]
pub struct AssemblyOutputSelectorBundle {
    pub marker: AssemblyOutputSelector,
    pub sprite: SpriteBundle
}
