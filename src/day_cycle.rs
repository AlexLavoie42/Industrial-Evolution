use crate::*;

const DAY_LENGTH_SECONDS : f32 = 60.0 * 1.5;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum DayCycleState {
    Day,
    #[default]
    Night
}

#[derive(Resource)]
pub struct DayTimer {
    pub day_count: i32,
    pub day_timer: Timer
}
impl Default for DayTimer {
    fn default() -> Self {
        Self {
            day_count: 0,
            day_timer: Timer::from_seconds(DAY_LENGTH_SECONDS, TimerMode::Repeating)
        }
    }
}

pub fn day_timer_system(
    mut day_timer: ResMut<DayTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<DayCycleState>>,
    day_state: Res<State<DayCycleState>>,
) {
    if day_state.get() == &DayCycleState::Night { return; }

    if day_timer.day_timer.tick(time.delta()).just_finished() {
        next_state.set(DayCycleState::Night);
        day_timer.day_count += 1;
    }
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct NightUIProps {
    pub image: Handle<Image>,
}
impl Widget for NightUIProps {}

#[derive(Bundle)]
pub struct NightUIBundle {
    pub props: NightUIProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for NightUIBundle {
    fn default() -> Self {
        Self {
            props: Default::default(),
            styles: KStyle {
                font_size: StyleProp::Value(45.0),
                ..default()
            },
            computed_styles: Default::default(),
            children: Default::default(),
            on_event: OnEvent::default(),
            widget_name: NightUIProps::default().get_name(),
        }
    }
}

pub fn widget_update_with_day_state<
Props: PartialEq + Component + Clone,
KState: PartialEq + Component + Clone,
>(
    In((entity, previous_entity)): In<(Entity, Entity)>,
    widget_context: Res<KayakWidgetContext>,
    widget_param: WidgetParam<Props, KState>,
    day_state: Res<State<DayCycleState>>,
) -> bool {
    widget_param.has_changed(&widget_context, entity, previous_entity) || day_state.is_changed()
}

pub fn night_ui_render(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    mut query: Query<(&mut NightUIProps, &mut ComputedStyles, &KStyle, &KChildren, &OnEvent)>,
    day_state: Res<State<DayCycleState>>
) -> bool {
    if let Ok((mut props, mut computed_styles, base_style, base_children, base_on_event)) = query.get_mut(entity) {
        *computed_styles = KStyle {
            ..Default::default()
        }
        .with_style(base_style)
        .into();
        if day_state.get() == &DayCycleState::Night {
            let parent_id = Some(entity);
            rsx!(
                <BackgroundBundle
                    styles={KStyle {
                        background_color: StyleProp::<Color>::Value(Color::rgb_u8(31, 31, 35)),
                        ..Default::default()
                    }}
                    children={base_children.clone()}
                    on_event={base_on_event.clone()}
                />
            );
        } else {
            computed_styles.0.height = StyleProp::Value(Units::Pixels(0.0));
            computed_styles.0.width = StyleProp::Value(Units::Pixels(0.0));
        }
    }
    true
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct DayCountText;
impl Widget for DayCountText {}

#[derive(Bundle)]
pub struct DayCountTextBundle {
    pub props: DayCountText,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
}
impl Default for DayCountTextBundle {
    fn default() -> Self {
        Self {
            props: Default::default(),
            styles: KStyle {
                font_size: StyleProp::Value(45.0),
                ..default()
            },
            computed_styles: Default::default(),
            widget_name: DayCountText::default().get_name(),
        }
    }
}

pub fn day_count_text_render(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    mut query: Query<(&mut ComputedStyles, &KStyle)>,
    day_timer: Res<DayTimer>,
) -> bool {
    if let Ok((mut computed_styles, base_style)) = query.get_mut(entity) {
        *computed_styles = KStyle {
            ..Default::default()
        }
        .with_style(base_style)
        .into();

        let parent_id = Some(entity);
        rsx!(
            <TextWidgetBundle
                text={TextProps {
                    content: format!("Day {}", day_timer.day_count),
                    ..Default::default()
                }}
            />
        );
    }
    true
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct ReceivablesSelection;
impl Widget for ReceivablesSelection {}

#[derive(Bundle)]
pub struct ReceivablesSelectionBundle {
    pub props: ReceivablesSelection,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for ReceivablesSelectionBundle {
    fn default() -> Self {
        Self {
            props: Default::default(),
            styles: KStyle {
                ..Default::default()
            },
            computed_styles: Default::default(),
            children: Default::default(),
            on_event: OnEvent::default(),
            widget_name: ReceivablesSelection::default().get_name(),
        }
    }
}

pub fn receivables_selection_render(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    mut query: Query<(&mut ComputedStyles, &KStyle, &KChildren, &OnEvent)>,
    economy: Res<Economy>,
) -> bool {
    if let Ok((mut computed_styles, base_style, base_children, base_on_event)) = query.get_mut(entity) {
        *computed_styles = KStyle {
            ..Default::default()
        }
        .with_style(base_style)
        .into();

    let parent_id = Some(entity);
    rsx!(
        <ElementBundle
            styles={KStyle {
                background_color: StyleProp::<Color>::Value(Color::rgb_u8(31, 31, 35)),
                ..Default::default()
            }}
            children={base_children.clone()}
            on_event={base_on_event.clone()}
        >
            <TextWidgetBundle
                text={TextProps {
                    content: "Receivables".to_string(),
                    ..Default::default()
                }}
            />
            {
                for (item, price) in economy.prices.iter() {
                    constructor!(
                        <ReceivableSelectorBundle
                            props={ReceivableSelector {
                                item: item.clone(),
                                price: price.current_price
                            }}
                            on_event={OnEvent::new(
                                move |In(entity): In<Entity>, event: ResMut<KEvent>, mut selected_receivables: ResMut<ReceivableSelections>, props: Query<&ReceivableSelector> | {
                                    if let EventType::Click(_) = event.event_type {
                                        let Ok(selector) = props.get(entity) else { return; };
                                        selected_receivables.selected.push(selector.item.clone());
                                    }
                                },
                            )}
                        />
                    );
                }
            }
        </ElementBundle>
    );
    }
    true
}

#[derive(Resource, Default)]
pub struct ReceivableSelections {
    pub selected: Vec<PurchasableItem>
}

pub fn widget_update_with_receivable_selection<
Props: PartialEq + Component + Clone,
KState: PartialEq + Component + Clone,
>(
    In((entity, previous_entity)): In<(Entity, Entity)>,
    widget_context: Res<KayakWidgetContext>,
    widget_param: WidgetParam<Props, KState>,
    receivables_selections: Res<ReceivableSelections>
) -> bool {
    widget_param.has_changed(&widget_context, entity, previous_entity) || receivables_selections.is_changed()
}

#[derive(Component, Clone, PartialEq)]
pub struct ReceivableSelector {
    pub item: PurchasableItem,
    pub price: f32
}
impl Default for ReceivableSelector {
    fn default() -> Self {
        Self {
            item: PurchasableItem::Resource(ResourceItem::Wood),
            price: 0.0
        }
    }
}
impl Widget for ReceivableSelector {}

#[derive(Bundle)]
pub struct ReceivableSelectorBundle {
    pub props: ReceivableSelector,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for ReceivableSelectorBundle {
    fn default() -> Self {
        Self {
            props: Default::default(),
            styles: KStyle {
                ..Default::default()
            },
            computed_styles: Default::default(),
            children: Default::default(),
            on_event: OnEvent::default(),
            widget_name: ReceivableSelector::default().get_name(),
        }
    }
}


pub fn receivable_selector_render(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    mut query: Query<(&ReceivableSelector, &mut ComputedStyles, &KStyle, &KChildren, &OnEvent)>,
    receivables_selections: Res<ReceivableSelections>,
) -> bool {
    if let Ok((props, mut computed_styles, base_style, base_children, base_on_event)) = query.get_mut(entity) {
        *computed_styles = KStyle {
            ..Default::default()
        }
        .with_style(base_style)
        .into();
    
        let selected_count = receivables_selections.selected.iter().filter(|item| *item == &props.item).count();
        let parent_id = Some(entity);
        rsx!(
            <NinePatchBundle
                styles={KStyle {
                    ..default()
                }}
            >
                <TextWidgetBundle
                    text={TextProps {
                        content: format!("{:?}: {:.2} || Selected: {:0} ", props.item, props.price, selected_count),
                        ..Default::default()
                    }}
                />
            </NinePatchBundle>
        );
    }
    true
}
