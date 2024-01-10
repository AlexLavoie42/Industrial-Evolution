use bevy::ecs::world::WorldCell;
use bevy::input::mouse::MouseButton;

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
        let mut world_cell = world.cell();
        check_action_finished(&mut world_cell)
    };

    if finished {
        let mut world_cell = world.cell();
        increment_step_system(&mut world_cell);
    }
}
pub enum TutorialAction {
    LeftClick,
}
impl Default for TutorialState {
    fn default() -> Self {
        Self {
            enabled: true,
            step: 0,
            steps: vec![
                TutorialStep {
                    dialogue: "test".to_string(),
                    action: TutorialAction::LeftClick,
                },
            ],
        }
    }
}
pub fn check_action_finished(world: &mut WorldCell) -> bool {
    let state = world.get_resource::<TutorialState>().unwrap();
    let action = &state.steps.get(state.step as usize).unwrap().action;
    match action {
        TutorialAction::LeftClick => {
            let mouse = world.get_resource::<Input<MouseButton>>().unwrap();
            mouse.just_pressed(MouseButton::Left)
        },
    }
}

pub fn increment_step_system(world_cell: &mut WorldCell) {
    let mut state = world_cell.get_non_send_resource_mut::<TutorialState>().unwrap();
    state.step += 1;
}