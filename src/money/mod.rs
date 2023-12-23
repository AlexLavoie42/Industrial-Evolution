use bevy::utils::HashMap;

use crate::*;

use rand::{thread_rng, Rng};

mod upkeep;
pub use upkeep::*;

pub struct MoneyPlugin;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(DayCycleState::Night), (market_forces, market_system))
            .add_systems(OnEnter(DayCycleState::Night), (factory_upkeep, living_expenses))
            .add_systems(OnEnter(DayCycleState::Day), upkeep_system)
            .insert_resource(PlayerMoney {
                amount: 2500.0
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
    pub base_price: f32,
    pub supply: f32,
    pub demand: f32,
    pub demand_weight: f32,
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
                (AssemblyType::PulpMill, 500.0),
                (AssemblyType::PaperPress, 1000.0)
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
    pub fn get_name(&self) -> &str { 
        match self {
            PurchasableItem::Good(x) => x.get_name(),
            PurchasableItem::Resource(x) => x.get_name()
        }
    }
}

#[derive(Resource, Reflect)]
pub struct Economy {
    pub prices: HashMap<PurchasableItem, EconomyPrice>
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            prices: HashMap::from([
                (PurchasableItem::Resource(ResourceItem::Wood), EconomyPrice { current_price: 3.0, base_price: 3.0, supply: 50.0, demand: 1.0, demand_weight: 0.8, supply_weight: 1.4 }),
                (PurchasableItem::Resource(ResourceItem::Pulp), EconomyPrice { current_price: 5.0, base_price: 5.0, supply: 0.0, demand: 10.0, demand_weight: 1.1, supply_weight: 0.9 }),
                (PurchasableItem::Good(GoodItem::Paper), EconomyPrice { current_price: 10.0, base_price: 10.0, supply: 0.0, demand: 40.0,  demand_weight: 1.35, supply_weight: 0.75 }),
            ])
        }
    }
}

pub trait Purchasable {
    fn get_price(&self, economy: &Economy) -> Option<f32>;
    fn buy(&mut self, economy: &mut Economy, amount: i32) -> Result<(), &'static str>;
    fn sell(&mut self, economy: &mut Economy, amount: i32) -> Result<(), &'static str>;
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
        price.demand += amount as f32;

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
        price.demand -= amount as f32;

        Ok(())
    }
}

#[derive(Resource)]
struct MarketTimer(Timer);
impl Default for MarketTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, TimerMode::Repeating))
    }
}

const MARKET_FORCE: f32 = 2.0;
const PRICE_INCREASE_MULT: f32 = 1.1;
const PRICE_DECREASE_MULT: f32 = 0.9;
fn market_system(
    mut economy: ResMut<Economy>,
    time: Res<Time>,
    mut market_timer: ResMut<MarketTimer>,
) {
    let mut rng = thread_rng();
    for (item, price) in economy.prices.iter_mut() {
        let price_gap = price.current_price / price.base_price;
        let supply_gap = price.demand / price.supply;

        if price_gap < supply_gap {
            price.current_price *= PRICE_INCREASE_MULT * rng.gen_range(1.0..1.2);
        } else if price_gap > supply_gap {
            price.current_price *= PRICE_DECREASE_MULT * rng.gen_range(1.0..1.2);
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
        let price_gap = price.current_price / price.base_price;
        if price.supply == 0.0 {
            price.supply += MARKET_FORCE * price_gap;
        }
        if price.demand == 0.0 {
            price.demand += MARKET_FORCE * price_gap;
        }
        if price.demand > 1000.0 && price.supply > 1000.0 {
            price.demand /= 100.0;
            price.supply /= 100.0;

            price.demand = price.demand.round();
            price.supply = price.supply.round();
        }
        let weighted_supply = rng.gen_range(price.supply*0.8..price.supply);
        let weighted_demand = rng.gen_range(price.demand*0.8..price.demand);
        if weighted_demand > weighted_supply {
            price.supply += MARKET_FORCE * price_gap;
        }
        if weighted_demand <= weighted_supply {
            price.demand += MARKET_FORCE * price_gap;
        }
    }
}
