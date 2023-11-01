use crate::*;

#[derive(Component, Debug, Reflect)]
pub struct AssemblyItemContainer {
    pub input: ItemContainer,
    pub output: ItemContainer,
}

#[derive(Component, Debug)]
pub struct AssemblyInput(pub Option<Item>);
impl Clickable for AssemblyInput {}

#[derive(Component, Debug)]
pub struct AssemblyOutput(pub Option<GoodItem>);
impl Clickable for AssemblyOutput {}

#[derive(Bundle)]
pub struct AssemblyInputBundle {
    pub marker: AssemblyInput,
    pub sprite: SpriteBundle
}

#[derive(Bundle)]
pub struct AssemblyOutputBundle {
    pub marker: AssemblyOutput,
    pub sprite: SpriteBundle
}
