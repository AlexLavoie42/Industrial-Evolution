use crate::*;

#[derive(Component, Debug, Reflect)]
pub struct ItemContainer {
    pub items: Vec<Option<Entity>>,
    pub max_items: usize,
}

pub struct ItemStackBundle {
    pub item: Item,
    pub items: ItemContainer,
    pub sprite: SpriteBundle
}

impl ItemContainer {
    pub fn add_item(&mut self, item: Option<Entity>) -> Result<(), &'static str> {
        if self.items.len() >= self.max_items {
            return Err("Maximum number of items reached");
        }
        Ok(self.items.push(item))
    }

    pub fn remove_item(&mut self, item: Option<Entity>) -> Result<Option<Entity>, &'static str> {
        let item_i = self.items.iter().position(|&x| x == item);
        if let Some(index) = item_i {
            if index >= self.items.len() {
                println!("invalid index");
                return Err("Invalid index");
            }
            let item = Ok(self.items.remove(index));
            return item;
        } else {
            println!("Item not found");
            return Err("Item not found");
        }
    }

    pub fn remove_index(&mut self, index: usize) -> Result<Option<Entity>, &'static str> {
        if index >= self.items.len() {
            return Err("Invalid index");
        }

        Ok(self.items.remove(index))
    }

    pub fn get_items(&self) -> &[Option<Entity>] {
        &self.items
    }
}

#[derive(Component, Debug, Reflect)]
pub struct ItemIOContainer {
    pub input: ItemContainer,
    pub output: ItemContainer,
}

#[derive(Component, Debug)]
pub struct ContainerInputSelector;
impl Clickable for ContainerInputSelector {}

#[derive(Component, Debug)]
pub struct ContainerOutputSelector;
impl Clickable for ContainerOutputSelector {}

#[derive(Bundle)]
pub struct ContainerInputSelectorBundle {
    pub marker: ContainerInputSelector,
    pub sprite: SpriteBundle
}

#[derive(Bundle)]
pub struct ContainerOutputSelectorBundle {
    pub marker: ContainerOutputSelector,
    pub sprite: SpriteBundle
}
