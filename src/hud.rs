use std::sync::{Arc, RwLock};

use kayak_ui::prelude::kayak_font::{TextLayout, TextProperties};

use crate::*;

pub fn hud_setup(
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

    let base_hud_menu_image = assets.load("Hud Menu-Thin.png");
    let assembly_mode_menu_image = assets.load("Assemblies Icon.png");
    let worker_mode_menu_image = assets.load("Workers Icon.png");
    let pulp_mill_menu_image = assets.load("Pulp Mill Icon.png");
    let paper_press_menu_image = assets.load("Paper Press Icon.png");
    let paper_drier_menu_image = assets.load("Paper Drier Icon.png");

    // Next we need to add the systems
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
        ImageButtonProps::default().get_name(),
        widget_update::<ImageButtonProps, EmptyState>,
        image_button_render,
    );
    widget_context.add_widget_system(
        AssembliesHudProps::default().get_name(),
        widget_update_with_player_state::<AssembliesHudProps, EmptyState>,
        assemblies_hud_render,
    );

    let assembly_button_click = OnEvent::new(
        move |In(_entity): In<Entity>, event: ResMut<KEvent>, mut next_player_state: ResMut<NextState<PlayerState>>, player_state: Res<State<PlayerState>> | {
            if let EventType::Click(_) = event.event_type {
                if player_state.get() == &PlayerState::Assemblies {
                    next_player_state.set(PlayerState::None);
                } else {
                    next_player_state.set(PlayerState::Assemblies);
                }
            }
        },
    );
    let worker_button_click = OnEvent::new(
        move |In(_entity): In<Entity>, event: ResMut<KEvent>, mut next_player_state: ResMut<NextState<PlayerState>>, player_state: Res<State<PlayerState>> | {
            if let EventType::Click(_) = event.event_type {
                if player_state.get() == &PlayerState::Workers {
                    next_player_state.set(PlayerState::None);
                } else {
                    next_player_state.set(PlayerState::Workers);
                }
            }
        },
    );
    let pulp_mill_button_click = OnEvent::new(
        move |In(_entity): In<Entity>, event: ResMut<KEvent>, mut selected_assembly: ResMut<SelectedAssembly> | {
            if let EventType::Click(_) = event.event_type {
                selected_assembly.selected = AssemblyType::PulpMill;
            }
        },
    );
    
    let paper_press_button_click = OnEvent::new(
        move |In(_entity): In<Entity>, event: ResMut<KEvent>, mut selected_assembly: ResMut<SelectedAssembly> | {
            if let EventType::Click(_) = event.event_type {
                selected_assembly.selected = AssemblyType::PaperPress;
            }
        },
    );
    let paper_drier_button_click = OnEvent::new(
        move |In(_entity): In<Entity>, event: ResMut<KEvent>, mut selected_assembly: ResMut<SelectedAssembly> | {
            if let EventType::Click(_) = event.event_type {
                selected_assembly.selected = AssemblyType::PaperDrier;
            }
        },
    );

    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <PlayerMoneyHUDBundle/>
            <AssembliesHudBundle
                props={AssembliesHudProps {
                    image: base_hud_menu_image.clone(),
                }}
                styles={KStyle {
                    top: Units::Stretch(15.0).into(),
                    left: Units::Stretch(1.0).into(),
                    right: Units::Stretch(1.0).into(),
                    height: Units::Pixels(128.0).into(),
                    width: Units::Pixels(1024.0).into(),
                    layout_type: LayoutType::Row.into(),
                    ..default()
                }}
                on_event={
                    OnEvent::new(
                        move |In(_entity): In<Entity>, event: ResMut<KEvent>, mut placement_state: ResMut<NextState<PlacementState>> | {
                            if let EventType::Hover(_) = event.event_type {
                                placement_state.set(PlacementState::Blocked);
                            }
                            if let EventType::MouseOut(_) = event.event_type {
                                placement_state.set(PlacementState::Allowed);
                            }
                        }
                    )
                }
            >
                <ImageButtonBundle
                    props={ImageButtonProps {
                        image: pulp_mill_menu_image.clone(),
                    }}
                    styles={KStyle {
                        width: Units::Pixels(64.0).into(),
                        height: Units::Pixels(64.0).into(),
                        offset: Edge::new(
                            Units::Stretch(1.0),
                            Units::Pixels(0.0),
                            Units::Stretch(1.0),
                            Units::Pixels(25.0),
                        ).into(),
                        
                        ..default()
                    }}
                    on_event={
                        pulp_mill_button_click
                    }
                />
                <ImageButtonBundle
                    props={ImageButtonProps {
                        image: paper_press_menu_image.clone(),
                    }}
                    styles={KStyle {
                        width: Units::Pixels(64.0).into(),
                        height: Units::Pixels(64.0).into(),
                        offset: Edge::new(
                            Units::Stretch(1.0),
                            Units::Pixels(0.0),
                            Units::Stretch(1.0),
                            Units::Pixels(25.0),
                        ).into(),
                        ..default()
                    }}
                    on_event={
                        paper_press_button_click
                    }
                />
                <ImageButtonBundle
                    props={ImageButtonProps {
                        image: paper_drier_menu_image.clone(),
                    }}
                    styles={KStyle {
                        width: Units::Pixels(64.0).into(),
                        height: Units::Pixels(64.0).into(),
                        offset: Edge::new(
                            Units::Stretch(1.0),
                            Units::Pixels(0.0),
                            Units::Stretch(1.0),
                            Units::Pixels(25.0),
                        ).into(),
                        ..default()
                    }}
                    on_event={
                        paper_drier_button_click
                    }
                />
            </AssembliesHudBundle>
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: base_hud_menu_image.clone(),
                    border: Edge::all(0.0),
                }}
                styles={KStyle {
                    bottom: Units::Pixels(15.0).into(),
                    top: Units::Stretch(1.0).into(),
                    left: Units::Stretch(1.0).into(),
                    right: Units::Stretch(1.0).into(),
                    height: Units::Pixels(128.0).into(),
                    width: Units::Pixels(1024.0).into(),
                    layout_type: LayoutType::Row.into(),
                    ..default()
                }}
                on_event={
                    OnEvent::new(
                        move |In(_entity): In<Entity>, event: ResMut<KEvent>, mut placement_state: ResMut<NextState<PlacementState>> | {
                            if let EventType::Hover(_) = event.event_type {
                                placement_state.set(PlacementState::Blocked);
                            }
                            if let EventType::MouseOut(_) = event.event_type {
                                placement_state.set(PlacementState::Allowed);
                            }
                        }
                    )
                }
            >
                <ImageButtonBundle
                    styles={KStyle {
                        width: Units::Pixels(128.0).into(),
                        height: Units::Pixels(64.0).into(),
                        offset: Edge::new(
                            Units::Stretch(1.0),
                            Units::Pixels(0.0),
                            Units::Stretch(1.0),
                            Units::Pixels(25.0),
                        ).into(),
                        ..default()
                    }}
                    props={ImageButtonProps {
                        image: assembly_mode_menu_image,
                    }}
                    on_event={
                        assembly_button_click
                    }
                />
                
                <ImageButtonBundle
                    styles={KStyle {
                        width: Units::Pixels(128.0).into(),
                        height: Units::Pixels(64.0).into(),
                        offset: Edge::new(
                            Units::Stretch(1.0),
                            Units::Pixels(0.0),
                            Units::Stretch(1.0),
                            Units::Pixels(25.0),
                        ).into(),
                        ..default()
                    }}
                    props={ImageButtonProps {
                        image: worker_mode_menu_image,
                    }}
                    on_event={
                        worker_button_click
                    }
                />
            </NinePatchBundle>
        </KayakAppBundle>
    };

    commands.spawn((widget_context, EventDispatcher::default()));
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct PlayerMoneyHUDProps {
    pub current_money: f32
}
impl Widget for PlayerMoneyHUDProps {}

