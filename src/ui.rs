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
        ClockHUDProps::default().get_name(),
        widget_update_on_tick::<ClockHUDProps, EmptyState>,
        clock_hud_render,
    );
    widget_context.add_widget_system(
        ImageButtonProps::default().get_name(),
        widget_update::<ImageButtonProps, EmptyState>,
        image_button_render,
    );
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
        NightUIProps::default().get_name(),
        widget_update_with_day_state::<NightUIProps, EmptyState>,
        night_ui_render,
    );
    widget_context.add_widget_system(
        HUDContainerProps::default().get_name(),
        widget_update_with_day_state::<HUDContainerProps, EmptyState>,
        hud_container_render,
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
