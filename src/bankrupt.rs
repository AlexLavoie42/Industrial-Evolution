use crate::*;

pub struct BankruptPlugin;
impl Plugin for BankruptPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BankruptDialogue>()
            .add_systems(Update, cycle_bankrupt_dialogue.run_if(in_state(DayCycleState::Bankrupt)));
    }
}

#[derive(Clone, Debug, Resource)]
pub struct BankruptDialogue {
    pub text: Vec<String>,
    pub index: usize,
}
impl Default for BankruptDialogue {
    fn default() -> Self {
        Self {
            text: vec![
                "You do not have enough money to pay todays bills.".to_string(),
                "The bank has seized your assets.".to_string(),
                "The End.".to_string(),
            ],
            index: 0,
        }
    }
}

pub fn cycle_bankrupt_dialogue(
    mut bankrupt_dialogue: ResMut<BankruptDialogue>,
    input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut tutorial_state: ResMut<TutorialState>,
) {
    tutorial_state.enabled = false;
    if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::Return) || mouse_input.just_pressed(MouseButton::Left) {
        if bankrupt_dialogue.index < bankrupt_dialogue.text.len() - 1 {
            bankrupt_dialogue.index += 1;
        }
    }
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct BankruptDialogueProps {
    pub hover_image: Handle<Image>,
    pub selected_image: Handle<Image>,
    pub selected: bool,
    pub disabled: bool,
}
impl Widget for BankruptDialogueProps {}

#[derive(Bundle)]
pub struct BankruptDialogueBundle {
    pub props: BankruptDialogueProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for BankruptDialogueBundle {
    fn default() -> Self {
        Self {
            on_event: OnEvent::default(),
            props: Default::default(),
            styles: KStyle {
                font_size: StyleProp::Value(45.0),
                ..default()
            },
            computed_styles: Default::default(),
            widget_name: BankruptDialogueProps::default().get_name(),
        }
    }
}

pub fn bankrupt_dialogue_render(
    In(entity): In<Entity>,
    mut commands: Commands,
    widget_context: Res<KayakWidgetContext>,
    mut query: Query<(&mut BankruptDialogueProps, &mut ComputedStyles, &KStyle, &mut OnEvent)>,
    day_state: Res<State<DayCycleState>>,
    bankrupt_dialogue: Res<BankruptDialogue>,
    
) -> bool {
    if let Ok((props, mut computed_styles, style, mut event)) = query.get_mut(entity) {
        *computed_styles = KStyle::default()
            .with_style(style)
            .into();
        if *day_state.get() != DayCycleState::Bankrupt {
            computed_styles.0.width = Units::Pixels(0.0).into();
            computed_styles.0.height = Units::Pixels(0.0).into();
            return true;
        }

        let parent_id = Some(entity);
        rsx!(
            <BackgroundBundle
                styles={KStyle {
                    z_index: StyleProp::Value(10000),
                    background_color: StyleProp::<Color>::Value(Color::rgb_u8(50, 58, 108)),
                    ..default()
                }}
            >
                // <NinePatchBundle
                //     styles={KStyle {
                //         ..default()
                //     }}
                // >
                    <DialogueBundle
                        props={DialogueProps {
                            dialogue: bankrupt_dialogue.text.get(bankrupt_dialogue.index).unwrap_or(bankrupt_dialogue.text.last().unwrap()).clone()
                        }}
                        styles={KStyle {
                            font_size: StyleProp::Value(64.0),
                            padding_top: Units::Pixels(128.0).into(),
                            // padding_left: Units::Pixels(32.0).into(),
                            // padding_right: Units::Pixels(32.0).into(),
                            ..default()
                        }}
                    />
                // </NinePatchBundle>
            </BackgroundBundle>
        );
    }
    true
}
