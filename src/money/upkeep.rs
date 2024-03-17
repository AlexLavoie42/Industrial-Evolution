use crate::*;

const DAY_TIMER: f32 = 6.5 * 60.0;

const FACTORY_COST: f32 = 4.5;
const LIVING_EXPENSE_BASE: f32 = 0.25;

pub const STORAGE_FEE: f32 = 0.05;
pub const WORKER_UPKEEP: f32 = 0.6;

#[derive(Resource)]
pub struct UpkeepTimer(Timer);
impl Default for UpkeepTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(DAY_TIMER, TimerMode::Repeating))
    }
}

#[derive(PartialEq, Clone, Copy, Reflect)]
pub enum UpkeepSource {
    Factory,
    Worker,
    Living,
    Storage
}

#[derive(Clone, Copy)]
pub struct Upkeep (pub f32, pub UpkeepSource);

#[derive(Resource)]
pub struct UpkeepTracker {
    pub upkeep: Vec<Upkeep>
}
impl UpkeepTracker {
    pub fn new() -> Self {
        let mut upkeep = Self {
            upkeep: vec![
            ]
        };
        upkeep
    }
    fn calculate_worker_upkeep(&mut self, workers: i32) {
        self.upkeep = self.upkeep.iter().filter(|x| x.1 != UpkeepSource::Worker).map(|x| *x).collect::<Vec<_>>();
        for _ in 0..workers {
            self.upkeep.push(Upkeep(WORKER_UPKEEP, UpkeepSource::Worker));
        }
    }
}

pub fn factory_upkeep(
    mut upkeep_tracker: ResMut<UpkeepTracker>,
    q_workers: Query<&Worker>,
) {
    upkeep_tracker.upkeep.push(Upkeep(FACTORY_COST, UpkeepSource::Factory));
    upkeep_tracker.calculate_worker_upkeep(q_workers.iter().count() as i32);
}

pub fn living_expenses(
    mut upkeep_tracker: ResMut<UpkeepTracker>,
) {
    upkeep_tracker.upkeep.push(Upkeep(LIVING_EXPENSE_BASE, UpkeepSource::Living));
}

pub fn upkeep_system(
    mut player_money: ResMut<PlayerMoney>,
    mut market_timer: ResMut<UpkeepTimer>,
    mut upkeep_tracker: ResMut<UpkeepTracker>,
    time: Res<Time>,
    mut day_cycle: ResMut<NextState<DayCycleState>>,
) {
    let total = upkeep_tracker.upkeep.iter().map(|x| x.0).sum();
    if let Err(err) = player_money.try_remove_money(total) {
        println!("Cant afford upkeep!");
        day_cycle.set(DayCycleState::Bankrupt);
    }
    upkeep_tracker.upkeep.clear();
}

pub fn item_storage_fee(
    mut money: ResMut<PlayerMoney>,
    mut upkeep_tracker: ResMut<UpkeepTracker>,
    mut q_import_containers: Query<&mut ItemContainer, (With<ItemImport>, Without<ItemExport>)>,
    mut q_export_containers: Query<&mut ItemContainer, (With<ItemExport>, Without<ItemImport>)>
) {
    for mut container in q_import_containers.iter().chain(q_export_containers.iter()) {
        for item in container.items.iter() {
            if let Some(item_entity) = item {
                upkeep_tracker.upkeep.push(Upkeep (STORAGE_FEE, UpkeepSource::Storage));
            }
        }
    }
}
