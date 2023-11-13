use crate::*;

const FACTORY_COST: f32 = 100.0;
const DAY_TIMER: f32 = 5.0 ;

#[derive(Resource)]
pub struct UpkeepTimer(Timer);
impl Default for UpkeepTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(DAY_TIMER, TimerMode::Repeating))
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum UpkeepSource {
    Factory,
    Worker
}

#[derive(Clone, Copy)]
pub struct Upkeep (f32, UpkeepSource);

#[derive(Resource)]
pub struct UpkeepTracker {
    upkeep: Vec<Upkeep>,
    total: f32
}
impl UpkeepTracker {
    pub fn new() -> Self {
        let mut upkeep = Self {
            upkeep: vec![
                Upkeep (FACTORY_COST, UpkeepSource::Factory)
            ],
            total: 0.0
        };
        upkeep.update();
        upkeep
    }
    fn update(&mut self) {
        self.total = self.upkeep.iter().map(|x| x.0).sum();
    }
    fn calculate_worker_upkeep(&mut self, workers: i32) {
        self.upkeep = self.upkeep.iter().filter(|x| x.1 != UpkeepSource::Worker).map(|x| *x).collect::<Vec<_>>();
        for _ in 0..workers {
            self.upkeep.push(Upkeep(WORKER_UPKEEP, UpkeepSource::Worker));
        }
    }
}

pub fn upkeep_system(
    mut player_money: ResMut<PlayerMoney>,
    mut market_timer: ResMut<UpkeepTimer>,
    mut upkeep_tracker: ResMut<UpkeepTracker>,
    q_workers: Query<&Worker>,
    time: Res<Time>,
) {
    if market_timer.0.tick(time.delta()).just_finished() {
        upkeep_tracker.calculate_worker_upkeep(q_workers.iter().count() as i32);
        upkeep_tracker.update();
        if let Err(err) = player_money.try_remove_money(upkeep_tracker.total) {
            println!("Cant afford upkeep!");
        }
    }
}
