use crate::*;

#[derive(Component, Debug)]
pub struct AssemblyInput(pub Option<Item>);

#[derive(Component, Debug)]
pub struct AssemblyOutput(pub Option<Item>);

#[derive(Component, Debug)]
pub struct AssemblyTimer(pub Timer);
