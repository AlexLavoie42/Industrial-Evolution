use crate::*;

pub fn ui_setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
    q_camera_entity: Query<Entity, With<Camera>>,
) {
    font_mapping.set_default(asset_server.load("roboto.kttf"));

    let camera_entity = q_camera_entity.single();
    let mut widget_context = KayakRootContext::new(camera_entity);
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    widget_context.add_widget_data::<PlayerMoneyHUDProps, EmptyState>();
    widget_context.add_widget_data::<ImageButtonProps, EmptyState>();

    widget_context.add_widget_system(
        HUDContainerProps::default().get_name(),
        widget_update_with_day_state::<HUDContainerProps, EmptyState>,
        hud_container_render,
    );
    widget_context.add_widget_system(
        // We are registering these systems with a specific WidgetName.
        PlayerMoneyHUDProps::default().get_name(),
        // widget_update auto diffs props and state.
        // Optionally if you have context you can use: widget_update_with_context
        // otherwise you will need to create your own widget update system!
        widget_update_with_money::<PlayerMoneyHUDProps, EmptyState>,
        // Add our render system!
        player_money_hud_render,
    );
    widget_context.add_widget_system(
        PowerMinigameHUDProps::default().get_name(),
        widget_update_on_tick::<PowerMinigameHUDProps, EmptyState>,
        power_minigame_hud_render,
    );
    widget_context.add_widget_system(
        ClockHUDProps::default().get_name(),
        widget_update_on_tick::<ClockHUDProps, EmptyState>,
        clock_hud_render,
    );
    widget_context.add_widget_system(
        ImageButtonProps::default().get_name(),
        widget_update::<ImageButtonProps, ImageButtonState>,
        image_button_render,
    );
    widget_context.add_widget_data::<ImageButtonProps, ImageButtonState>();
    widget_context.add_widget_system(
        BaseHudProps::default().get_name(),
        widget_update_with_player_state::<BaseHudProps, EmptyState>,
        base_hud_render,
    );
    widget_context.add_widget_system(
        AssembliesHudProps::default().get_name(),
        widget_update_with_player_state::<AssembliesHudProps, EmptyState>,
        assemblies_hud_render,
    );
    widget_context.add_widget_system(
        WorkerMenuHUDProps::default().get_name(),
        widget_update_on_tick::<WorkerMenuHUDProps, EmptyState>,
        worker_menu_hud_render,
    );


    widget_context.add_widget_system(
        NightUIProps::default().get_name(),
        widget_update_with_day_state::<NightUIProps, EmptyState>,
        night_ui_render,
    );
    widget_context.add_widget_system(
        DayCountText::default().get_name(),
        widget_update_with_player_state::<DayCountText, EmptyState>,
        day_count_text_render,
    );
    widget_context.add_widget_system(
        ImportsSelection::default().get_name(),
        widget_update_with_day_state::<ImportsSelection, EmptyState>,
        imports_selection_render,
    );
    widget_context.add_widget_system(
        ImportSelector::default().get_name(),
        widget_update_with_import_selection::<ImportSelector, EmptyState>,
        import_selector_render,
    );
    widget_context.add_widget_system(
        RevenueSummaryProps::default().get_name(),
        widget_update_on_tick::<RevenueSummaryProps, EmptyState>,
        render_revenue_summary,
    );
    
    
    let base_hud_menu_image = assets.load("Hud Menu-Thin.png");

    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <HUDContainerBundle />
            <NightUIBundle
                props={NightUIProps {
                    image: base_hud_menu_image.clone(),
                }}
            />
        </KayakAppBundle>
    };

    commands.spawn((widget_context, EventDispatcher::default()));
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct ImageButtonProps {
    pub image: Handle<Image>,
    pub hover_image: Handle<Image>,
    pub selected_image: Handle<Image>,
    pub selected: bool,
}
impl Widget for ImageButtonProps {}

#[derive(Component, Default, PartialEq, Clone)]
pub struct ImageButtonState {
    pub hover: bool,
}

#[derive(Bundle)]
pub struct ImageButtonBundle {
    pub props: ImageButtonProps,
    pub state: ImageButtonState,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for ImageButtonBundle {
    fn default() -> Self {
        Self {
            on_event: OnEvent::default(),
            props: Default::default(),
            state: Default::default(),
            styles: KStyle {
                font_size: StyleProp::Value(45.0),
                ..default()
            },
            computed_styles: Default::default(),
            widget_name: ImageButtonProps::default().get_name(),
        }
    }
}

pub fn image_button_render(
    In(entity): In<Entity>,
    mut commands: Commands,
    widget_context: Res<KayakWidgetContext>,
    mut query: Query<(&mut ImageButtonProps, &mut ComputedStyles, &KStyle, &mut OnEvent)>,
    button_state: Query<&ImageButtonState>
) -> bool {
    if let Ok((props, mut computed_styles, style, mut event)) = query.get_mut(entity) {
        *computed_styles = KStyle::default()
            .with_style(style)
            .into();

        let parent_id = Some(entity);
        let mut image = props.image.clone();
        let state_entity = widget_context.use_state(
            // Bevy commands
            &mut commands,
            // The widget entity.
            entity,
            // The default starting values for the state.
            ImageButtonState::default()
        );
        if let Ok(state) = button_state.get(state_entity) {
            if props.selected {
                image = props.selected_image.clone();
            }
            if state.hover {
                image = props.hover_image.clone();
            }
        }
        rsx!(
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: image.clone(),
                    border: Edge::all(0.0),
                }}
                styles={KStyle {
                    cursor: KCursorIcon(CursorIcon::Hand).into(),
                    ..default()
                }}
                on_event={OnEvent::new(
                    move |
                        In(entity): In<Entity>,
                        mut commands: Commands,
                        event: ResMut<KEvent>,
                        mut q_state: Query<&mut ImageButtonState>,
                    | {
                        if let Ok(mut state) = q_state.get_mut(state_entity) {
                            state.hover = false;
                            if let EventType::Hover(_) = event.event_type {
                                state.hover = true;
                            }
                        }
                    },
                )}
            />
        );
    }
    true
}
