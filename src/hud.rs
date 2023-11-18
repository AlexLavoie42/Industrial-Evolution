use kayak_ui::prelude::kayak_font::{TextLayout, TextProperties};

use crate::*;

pub fn hud_setup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
    q_camera_entity: Query<Entity, With<Camera>>,
) {
    font_mapping.set_default(asset_server.load("roboto.kttf"));

    let camera_entity = q_camera_entity.single();
    let mut widget_context = KayakRootContext::new(camera_entity);
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    widget_context.add_widget_data::<PlayerMoneyHUDProps, EmptyState>();

    // Next we need to add the systems
    widget_context.add_widget_system(
        // We are registering these systems with a specific WidgetName.
        PlayerMoneyHUDProps::default().get_name(),
        // widget_update auto diffs props and state.
        // Optionally if you have context you can use: widget_update_with_context
        // otherwise you will need to create your own widget update system!
        widget_update::<PlayerMoneyHUDProps, EmptyState>,
        // Add our render system!
        player_money_hud_render,
    );
    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <PlayerMoneyHUDBundle/>
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
    // This allows us to hook into on click events!
    pub on_event: OnEvent,
    // Widget name is required by Kayak UI!
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
            on_event: Default::default(),
            widget_name: PlayerMoneyHUDProps::default().get_name(),
        }
    }
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
        return true;
    }
    false
}
