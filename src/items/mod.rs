use std::marker::PhantomData;

use bevy::ecs::system::EntityCommands;

use crate::*;

mod resources;
pub use resources::*;

mod goods;
pub use goods::*;

mod container;
pub use container::*;

mod imports;
pub use imports::*;

mod exports;
pub use exports::*;

mod materials;
pub use materials::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayerState::Imports),
                |mut ev_show_ghost: EventWriter<ShowHoverGhost<ItemImportBundle>>| {
                    ev_show_ghost.send(ShowHoverGhost::<ItemImportBundle> {
                        bundle: PhantomData::<ItemImportBundle>
                    });
                }
            )
            .add_systems(OnExit(PlayerState::Imports),
                |mut ev_hide_ghost: EventWriter<HideHoverGhost>| {
                    ev_hide_ghost.send(HideHoverGhost);
                }
            )
            .add_systems(Update, show_hover_ghost::<ItemImportBundle>)
            .add_event::<ShowHoverGhost::<ItemImportBundle>>()
            .add_systems(OnEnter(PlayerState::Export),
                |mut ev_show_ghost: EventWriter<ShowHoverGhost<ItemExportBundle>>| {
                    ev_show_ghost.send(ShowHoverGhost::<ItemExportBundle> {
                        bundle: PhantomData::<ItemExportBundle>
                    });
                }
            )
            .add_systems(OnExit(PlayerState::Export),
                |mut ev_hide_ghost: EventWriter<HideHoverGhost>| {
                    ev_hide_ghost.send(HideHoverGhost);
                }
            )
            .add_systems(OnEnter(PlayerState::Jobs), toggle_container_selectors)
            .add_systems(OnExit(PlayerState::Jobs), toggle_container_selectors)
            .add_event::<ShowHoverGhost::<ItemExportBundle>>()
            .add_systems(Update, show_hover_ghost::<ItemExportBundle>)
            .add_systems(PreUpdate, mouse_collision_system::<Item>)
            // .add_systems(Update, (
            //     place_import.run_if(in_state(PlayerState::Imports)),
            //     input_toggle_import_mode
            // ).run_if(in_state(DayCycleState::Day)))
            .add_systems(OnEnter(DayCycleState::Night), (sell_export_items, item_imports_storage_fee))
            .add_systems(OnExit(DayCycleState::Night), (purchase_item_imports, |mut sold_items: ResMut<SoldItems>| sold_items.items.clear()))
            .insert_resource(SoldItems::default())
            // .add_systems(Update, (
            //     place_export.run_if(in_state(PlayerState::Export)),
            //     input_toggle_export_mode
            // ).run_if(in_state(DayCycleState::Day)))
            .add_event::<GenericMouseCollisionEvent<Item>>()
            .register_type::<ItemContainer>()
        ;
    }
}

#[derive(Component, PartialEq, Debug, Reflect, Eq, Hash, Clone, Copy)]
pub enum Item {
    Good(GoodItem),
    Resource(ResourceItem),
    Material(MaterialItem)
}

impl ItemType for Item {
    fn get_name (&self) -> &str {
        match self {
            Item::Good(good) => good.get_name(),
            Item::Resource(resource) => resource.get_name(),
            Item::Material(material) => material.get_name(),
        }
    }
}

impl<'a, 'w, 's> ItemSpawn<'a, 'w, 's> for Item {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>
    ) -> EntityCommands<'w, 's, 'a> {
        match self {
            Item::Good(good) => {
                good.spawn_bundle(commands)
            },
            Item::Resource(resource) => {
                resource.spawn_bundle(commands)
            },
            Item::Material(material) => {
                material.spawn_bundle(commands)
            }
        }
    }
}

impl Clickable for Item {}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub sprite: SpriteBundle
}

pub trait ItemType {
    fn get_name (&self) -> &str;
}

pub trait ItemSpawn<'a, 'w, 's>: Component {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>
    ) -> EntityCommands<'w, 's, 'a>;
}