#[derive(Bundle)]
pub struct PlayerMoneyHUDBundle {
    pub props: PlayerMoneyHUDProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub widget_name: WidgetName,
}
impl Default for PlayerMoneyHUDBundle {
    fn default() -> Self {
        Self {
            props: Default::default(),
            styles: KStyle {
                font_size: StyleProp::Value(45.0),
                ..default()
            },
            computed_styles: Default::default(),
            children: Default::default(),
            widget_name: PlayerMoneyHUDProps::default().get_name(),
        }
    }
}

// Our own version of widget_update that handles resource change events.
pub fn widget_update_with_money<
    Props: PartialEq + Component + Clone,
    State: PartialEq + Component + Clone,
>(
    In((entity, previous_entity)): In<(Entity, Entity)>,
    widget_context: Res<KayakWidgetContext>,
    player_money: Res<PlayerMoney>,
    widget_param: WidgetParam<Props, State>,
) -> bool {
    widget_param.has_changed(&widget_context, entity, previous_entity) || player_money.is_changed()
}

pub fn player_money_hud_render(
    In(entity): In<Entity>,
    mut query: Query<(&mut PlayerMoneyHUDProps, &mut ComputedStyles, &KStyle)>,
    player_money: Res<PlayerMoney>,
) -> bool {
    if let Ok((mut props, mut computed_styles, style)) = query.get_mut(entity) {
        props.current_money = player_money.amount;
        *computed_styles = KStyle {
            color: Color::BLACK.into(),
            render_command: StyleProp::Value(RenderCommand::Text {
                content: format!("Money: {:}", props.current_money),
                alignment: Alignment::Start,
                word_wrap: false,
                subpixel: false,
                text_layout: TextLayout::default(),
                properties: TextProperties::default()
            }),
            ..Default::default()
        }
        .with_style(style)
        .into();
    }
    true
}

