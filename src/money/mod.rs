use crate::*;

pub struct MoneyPlugin;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerMoney {
                amount: 10000.0
            })
        ;
    }
}

#[derive(Resource)]
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