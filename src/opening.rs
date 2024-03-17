use crate::*;

pub struct OpeningPlugin;
impl Plugin for OpeningPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OpeningDialogue>()
            .add_systems(Update, cycle_opening_dialogue.run_if(in_state(DayCycleState::Opening)));
    }
}

#[derive(Clone, Debug, Resource)]
pub struct OpeningDialogue {
    pub text: Vec<String>,
    pub index: usize,
}
impl Default for OpeningDialogue {
    fn default() -> Self {
        Self {
            text: vec![
                "Congratulations. You have inherited a factory from your late grandfather as well as a small amount of cash.".to_string(),
                "Unfortunately the factory has been abandoned for a while. There doesnt seem to be any equipment left.".to_string(),
                "Luckily you should have enough money to purchase a new saw mill.".to_string(),
                "I have purchased some wood for you to get started. Good luck!".to_string(),
            ],
            index: 0,
        }
    }
}

pub fn cycle_opening_dialogue(
    mut opening_dialogue: ResMut<OpeningDialogue>,
    input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut day_state: ResMut<NextState<DayCycleState>>,
) {
    if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::Return) || mouse_input.just_pressed(MouseButton::Left) {
        if opening_dialogue.index < opening_dialogue.text.len() - 1 {
            opening_dialogue.index += 1;
        } else {
            day_state.set(DayCycleState::Day);
        }
    }
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct OpeningDialogueProps {
    pub hover_image: Handle<Image>,
    pub selected_image: Handle<Image>,
    pub selected: bool,
    pub disabled: bool,
}
impl Widget for OpeningDialogueProps {}

#[derive(Bundle)]
pub struct OpeningDialogueBundle {
    pub props: OpeningDialogueProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for OpeningDialogueBundle {
    fn default() -> Self {
        Self {
            on_event: OnEvent::default(),
            props: Default::default(),
            styles: KStyle {
                font_size: StyleProp::Value(45.0),
                ..default()
            },
            computed_styles: Default::default(),
            widget_name: OpeningDialogueProps::default().get_name(),
        }
    }
}

pub fn opening_dialogue_render(
    In(entity): In<Entity>,
    mut commands: Commands,
    widget_context: Res<KayakWidgetContext>,
    mut query: Query<(&mut OpeningDialogueProps, &mut ComputedStyles, &KStyle, &mut OnEvent)>,
    day_state: Res<State<DayCycleState>>,
    opening_dialogue: Res<OpeningDialogue>,
    
) -> bool {
    if let Ok((props, mut computed_styles, style, mut event)) = query.get_mut(entity) {
        *computed_styles = KStyle::default()
            .with_style(style)
            .into();
        if *day_state.get() != DayCycleState::Opening {
            computed_styles.0.width = Units::Pixels(0.0).into();
            computed_styles.0.height = Units::Pixels(0.0).into();
            return true;
        }

        let parent_id = Some(entity);

        rsx!(
            <BackgroundBundle
                styles={KStyle {
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
                            dialogue: opening_dialogue.text[opening_dialogue.index].clone()
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

#[derive(Component, Clone, PartialEq, Default)]
pub struct DialogueProps {
    pub dialogue: String
}
impl Widget for DialogueProps {}

#[derive(Bundle)]
pub struct DialogueBundle {
    pub props: DialogueProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for DialogueBundle {
    fn default() -> Self {
        Self {
            on_event: OnEvent::default(),
            props: Default::default(),
            styles: KStyle {
                font_size: StyleProp::Value(45.0),
                ..default()
            },
            computed_styles: Default::default(),
            widget_name: DialogueProps::default().get_name(),
        }
    }
}

pub fn dialogue_render(
    In(entity): In<Entity>,
    mut commands: Commands,
    widget_context: Res<KayakWidgetContext>,
    mut query: Query<(&mut DialogueProps, &mut ComputedStyles, &KStyle, &mut OnEvent)>,
) -> bool {
    if let Ok((props, mut computed_styles, style, mut event)) = query.get_mut(entity) {
        *computed_styles = KStyle::default()
            .with_style(style)
            .into();

        let parent_id = Some(entity);
        rsx!(
            <TextWidgetBundle
                text={TextProps {
                    content: props.dialogue.clone(),
                    ..Default::default()
                }}
                styles={KStyle {
                    ..default()
                }}
            />
        );
    }
    true
}