pub fn widget_update_with_player_state<
Props: PartialEq + Component + Clone,
KState: PartialEq + Component + Clone,
>(
    In((entity, previous_entity)): In<(Entity, Entity)>,
    widget_context: Res<KayakWidgetContext>,
    widget_param: WidgetParam<Props, KState>,
    player_state: Res<State<PlayerState>>,
) -> bool {
    widget_param.has_changed(&widget_context, entity, previous_entity) || player_state.is_changed()
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct AssembliesHudProps {
    pub image: Handle<Image>,
}
impl Widget for AssembliesHudProps {}

#[derive(Bundle)]
pub struct AssembliesHudBundle {
    pub props: AssembliesHudProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for AssembliesHudBundle {
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
            widget_name: AssembliesHudProps::default().get_name(),
        }
    }
}

pub fn assemblies_hud_render(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    mut query: Query<(&mut AssembliesHudProps, &mut ComputedStyles, &KStyle, &KChildren, &OnEvent)>,
    player_state: Res<State<PlayerState>>
) -> bool {
    if let Ok((mut props, mut computed_styles, base_style, base_children, base_on_event)) = query.get_mut(entity) {
        *computed_styles = KStyle {
            ..Default::default()
        }
        .with_style(base_style)
        .into();

        let parent_id = Some(entity);
        rsx!(
            <ElementBundle>
                {if player_state.get() == &PlayerState::Assemblies {
                    constructor! {
                        <NinePatchBundle
                            nine_patch={NinePatch {
                                handle: props.image.clone(),
                                border: Edge::all(0.0),
                            }}
                            styles={base_style.clone()}
                            children={base_children.clone()}
                            on_event={base_on_event.clone()}
                        />
                    }
                }}
            </ElementBundle>
        );
    }
    true
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct ImageButtonProps {
    pub image: Handle<Image>,
    pub hover_image: Handle<Image>,
    pub selected_image: Handle<Image>
}
impl Widget for ImageButtonProps {}

#[derive(Bundle)]
pub struct ImageButtonBundle {
    pub props: ImageButtonProps,
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
    mut query: Query<(&mut ImageButtonProps, &mut ComputedStyles, &KStyle)>,
) -> bool {
    if let Ok((props, mut computed_styles, style)) = query.get_mut(entity) {
        *computed_styles = KStyle::default()
            .with_style(style)
            .into();

        let parent_id = Some(entity);
        rsx!(
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: props.image.clone(),
                    border: Edge::all(0.0),
                }}
                on_event={OnEvent::new(
                    move |In(_entity): In<Entity>, event: ResMut<KEvent>, mut placement_state: ResMut<NextState<PlacementState>> | {
                        if let EventType::Hover(_) = event.event_type {
                            props.image = props.hover_image.clone();
                        }
                    }
                )}
            />
        );
    }
    true
}
