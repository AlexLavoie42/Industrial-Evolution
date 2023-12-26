use crate::*;

#[derive(Component, Debug, Reflect)]
pub struct ItemContainer {
    pub items: Vec<Option<Entity>>,
    pub max_items: usize,
    pub item_type: Option<Item>,
    pub start_transform: Transform,
    pub width: i32,
}
impl Default for ItemContainer {
    fn default() -> Self {
        ItemContainer {
            items: Vec::new(),
            max_items: 1,
            item_type: None,
            start_transform: Transform::default(),
            width: 1,
        }

    }
}

pub struct ItemStackBundle {
    pub item: Item,
    pub items: ItemContainer,
    pub sprite: SpriteBundle
}

impl ItemContainer {
    pub fn add_item(&mut self, item: (Option<Entity>, Option<Item>)) -> Result<(), &'static str> {
        if self.items.len() >= self.max_items {
            return Err("Maximum number of items reached");
        }
        if let Some(item_type) = self.item_type {
            if Some(item_type) != item.1 {
                return Err("Invalid item type");
            }
        }
        Ok(self.items.push(item.0))
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

    pub fn get_transform(&self) -> Transform {
        let mut y = 0;
        let mut x = 0;
        for i in 1..self.items.len() {
            if i % self.width as usize == 0 {
                y += 1;
                x = 0;
            } else {
                x += 1;
            }
        }
        Transform::from_xyz(
            self.start_transform.translation.x + x as f32 * 16.0,
            self.start_transform.translation.y - y as f32 * 16.0,
            2.0
        )
        
    }

    pub fn get_transform_at_index(&self, index: usize) -> Transform {
        let mut y = 0;
        let mut x = 0;

        for i in 1..index+1 {
            if i % self.width as usize == 0 {
                y += 1;
                x = 0;
            } else {
                x += 1;
            }
        }
        Transform::from_xyz(
            self.start_transform.translation.x + x as f32 * 16.0,
            self.start_transform.translation.y - y as f32 * 16.0,
            2.0
        )
    }
}

pub fn move_container_items(
    mut q_containers: Query<&ItemContainer>,
    mut q_items: Query<&mut Transform, With<Item>>,
) {
    for container in q_containers.iter() {
        for (i, item) in container.items.iter().enumerate() {
            if let Some(entity) = item {
                let Ok(mut transform) = q_items.get_mut(*entity) else { continue };
                *transform = container.get_transform_at_index(i);
            }
        }
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
