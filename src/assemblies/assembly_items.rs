use crate::*;

#[derive(Component)]
pub struct AssemblyItemContainer {
    pub input: ItemContainer,
    pub output: ItemContainer,
}

#[derive(Component)]
pub struct AssemblyInput(pub Option<Item>);
impl Clickable for AssemblyInput {}

#[derive(Component)]
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
