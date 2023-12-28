use std::ops::Range;

use bevy::utils::HashMap;
use bevy_inspector_egui::{InspectorOptions, inspector_options::ReflectInspectorOptions};

use crate::*;

use rand::{thread_rng, Rng};

mod upkeep;
pub use upkeep::*;

// TODO: Per item
const MARKET_FORCE: f32 = 1.25;
const PRICE_INCREASE_MULT: Range<f32> = 1.01..1.03;
const PRICE_DECREASE_MULT: Range<f32> = 0.97..0.995;

const MARKET_SELL_PERCENTAGE: f32 = 0.45;

pub struct MoneyPlugin;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(DayCycleState::Night), (market_forces, market_system))
            .add_systems(OnEnter(DayCycleState::Night), (factory_upkeep, living_expenses))
            .add_systems(OnEnter(DayCycleState::Day), upkeep_system)
            .insert_resource(PlayerMoney {
                amount: 600.0
            })
            .insert_resource(MarketTimer::default())
            .insert_resource(Economy::default())
            .insert_resource(AssemblyPrices::default())
            .insert_resource(UpkeepTimer::default())
            .insert_resource(UpkeepTracker::new())
            .register_type::<PlayerMoney>()
            .register_type::<Economy>()
        ;
    }
}

#[derive(Resource, Reflect)]
pub struct PlayerMoney {
    pub amount: f32
}
impl Money for PlayerMoney {
    fn add_money(&mut self, amount: f32) {
        self.amount += amount;
    }
    fn try_remove_money(&mut self, amount: f32) -> Result<(), &str> {
        if self.amount >= amount {
            self.amount -= amount;
            return Ok(());
        }

        Err("Not enough money")
    }
}

pub trait Money {
    fn add_money(&mut self, amount: f32);
    fn try_remove_money(&mut self, amount: f32) -> Result<(), &str>;
}

#[derive(Reflect)]
pub struct EconomyPrice {
    pub current_price: f32,
    // Price that is considered baseline and should fluctuate around this value
    pub base_price: f32,
    // How much supply is considered baseline
    pub base_supply: f32,
    // How much is currently available to purchase. When this is higher than base price will trend down
    pub supply: f32,
    // How much demand is considered baseline
    pub base_demand: f32,
    // How much is currently demanded. When this is lower than base price will trend up
    pub demand: f32,
    // How often should more demand be added
    pub demand_weight: f32,
    // How often should more supply be added
    pub supply_weight: f32
}

#[derive(Resource, Reflect)]
pub struct AssemblyPrices {
    pub prices: HashMap<AssemblyType, f32>
}

impl Default for AssemblyPrices {
    fn default() -> Self {
        Self {
            prices: HashMap::from([
                (AssemblyType::WoodChipper, 50.0),
                (AssemblyType::PulpMachine, 150.0),
                (AssemblyType::PaperMachine, 100.0)
            ])
        }
    }
}

#[derive(Reflect, Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum PurchasableItem {
    Good(GoodItem),
    Resource(ResourceItem)
}

impl PurchasableItem {
    pub fn get_price(&self, economy: &Economy) -> Option<f32> {
        economy.prices.get(self).map(|x| { x.current_price })
    }
    pub fn get_supply(&self, economy: &Economy) -> Option<f32> {
        economy.prices.get(self).map(|x| { x.supply })
    }
    pub fn get_demand(&self, economy: &Economy) -> Option<f32> {
        economy.prices.get(self).map(|x| { x.demand })
    }
    pub fn get_name(&self) -> &str {
        match self {
            PurchasableItem::Good(x) => x.get_name(),
            PurchasableItem::Resource(x) => x.get_name()
        }
    }
}

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Economy {
    pub prices: HashMap<PurchasableItem, EconomyPrice>
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            prices: HashMap::from([
                (PurchasableItem::Resource(ResourceItem::Wood), EconomyPrice {
                    current_price: 0.75,
                    base_price: 0.75,
                    base_supply: 50.0,
                    supply: 50.0,
                    base_demand: 1.0,
                    demand: 1.0,
                    demand_weight: 0.8,
                    supply_weight: 1.4
                }),
                (PurchasableItem::Resource(ResourceItem::WoodChips), EconomyPrice {
                    current_price: 4.0,
                    base_price: 2.75,
                    base_supply: 3.0,
                    supply: 4.0,
                    base_demand: 4.0,
                    demand: 10.0,
                    demand_weight: 0.9,
                    supply_weight: 1.0
                }),
                (PurchasableItem::Good(GoodItem::Paper), EconomyPrice {
                    current_price: 8.5,
                    base_price: 12.75,
                    base_supply: 0.0,
                    supply: 0.0,
                    base_demand: 40.0,
                    demand: 40.0,
                    demand_weight: 1.15,
                    supply_weight: 0.85
                }),
                (PurchasableItem::Resource(ResourceItem::Lumber), EconomyPrice {
                    current_price: 5.0,
                    base_price: 3.75,
                    base_supply: 3.0,
                    supply: 6.0,
                    base_demand: 16.0,
                    demand: 30.0,
                    demand_weight: 1.0,
                    supply_weight: 1.0
                })
            ])
        }
    }
}

