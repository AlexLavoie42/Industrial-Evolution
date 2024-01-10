use crate::*;
macro_rules! make_assembly_bundle {
    ($assembly_name:ident) => {
        #[derive(Bundle)]
        pub struct $assembly_name {
            pub assembly_type: AssemblyType,
            pub assembly: Assembly,
            pub solid: SolidEntity,
            pub tile_size: EntityTileSize,
            pub assembly_items: ItemIOContainer,
            pub sprite: SpriteBundle
        }
        impl GetGhostBundle for $assembly_name {
            fn get_sprite_bundle(&self) -> SpriteBundle {
                self.sprite.clone()
            }
            fn get_tile_size(&self) -> Option<EntityTileSize> {
                return Some(self.tile_size);
            }
        }
    };

    ($assembly_name:ident, $($extra_field:ident: $extra_type:ty),*) => {
        #[derive(Bundle)]
        pub struct $assembly_name {
            pub assembly_type: AssemblyType,
            pub assembly: Assembly,
            pub solid: SolidEntity,
            pub tile_size: EntityTileSize,
            pub assembly_items: ItemIOContainer,
            pub sprite: SpriteBundle,
            $(pub $extra_field: $extra_type,)*
        }
        impl GetGhostBundle for $assembly_name {
            fn get_sprite_bundle(&self) -> Option<SpriteBundle> {
                Some(self.sprite.clone())
            }
            fn get_tile_size(&self) -> Option<EntityTileSize> {
                return Some(self.tile_size);
            }
        }
    }
}

make_assembly_bundle!(WoodChipperBundle,
    power: AssemblyPower,
    input: AssemblyInput,
    output: AssemblyOutput,
    timer: AssemblyTimer
);
impl DefaultWithSprites for WoodChipperBundle {
    fn default_with_sprites(sprites: &SpriteStorage) -> WoodChipperBundle {
        WoodChipperBundle {
            assembly_type: AssemblyType::WoodChipper,
            assembly: Assembly,
            input: AssemblyInput(Some(Item::Resource(ResourceItem::Wood))),
            output: AssemblyOutput(Some(Item::Resource(ResourceItem::WoodChips))),
            timer: AssemblyTimer {
                timer: Timer::from_seconds(15.0, TimerMode::Repeating),
                item: None
            },
            power: AssemblyPower {
                current_power: Power::Mechanical(0.0),
                max_power: 45.0,
                power_cost: 16.0,
                powering_entities: Vec::new()
            },
            assembly_items: ItemIOContainer {
                input: ItemContainer {
                    items: Vec::new(),
                    item_type: Some(Item::Resource(ResourceItem::Wood)),
                    max_items: 4,
                    start_transform: Transform::from_xyz(-26.0, 26.0, 2.0),
                    width: 4,
                    ..Default::default()
                },
                output: ItemContainer {
                    items: Vec::new(),
                    item_type: None,
                    max_items: 4,
                    start_transform: Transform::from_xyz(-26.0, -26.0, 2.0),
                    width: 4,
                    ..Default::default()
                }
            },
            solid: SolidEntity,
            tile_size: EntityTileSize(IVec2::new(4, 4)),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..Default::default()
                },
                texture: sprites.wood_chipper.clone(),
                ..AssemblyBundle::default().sprite
            }
        }
    }
}

make_assembly_bundle!(PulpMachineBundle, 
    power: AssemblyPower,
    input: AssemblyInput,
    output: AssemblyOutput,
    timer: AssemblyTimer
);
impl DefaultWithSprites for PulpMachineBundle {
    fn default_with_sprites(sprites: &SpriteStorage) -> PulpMachineBundle {
        PulpMachineBundle {
            assembly_type: AssemblyType::PulpMachine,
            assembly: Assembly,
            input: AssemblyInput(Some(Item::Resource(ResourceItem::WoodChips))),
            output: AssemblyOutput(Some(Item::Material(MaterialItem::WoodPulp))),
            timer: AssemblyTimer {
                timer: Timer::from_seconds(9.0, TimerMode::Repeating),
                item: None
            },
            power: AssemblyPower {
                current_power: Power::Mechanical(0.0),
                max_power: 100.0,
                power_cost: 25.0,
                powering_entities: Vec::new()
            },
            assembly_items: ItemIOContainer {
                input: ItemContainer {
                    items: Vec::new(),
                    item_type: Some(Item::Resource(ResourceItem::WoodChips)),
                    max_items: 4,
                    start_transform: Transform::from_xyz(-26.0, 26.0, 2.0),
                    width: 4,
                    ..Default::default()
                },
                output: ItemContainer {
                    items: Vec::new(),
                    item_type: None,
                    max_items: 4,
                    start_transform: Transform::from_xyz(-26.0, 0.0, 2.0),
                    width: 4,
                    ..Default::default()
                }
            },
            solid: SolidEntity,
            tile_size: EntityTileSize(IVec2::new(4, 4)),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                texture: sprites.pulp_machine.clone(),
                ..AssemblyBundle::default().sprite
            }
        }
    }
}

