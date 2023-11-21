use std::marker::PhantomData;

use bevy::ecs::system::EntityCommands;

use crate::*;

mod resources;
pub use resources::*;

mod goods;
pub use goods::*;

mod container;
pub use container::*;

mod receivables;
pub use receivables::*;

mod trade_depot;
pub use trade_depot::*;

mod materials;
pub use materials::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayerState::Recievables),
                |mut ev_show_ghost: EventWriter<ShowHoverGhost<ItemReceivableBundle>>| {
                    ev_show_ghost.send(ShowHoverGhost::<ItemReceivableBundle> {
                        bundle: PhantomData::<ItemReceivableBundle>
                    });
                }
            )
            .add_systems(OnExit(PlayerState::Recievables),
                |mut ev_hide_ghost: EventWriter<HideHoverGhost>| {
                    ev_hide_ghost.send(HideHoverGhost);
                }
            )
            .add_systems(Update, show_hover_ghost::<ItemReceivableBundle>)
            .add_event::<ShowHoverGhost::<ItemReceivableBundle>>()
            .add_systems(OnEnter(PlayerState::TradeDepot),
                |mut ev_show_ghost: EventWriter<ShowHoverGhost<TradeDepotBundle>>| {
                    ev_show_ghost.send(ShowHoverGhost::<TradeDepotBundle> {
                        bundle: PhantomData::<TradeDepotBundle>
                    });
                }
            )
            .add_systems(OnExit(PlayerState::TradeDepot),
                |mut ev_hide_ghost: EventWriter<HideHoverGhost>| {
                    ev_hide_ghost.send(HideHoverGhost);
                }
            )
            .add_systems(OnEnter(PlayerState::Jobs), toggle_container_selectors)
            .add_systems(OnExit(PlayerState::Jobs), toggle_container_selectors)
            .add_event::<ShowHoverGhost::<TradeDepotBundle>>()
            .add_systems(Update, show_hover_ghost::<TradeDepotBundle>)
            .add_systems(PreUpdate, mouse_collision_system::<Item>)
            .add_systems(Update, purchase_receivables)
            .add_systems(Update, (
                place_receivable.run_if(in_state(PlayerState::Recievables)),
                input_toggle_receivable_mode
            ))
            .add_systems(Update, sell_trade_depot_items)
            .add_systems(Update, (
                place_trade_depot.run_if(in_state(PlayerState::TradeDepot)),
                input_toggle_trade_depot_mode
            ))
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

impl ItemType for Item {}

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

pub trait ItemType {}

pub trait ItemSpawn<'a, 'w, 's>: Component {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>
    ) -> EntityCommands<'w, 's, 'a>;
}
