use bevy::ecs::{world::WorldCell, system::SystemState};

use crate::*;

pub struct TutorialPlugin;
impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, tutorial_system)
            .insert_non_send_resource(TutorialState::default());
    }
}

pub struct TutorialStep {
    pub dialogue: String,
    pub action: TutorialAction,
}

#[derive(Resource)]
pub struct TutorialState {
    pub enabled: bool,
    pub step: u8,
    pub steps: Vec<TutorialStep>,
}

pub fn tutorial_system(
    mut world: &mut World,
) {
    let finished = {
        check_action_finished(&mut world)
    };

    if finished {
        increment_step_system(&mut world);
    }
}
pub enum TutorialAction {
    Movement,
}
impl Default for TutorialState {
    fn default() -> Self {
        Self {
            enabled: true,
            step: 0,
            steps: vec![
                TutorialStep {
                    dialogue: "Move your character with WASD".to_string(),
                    action: TutorialAction::Movement,
                },
            ],
        }
    }
}
pub fn check_action_finished(world: &mut World) -> bool {
    let state = world.get_resource::<TutorialState>().unwrap();
    let action = &state.steps.get(state.step as usize).unwrap().action;
    match action {
        TutorialAction::Movement => {
            let mut movement: SystemState<
                Query<&Movement, With<Player>>
            > = SystemState::new(world);

            movement.get(world).single().speed_x > 0.0 || movement.get(world).single().speed_y > 0.0
        },
    }
}

pub fn increment_step_system(world_cell: &mut World) {
    let mut state = world_cell.get_non_send_resource_mut::<TutorialState>().unwrap();
    state.step += 1;
}


