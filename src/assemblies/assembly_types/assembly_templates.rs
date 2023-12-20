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

make_assembly_bundle!(PulpMillBundle,
    power: AssemblyPower,
    input: AssemblyInput,
    output: AssemblyOutput,
    timer: AssemblyTimer
);
impl DefaultWithSprites for PulpMillBundle {
    fn default_with_sprites(sprites: &SpriteStorage) -> PulpMillBundle {
        PulpMillBundle {
            assembly_type: AssemblyType::PulpMill,
            assembly: Assembly,
            input: AssemblyInput(Some(Item::Resource(ResourceItem::Wood))),
            output: AssemblyOutput(Some(Item::Resource(ResourceItem::Pulp))),
            timer: AssemblyTimer {
                timer: Timer::from_seconds(15.0, TimerMode::Repeating),
                item: None
            },
            power: AssemblyPower {
                current_power: Power::Mechanical(0.0),
                max_power: 100.0,
                power_cost: 10.0,
                powering_entities: Vec::new()
            },
            assembly_items: ItemIOContainer {
                input: ItemContainer {
                    items: Vec::new(),
                    item_type: Some(Item::Resource(ResourceItem::Wood)),
                    max_items: 5
                },
                output: ItemContainer {
                    items: Vec::new(),
                    item_type: None,
                    max_items: 3
                }
            },
            solid: SolidEntity,
            tile_size: EntityTileSize(IVec2::new(4, 4)),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..Default::default()
                },
                texture: sprites.pulp_mill.clone(),
                ..AssemblyBundle::default().sprite
            }
        }
    }
}

make_assembly_bundle!(PaperPressBundle, 
    power: AssemblyPower,
    input: AssemblyInput,
    output: AssemblyOutput,
    timer: AssemblyTimer
);
impl DefaultWithSprites for PaperPressBundle {
    fn default_with_sprites(sprites: &SpriteStorage) -> PaperPressBundle {
        PaperPressBundle {
            assembly_type: AssemblyType::PaperPress,
            assembly: Assembly,
            input: AssemblyInput(Some(Item::Resource(ResourceItem::Pulp))),
            output: AssemblyOutput(Some(Item::Material(MaterialItem::WetPaper))),
            timer: AssemblyTimer {
                timer: Timer::from_seconds(5.0, TimerMode::Repeating),
                item: None
            },
            power: AssemblyPower {
                current_power: Power::Mechanical(0.0),
                max_power: 100.0,
                power_cost: 50.0,
                powering_entities: Vec::new()
            },
            assembly_items: ItemIOContainer {
                input: ItemContainer {
                    items: Vec::new(),
                    item_type: Some(Item::Resource(ResourceItem::Pulp)),
                    max_items: 5
                },
                output: ItemContainer {
                    items: Vec::new(),
                    item_type: None,
                    max_items: 3
                }
            },
            solid: SolidEntity,
            tile_size: EntityTileSize(IVec2::new(2, 2)),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                texture: sprites.paper_press.clone(),
                ..AssemblyBundle::default().sprite
            }
        }
    }
}

make_assembly_bundle!(PaperDrierBundle,
    input: AssemblyInput,
    output: AssemblyOutput,
    timer: AssemblyTimer
);
impl DefaultWithSprites for PaperDrierBundle {
    fn default_with_sprites(sprites: &SpriteStorage) -> Self {
        PaperDrierBundle {
            assembly_type: AssemblyType::PaperDrier,
            assembly: Assembly,
            input: AssemblyInput(Some(Item::Material(MaterialItem::WetPaper))),
            output: AssemblyOutput(Some(Item::Good(GoodItem::Paper))),
            timer: AssemblyTimer {
                timer: Timer::from_seconds(45.0, TimerMode::Repeating),
                item: None
            },
            assembly_items: ItemIOContainer {
                input: ItemContainer {
                    items: Vec::new(),
                    item_type: Some(Item::Material(MaterialItem::WetPaper)),
                    max_items: 25
                },
                output: ItemContainer {
                    items: Vec::new(),
                    item_type: None,
                    max_items: 2
                }
            },
            solid: SolidEntity,
            tile_size: EntityTileSize(IVec2::new(2, 2)),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                texture: sprites.paper_drier.clone(),
                ..AssemblyBundle::default().sprite
            }
        }
    }
}
