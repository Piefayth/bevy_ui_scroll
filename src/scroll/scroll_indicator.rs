use std::marker::PhantomData;

use super::components::*;
use super::ScrollSystemSet;
use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct ScrollIndicatorWidget {
    pub scroll_direction: ScrollDirection,
    pub absolute_position: Vec2,
    pub scroll_content: Entity,
}

#[derive(Component, Clone, Debug)]
pub struct ScrollIndicatorElement {
    scroll_indicator: Entity,
}

pub struct ScrollIndicatorPlugin;

impl Plugin for ScrollIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UiEvent<ScrollIndicatorWidget, ScrollIndicatorDisabled>>()
            .add_system(init_scroll_indicator.in_set(ScrollSystemSet::Create))
            .add_system(scroll_indicator_disabled_publisher.in_set(ScrollSystemSet::Interact))
            .add_system(scroll_indicator_disabled_subscriber.in_set(ScrollSystemSet::Update))
            .add_system(delete_scroll_indicator.in_set(ScrollSystemSet::Extract));
    }
}

pub fn init_scroll_indicator(
    q_uninitialized_widgets: Query<
        (Entity, &ScrollIndicatorWidget),
        Without<ScrollIndicatorElement>,
    >,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (entity, widget) in q_uninitialized_widgets.iter() {
        let texture = match widget.scroll_direction {
            ScrollDirection::Vertical => asset_server.load("sprites/scroll_cursor_vert.png"),
            ScrollDirection::Horizontal => asset_server.load("sprites/scroll_cursor_horiz.png"),
            ScrollDirection::Both => asset_server.load("sprites/scroll_cursor_multi.png"),
            ScrollDirection::Neither => todo!(),
        };

        let scroll_indicator = commands
            .spawn(ImageBundle {
                image: UiImage {
                    texture,
                    ..default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(widget.absolute_position.x),
                        right: Val::Px(0.0),
                        top: Val::Px(widget.absolute_position.y),
                        bottom: Val::Px(0.0),
                    },
                    size: Size {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                    },
                    ..default()
                },
                ..default()
            })
            .insert(ControlledByElement {
                element: entity,
                kind: PhantomData::<ScrollIndicatorElement>,
            })
            .id();

        commands
            .entity(entity)
            .insert(ScrollIndicatorElement { scroll_indicator });
    }
}

pub fn scroll_indicator_disabled_publisher(
    mut ew_scroll_content_indicator_disabled: EventWriter<
        UiEvent<ScrollIndicatorWidget, ScrollIndicatorDisabled>
    >,
    q_scroll_indicators: Query<Entity, With<ScrollIndicatorWidget>>,
    mouse_button_input: ResMut<Input<MouseButton>>,
) {
    if mouse_button_input.just_released(MouseButton::Middle) {
        for entity in q_scroll_indicators.iter() {
            ew_scroll_content_indicator_disabled.send(UiEvent {
                widget: entity,
                action: ScrollIndicatorDisabled,
                element_kind: PhantomData::<ScrollIndicatorWidget>,
            });
        }
    }
}

pub fn scroll_indicator_disabled_subscriber(
    mut er_scroll_content_indicator_disabled: EventReader<UiEvent<ScrollIndicatorWidget, ScrollIndicatorDisabled>>,
    mut commands: Commands,
) {
    for event in er_scroll_content_indicator_disabled.iter() {
        commands.entity(event.widget).remove::<ScrollIndicatorWidget>();
    }
}

pub fn delete_scroll_indicator(
    q_removed_widgets: Query<(Entity, &ScrollIndicatorElement), Without<ScrollIndicatorWidget>>,
    mut commands: Commands,
) {
    for (entity, element) in q_removed_widgets.iter() {
        commands.entity(element.scroll_indicator).despawn();
        commands.entity(entity).despawn();
    }
}
