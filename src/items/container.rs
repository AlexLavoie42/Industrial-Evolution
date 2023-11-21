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

impl ContainerInputSelectorBundle {
    pub fn new(asset_server: AssetServer) -> Self {
        ContainerInputSelectorBundle {
            marker: ContainerInputSelector,
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, -42.0, 1.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32.0, 64.0)),
                    ..Default::default()
                },
                texture: asset_server.load("Input Arrow.png"),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct ContainerOutputSelectorBundle {
    pub marker: ContainerOutputSelector,
    pub sprite: SpriteBundle
}

impl ContainerOutputSelectorBundle {
    pub fn new(asset_server: AssetServer) -> Self {
        ContainerOutputSelectorBundle {
            marker: ContainerOutputSelector,
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 42.0, 1.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(32.0, 64.0)),
                    ..Default::default()
                },
                texture: asset_server.load("Output Arrow.png"),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
        }
    }
}

pub fn toggle_container_selectors(
    mut q_input_selector: Query<&mut Visibility, (With<ContainerInputSelector>, Without<ContainerOutputSelector>)>,
    mut q_output_selector: Query<&mut Visibility, (With<ContainerOutputSelector>, Without<ContainerInputSelector>)>,
) {
    for mut selector in q_input_selector.iter_mut() {
        if *selector == Visibility::Hidden {
            *selector = Visibility::Visible;
        } else {
            *selector = Visibility::Hidden;
        }
    }
    for mut selector in q_output_selector.iter_mut() {
        if *selector == Visibility::Hidden {
            *selector = Visibility::Visible;
        } else {
            *selector = Visibility::Hidden;
        }
    }
}