pub trait Purchasable {
    fn get_price(&self, economy: &Economy) -> Option<f32>;
    fn buy(&mut self, economy: &mut Economy, amount: i32) -> Result<(), &'static str>;
    fn sell(&mut self, economy: &mut Economy, amount: i32) -> Result<(), &'static str>;
    fn get_supply(&self, economy: &Economy) -> Option<f32>;
    fn get_demand(&self, economy: &Economy) -> Option<f32>;
}

impl Purchasable for Item {
    fn get_price(&self, economy: &Economy) -> Option<f32> {
        let purchasable = match self {
            Item::Good(good) => Some(PurchasableItem::Good(*good)),
            Item::Resource(resource) => Some(PurchasableItem::Resource(*resource)),
            Item::Material(material) => None
        };
        let Some(purchasable) = purchasable else { return None; };
        economy.prices.get(&purchasable).map(|x| { x.current_price })
    }
    fn buy(&mut self, economy: &mut Economy, amount: i32) -> Result<(), &'static str> {
        let purchasable = match self {
            Item::Good(good) => Some(PurchasableItem::Good(*good)),
            Item::Resource(resource) => Some(PurchasableItem::Resource(*resource)),
            Item::Material(material) => None
        };
        let Some(purchasable) = purchasable else { return Err("Item not purchasable"); };
        let Some(price) = economy.prices.get_mut(&purchasable) else { return Err("Item not purchasable"); };
        if (price.supply as i32) < amount {
            return Err("Not enough supply");
        }
        price.supply -= amount as f32;

        Ok(())
    }
    fn sell(&mut self, economy: &mut Economy, amount: i32) -> Result<(), &'static str> {
        let purchasable = match self {
            Item::Good(good) => Some(PurchasableItem::Good(*good)),
            Item::Resource(resource) => Some(PurchasableItem::Resource(*resource)),
            Item::Material(material) => None
        };
        let Some(purchasable) = purchasable else { return Err("Item not purchasable"); };
        let Some(price) = economy.prices.get_mut(&purchasable) else { return Err("Item not purchasable"); };
        if (price.demand as i32) < amount {
            return Err("Not enough demand");
        }

        price.supply += amount as f32;

        Ok(())
    }
    fn get_supply(&self, economy: &Economy) -> Option<f32> {
        let purchasable = match self {
            Item::Good(good) => Some(PurchasableItem::Good(*good)),
            Item::Resource(resource) => Some(PurchasableItem::Resource(*resource)),
            Item::Material(material) => None
        };
        let Some(purchasable) = purchasable else { return None; };
        let Some(price) = economy.prices.get(&purchasable) else { return None; };

        Some(price.supply)
    }
    fn get_demand(&self, economy: &Economy) -> Option<f32> {
        let purchasable = match self {
            Item::Good(good) => Some(PurchasableItem::Good(*good)),
            Item::Resource(resource) => Some(PurchasableItem::Resource(*resource)),
            Item::Material(_) => None
        };
        let Some(purchasable) = purchasable else { return None; };
        let Some(price) = economy.prices.get(&purchasable) else { return None; };

        Some(price.demand)
    }
}

#[derive(Resource)]
struct MarketTimer(Timer);
impl Default for MarketTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, TimerMode::Repeating))
    }
}

fn market_system(
    mut economy: ResMut<Economy>,
    time: Res<Time>,
    mut market_timer: ResMut<MarketTimer>,
) {
    let mut rng = thread_rng();
    for (item, price) in economy.prices.iter_mut() {
        let supply = price.supply - price.base_supply;
        let demand = price.demand - price.base_demand;
        let price_gap = price.current_price / price.base_price;
        let supply_gap = demand / supply;
        if item == &PurchasableItem::Good(GoodItem::Paper) {
            println!("{} {} {} {}", supply, demand, price_gap, supply_gap);
        }

        if price_gap < supply_gap {
            price.current_price *= rng.gen_range(PRICE_INCREASE_MULT);
        } else if price_gap > supply_gap {
            price.current_price *= rng.gen_range(PRICE_DECREASE_MULT);
        }
    }
}

fn market_forces(
    mut economy: ResMut<Economy>,
    time: Res<Time>,
    mut market_timer: ResMut<MarketTimer>,
) {
    let mut rng = thread_rng();
    for (item, price) in economy.prices.iter_mut() {
        let price_gap = (price.current_price / price.base_price).max(price.base_price / price.current_price);

        let supply = price.supply - price.base_supply;
        let demand = price.demand - price.base_demand;
        // TODO: Min max values for supply & demand

        let weighted_supply = supply * price.demand_weight;
        let weighted_demand = demand * price.supply_weight;
        if weighted_supply >= weighted_demand {
            price.demand += MARKET_FORCE * price_gap * price.demand_weight;
        }
        if weighted_supply < weighted_demand {
            price.supply += MARKET_FORCE * price_gap * price.supply_weight;
        }
        
        if price.supply > 1.0 && price.demand > 1.0 {
            let sold = (supply).min(price.demand * MARKET_SELL_PERCENTAGE).floor();
            price.supply -= sold;
            price.demand -= sold;
        }
    }
}
