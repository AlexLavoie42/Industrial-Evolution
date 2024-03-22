use bevy::{core_pipeline::core_2d::graph::input, ecs::system::{SystemId, SystemState}};

use crate::*;

pub struct TutorialPlugin;
impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, register_tutorial)
            .add_systems(Update, tutorial_system)
            .add_state::<TutorialState>()
            .insert_resource(MovementTimer(0.0));
    }
}

pub struct TutorialStep {
    pub dialogue: String,
    pub action: SystemId,
}

#[derive(Resource)]
pub struct TutorialSteps {
    pub step: u8,
    pub steps: Vec<TutorialStep>,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum TutorialState {
    #[default]
    Enabled,
    Disabled,
}

pub fn register_tutorial(
    mut world: &mut World,

) {
    let tut_state = TutorialSteps::new(&mut world);
    world.insert_resource(tut_state);
}

pub fn tutorial_system(
    mut tut_steps: ResMut<TutorialSteps>,
    mut next_tut_state: ResMut<NextState<TutorialState>>,
    tut_state: Res<State<TutorialState>>,
    mut commands: Commands,
) {
    if tut_state.get() == &TutorialState::Enabled {
        let Some(step) = tut_steps.steps.get(tut_steps.step as usize) else {
            print!("Reached end of tutorial");
            next_tut_state.set(TutorialState::Disabled);
            return;
        };
        commands.run_system(step.action);
    }
}

impl TutorialSteps {
    fn new(world: &mut World) -> Self {
        Self {
            step: 0,
            steps: vec![
                TutorialStep {
                    dialogue: "Move your character with WASD.\nYou can adjust the camera level with the scroll wheel".to_string(),
                    action: world.register_system(check_movement),
                },
                TutorialStep {
                    dialogue: "Lets start with building a saw mill.\nFirst click on \"Assemblies\"".to_string(),
                    action: world.register_system(
                        |
                            player_state: Res<State<PlayerState>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            if *player_state.get() == PlayerState::Assemblies {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    )
                },
                TutorialStep {
                    dialogue: "Next click on \"Saw Mill\".\nYou can then click on the ground to place one".to_string(),
                    action: world.register_system(
                        |
                            q_assemblies: Query<&AssemblyType, With<Assembly>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            let assemblies = q_assemblies.iter().collect::<Vec<_>>();
                            if assemblies.len() > 0 && assemblies.contains(&&AssemblyType::SawMill) {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    )
                },
                TutorialStep {
                    dialogue: "Press \"Tab\" to exit building mode.".to_string(),
                    action: world.register_system(
                        |
                            player_state: Res<State<PlayerState>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            if *player_state.get() == PlayerState::None {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    ),
                },
                TutorialStep {
                    dialogue: "Now grab some wood and get working!
                    \nYou can find the wood in your imports section
                    \nPress \"F\" to pickup items. You will pick up items that are closest to the mouse cursor.
                    \nYou can also drop items by pressing \"Q\"".to_string(),
                    action: world.register_system(
                        |
                            q_player: Query<&ItemContainer, With<Player>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            let player_container = q_player.single();

                            if player_container.items.len() > 0 {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    )
                },
                TutorialStep {
                    dialogue: "Now approach the saw mill and press \"F\" to place the wood inside.".to_string(),
                    action: world.register_system(
                        |
                            q_assemblies: Query<(&ItemIOContainer, &AssemblyType), With<Assembly>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            let assemblies = q_assemblies.iter().filter(|x| x.1 == &AssemblyType::SawMill).collect::<Vec<_>>();
                            for (assembly_container, _) in assemblies {
                                if assembly_container.input.items.len() > 0 {
                                    increment_step_system(tut_steps, tut_state);
                                    break;
                                }
                            }
                        }
                    )
                },
                TutorialStep {
                    dialogue: "Now click on the saw mill to start manually cutting.".to_string(),
                    action: world.register_system(
                        |
                            player_state: Res<State<PlayerState>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            if *player_state.get() == PlayerState::Power {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    )
                },
                TutorialStep {
                    dialogue: "Keep using the saw mill until it is finished.
                    \nTry pressing the space bar quickly.
                    \nThe faster you press the space bar, the faster you will produce!".to_string(),
                    action: world.register_system(
                        |
                            q_assemblies: Query<(&ItemIOContainer, &AssemblyType), With<Assembly>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            let assemblies = q_assemblies.iter().filter(|x| x.1 == &AssemblyType::SawMill).collect::<Vec<_>>();
                            for (assembly_container, _) in assemblies {
                                if assembly_container.output.items.len() > 0 {
                                    increment_step_system(tut_steps, tut_state);
                                    break;
                                }
                            }
                        }
                    )
                },
                TutorialStep {
                    dialogue: "The saw mill has finished cutting all the wood!
                    \nPress \"Tab\" to leave the saw mill".to_string(),
                    action: world.register_system(
                        |
                            player_state: Res<State<PlayerState>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            if *player_state.get() != PlayerState::Power {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    )
                },
                TutorialStep {
                    dialogue: "Now that you have some lumber, pick it up from the saw mill.
                    \nPress \"F\" while standing near the saw mill to pickup the lumber.".to_string(),
                    action: world.register_system(
                        |
                            q_player: Query<&ItemContainer, With<Player>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            let player_container = q_player.single();

                            if player_container.items.len() > 0 {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    )
                },
                TutorialStep {
                    dialogue: "Place the lumber in the exports area to be sold at the end of the day.
                    \nPress \"F\" while standing near the exports area to place the lumber.".to_string(),
                    action: world.register_system(
                        |
                            q_exports: Query<&ItemContainer, With<ItemExport>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            let exports_container = q_exports.single();
                            if exports_container.items.len() > 0 {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    )
                },
                TutorialStep {
                    dialogue: "You wont be able to produce much on your own.
                    \nYou should hire somebody to do it for you!
                    \nClick on \"Workers\"".to_string(),
                    action: world.register_system(
                        |
                            player_state: Res<State<PlayerState>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            if *player_state.get() == PlayerState::Workers {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    )
                },
                TutorialStep {
                    dialogue: "Now click on the ground to place a worker.
                    \nEach worker costs $5 to hire, and will cost you 60¢ each day in salary.".to_string(),
                    action: world.register_system(
                        |
                            q_workers: Query<&Worker>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            if q_workers.iter().count() > 0 {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    ),
                },
                TutorialStep {
                    dialogue: "Press \"Tab\" to exit hiring mode.".to_string(),
                    action: world.register_system(
                        |
                            player_state: Res<State<PlayerState>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            if *player_state.get() == PlayerState::None {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    ),
                },
                TutorialStep {
                    dialogue: "Now click on the worker so you can tell them what to do.".to_string(),
                    action: world.register_system(
                        |
                            player_state: Res<State<PlayerState>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            // if *player_state.get() != PlayerState::None {
                            //     tut_steps.step -= 1;
                            // }
                            if *player_state.get() == PlayerState::Jobs {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    ),
                },
                TutorialStep {
                    dialogue: "Click anywhere on the screen to tell the worker what to do.
                    \nClicking on red arrows will instruct the worker to pick up items.
                    \nClick on the red arrow under the export area to instruct the worker to pickup wood.
                    \nRight clicking on a green box will delete the instruction.".to_string(),
                    action: world.register_system(
                        |
                            selected_worker: Res<SelectedWorker>,
                            q_jobs: Query<&Job>,
                            mut tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            if selected_worker.selected.is_none() {
                                return;
                            }
                            let Some(job) = q_jobs.get(selected_worker.selected.unwrap()).ok() else { 
                                tut_steps.step -= 1;
                                return; 
                            };
                            if job.path.len() == 1 && matches!(job.path[0].action, JobAction::ContainerPickup { .. }) {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    ),
                },
                TutorialStep {
                    dialogue: "Clicking on green arrows will instruct the worker to drop items.
                    \nClick on the green arrow above the saw mill to instruct the worker to drop the wood.
                    \nRight clicking on a green box will delete the instruction.".to_string(),
                    action: world.register_system(
                        |
                            selected_worker: Res<SelectedWorker>,
                            q_jobs: Query<&Job>,
                            mut tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            if selected_worker.selected.is_none() {
                                return;
                            }
                            let Some(job) = q_jobs.get(selected_worker.selected.unwrap()).ok() else { 
                                return; 
                            };
                            if job.path.len() == 2 && matches!(job.path[0].action, JobAction::ContainerPickup { .. }) && matches!(job.path[1].action, JobAction::Drop { .. }) {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    ),
                },
                TutorialStep {
                    dialogue: "Clicking on an assembly will instruct the worker to power it.
                    \nClick on the saw mill to instruct the worker to power it.
                    \nRight clicking on a green box will delete the instruction.".to_string(),
                    action: world.register_system(
                        |
                            selected_worker: Res<SelectedWorker>,
                            q_jobs: Query<&Job>,
                            mut tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            if selected_worker.selected.is_none() {
                                return;
                            }
                            let Some(job) = q_jobs.get(selected_worker.selected.unwrap()).ok() else { 
                                return; 
                            };
                            if job.path.len() == 3 && matches!(job.path[0].action, JobAction::ContainerPickup { .. }) && matches!(job.path[1].action, JobAction::Drop { .. }) {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    ),
                },
                TutorialStep {
                    dialogue: "Great work! Your worker should now be producing the lumber for you.
                    \nPress \"Tab\" to exit".to_string(),
                    action: world.register_system(
                        |
                            player_state: Res<State<PlayerState>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            if *player_state.get() == PlayerState::None {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }*
                    ),
                },
                TutorialStep {
                    dialogue: "You seem to be getting the hang of this!
                    \nOnce you are ready, click on \"End Day\" to end your day.
                    \nKeep in mind you will be charged a storage fee for any items left in your factory!".to_string(),
                    action: world.register_system(
                        |
                            day_cycle: Res<State<DayCycleState>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            if *day_cycle.get() == DayCycleState::Night {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    ),
                },
                TutorialStep {
                    dialogue: "Purchase some wood for tomorrow. Click on the \"+\" next to wood.".to_string(),
                    action: world.register_system(
                        |
                            day_cycle: Res<State<DayCycleState>>,
                            tut_steps: ResMut<TutorialSteps>,
                            tut_state: ResMut<NextState<TutorialState>>,
                        | {
                            if *day_cycle.get() == DayCycleState::Day {
                                increment_step_system(tut_steps, tut_state)
                            }
                        }
                    ),
                },
            ],
        }
    }
}

pub fn increment_step_system(
    mut tut_steps: ResMut<TutorialSteps>,
    mut tut_state: ResMut<NextState<TutorialState>>
) {
    if tut_steps.step < tut_steps.steps.len() as u8 - 1 {
        tut_steps.step += 1;
    } else {
        tut_state.set(TutorialState::Disabled);
    }
}

const MOVEMENT_TIME: f32 = 1.3;
#[derive(Resource)]
pub struct MovementTimer(pub f32);
pub fn check_movement(
    player: Query<&Movement, With<Player>>,
    time: Res<Time>,
    tut_steps: ResMut<TutorialSteps>,
    tut_state: ResMut<NextState<TutorialState>>,
    mut movement_timer: ResMut<MovementTimer>,
) {
    let Some(input) = player.single().input else { return };
    if (input.x == 0.0) && (input.y == 0.0) {
        return;
    }
    movement_timer.0 += time.delta_seconds();
    if movement_timer.0 > MOVEMENT_TIME {
        increment_step_system(tut_steps, tut_state);
    }
}


#[derive(Component, Clone, PartialEq, Default)]
pub struct TutorialProps;
impl Widget for TutorialProps {}

#[derive(Bundle)]
pub struct TutorialBundle {
    pub props: TutorialProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for TutorialBundle {
    fn default() -> Self {
        Self {
            on_event: OnEvent::default(),
            props: Default::default(),
            styles: KStyle {
                font_size: StyleProp::Value(45.0),
                ..default()
            },
            computed_styles: Default::default(),
            widget_name: TutorialProps::default().get_name(),
        }
    }
}

pub fn tutorial_dialogue_render(
    In(entity): In<Entity>,
    mut commands: Commands,
    widget_context: Res<KayakWidgetContext>,
    mut query: Query<(&mut TutorialProps, &mut ComputedStyles, &KStyle, &mut OnEvent)>,
    tutorial_steps: Res<TutorialSteps>,
    tut_state: Res<State<TutorialState>>,
    day_cycle: Res<State<DayCycleState>>,
    assets: Res<AssetServer>,
) -> bool {
    if let Ok((props, mut computed_styles, style, mut event)) = query.get_mut(entity) {
        *computed_styles = KStyle::default()
            .with_style(style)
            .into();
        computed_styles.0.height = Units::Pixels(0.0).into();
        computed_styles.0.width = Units::Pixels(0.0).into();
        computed_styles.0.padding_top = Units::Pixels(64.0).into();
        computed_styles.0.padding_left = Units::Pixels(32.0).into();
        if tut_state.get() == &TutorialState::Disabled || *day_cycle.get() == DayCycleState::Opening {
            return true;
        }

        let parent_id = Some(entity);
        let Some(step) = tutorial_steps.steps.get(tutorial_steps.step as usize) else {
            return true;
        };
        computed_styles.0.color = StyleProp::Value(Color::rgb(0.0, 0.0, 0.0));
        if day_cycle.get() == &DayCycleState::Night {
            computed_styles.0.color = StyleProp::Value(Color::rgb(1.0, 1.0, 1.0));
        }
        computed_styles.0.width = Units::Stretch(1.0).into();

        let skip_tutorial_icon = assets.load("End Tutorial Icon.png");
        let skip_tutorial_icon_hover = assets.load("End Tutorial Icon-Hover.png");
        let skip_tutorial_icon_selected = assets.load("End Tutorial Icon-Selected.png");
        rsx!(
            <ElementBundle>
                <DialogueBundle
                    props={DialogueProps {
                        dialogue: step.dialogue.clone(),
                    }}
                    styles={KStyle {
                        z_index: StyleProp::Value(1000),
                        font_size: StyleProp::Value(38.0),
                        line_height: StyleProp::Value(30.0),
                        background_color: StyleProp::Value(Color::rgb(0.0, 0.0, 0.0)),
                        ..default()
                    }}
                />
                <ImageButtonBundle 
                    props={ImageButtonProps {
                        image: skip_tutorial_icon.clone(),
                        hover_image: skip_tutorial_icon_hover.clone(),
                        selected_image: skip_tutorial_icon_selected.clone(),
                        ..default()
                    }}
                    styles={KStyle {
                        width: Units::Pixels(128.0).into(),
                        height: Units::Pixels(64.0).into(),
                        position_type: StyleProp::Value(KPositionType::SelfDirected),
                        offset: Edge::new(
                            Units::Stretch(0.0),
                            Units::Stretch(0.1),
                            Units::Stretch(0.4),
                            Units::Stretch(1.0),
                        ).into(),
                        ..default()
                    }}
                    on_event={OnEvent::new(
                        move |In(_entity): In<Entity>, event: ResMut<KEvent>, mut tut_state: ResMut<NextState<TutorialState>> | {
                            if let EventType::Click(_) = event.event_type {
                                tut_state.set(TutorialState::Disabled);
                            }
                        },
                    )}
                />
            </ElementBundle>
        );
    }
    true
}