make_assembly_bundle!(PaperMachineBundle,
    input: AssemblyInput,
    output: AssemblyOutput,
    power: AssemblyPower,
    timer: AssemblyTimer
);
impl DefaultWithSprites for PaperMachineBundle {
    fn default_with_sprites(sprites: &SpriteStorage) -> Self {
        PaperMachineBundle {
            assembly_type: AssemblyType::PaperMachine,
            assembly: Assembly,
            input: AssemblyInput(Some(Item::Material(MaterialItem::WoodPulp))),
            output: AssemblyOutput(Some(Item::Good(GoodItem::Paper))),
            power: AssemblyPower {
                current_power: Power::Mechanical(0.0),
                max_power: 150.0,
                power_cost: 45.0,
                powering_entities: Vec::new()
            },
            timer: AssemblyTimer {
                timer: Timer::from_seconds(25.0, TimerMode::Repeating),
                item: None
            },
            assembly_items: ItemIOContainer {
                input: ItemContainer {
                    items: Vec::new(),
                    item_type: Some(Item::Material(MaterialItem::WoodPulp)),
                    max_items: 25,
                    start_transform: Transform::from_xyz(-26.0, 26.0, 2.0),
                    width: 2,
                    ..Default::default()
                },
                output: ItemContainer {
                    items: Vec::new(),
                    item_type: None,
                    max_items: 4,
                    start_transform: Transform::from_xyz(-26.0, -26.0, 2.0),
                    width: 4,
                    ..Default::default()
                }
            },
            solid: SolidEntity,
            tile_size: EntityTileSize(IVec2::new(4, 4)),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                texture: sprites.paper_machine.clone(),
                ..AssemblyBundle::default().sprite
            }
        }
    }
}

make_assembly_bundle!(SawMillBundle,
    input: AssemblyInput,
    output: AssemblyOutput,
    timer: AssemblyTimer,
    power: AssemblyPower
);
impl DefaultWithSprites for SawMillBundle {
    fn default_with_sprites(sprites: &SpriteStorage) -> Self {
        SawMillBundle {
            assembly_type: AssemblyType::SawMill,
            assembly: Assembly,
            input: AssemblyInput(Some(Item::Resource(ResourceItem::Wood))),
            output: AssemblyOutput(Some(Item::Resource(ResourceItem::Lumber))),
            timer: AssemblyTimer {
                timer: Timer::from_seconds(5.0, TimerMode::Repeating),
                item: None
            },
            power: AssemblyPower {
                current_power: Power::Mechanical(0.0),
                max_power: 45.0,
                power_cost: 10.0,
                powering_entities: Vec::new()
            },
            assembly_items: ItemIOContainer {
                input: ItemContainer {
                    items: Vec::new(),
                    item_type: Some(Item::Resource(ResourceItem::Wood)),
                    max_items: 4,
                    start_transform: Transform::from_xyz(-26.0, 26.0, 2.0),
                    width: 4,
                    ..Default::default()
                },
                output: ItemContainer {
                    items: Vec::new(),
                    item_type: None,
                    max_items: 4,
                    start_transform: Transform::from_xyz(-26.0, 0.0, 2.0),
                    width: 4,
                    ..Default::default()
                }
            },
            solid: SolidEntity,
            tile_size: EntityTileSize(IVec2::new(4, 4)),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                texture: sprites.saw_mill.clone(),
                ..AssemblyBundle::default().sprite
            }
        }
    }
}
