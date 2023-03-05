use std::{marker::PhantomData, ops::Neg};

use super::components::*;
use super::ScrollSystemSet;
use super::scroll_indicator::ScrollIndicatorWidget;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    ui::RelativeCursorPosition,
};

use super::styles::{scroll_content_style, scroll_wrapper_style};

#[derive(Component, Clone, Debug)]
pub struct ScrollContentWidget {
    pub scroll_direction: ScrollDirection,
    pub current_offset: Vec2,
}

#[derive(Component, Clone, Debug)]
pub struct ScrollContentElement {
    pub scroll_content: Entity,
    pub scroll_wrapper: Entity,
}

pub struct ScrollContentPlugin;

impl Plugin for ScrollContentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UiEvent<ScrollContentWidget, ScrollInput>>()
            .add_event::<UiEvent<ScrollContentWidget, ScrollIndicatorEnabled>>()
            .add_system(init_scroll_content.in_set(ScrollSystemSet::Create))
            .add_system(scroll_content_scroll_publisher.in_set(ScrollSystemSet::Interact))
            .add_system(scroll_content_indicator_publisher.in_set(ScrollSystemSet::Interact))
            .add_system(scroll_content_scroll_subscriber.in_set(ScrollSystemSet::Update))
            .add_system(scroll_content_indcator_enabled_subscriber.in_set(ScrollSystemSet::Update))
            .add_system(scroll_from_scroll_indicator.in_set(ScrollSystemSet::Update))
            .add_system(constrain_scroll_content.in_set(ScrollSystemSet::Constrain))
            .add_system(extract_scroll_content.in_set(ScrollSystemSet::Extract));
    }
}

const PIXELS_SCROLLED_PER_LINE: f32 = 15.0;

pub fn init_scroll_content(
    q_uninitialized_widgets: Query<(Entity, &ScrollContentWidget), Without<ScrollContentElement>>,
    mut commands: Commands,
) {
    for (widget_entity, widget) in q_uninitialized_widgets.iter() {
        let scroll_content = commands
            .spawn(NodeBundle {
                style: scroll_content_style(widget.scroll_direction),
                background_color: BackgroundColor(Color::rgb(0.25, 0.25, 0.25)),
                ..default()
            })
            .insert(ControlledByElement {
                element: widget_entity,
                kind: PhantomData::<ScrollContentElement>,
            })
            .id();

        let scroll_wrapper = commands
            .spawn(NodeBundle {
                style: scroll_wrapper_style(widget.scroll_direction),
                background_color: BackgroundColor(Color::rgb(0.15, 0.15, 0.15)),
                ..default()
            })
            .insert(Interaction::None)
            .insert(RelativeCursorPosition::default())
            .insert(ControlledByElement {
                element: widget_entity,
                kind: PhantomData::<ScrollContentElement>,
            })
            .id();

        commands.entity(scroll_wrapper).add_child(scroll_content);
        commands.entity(widget_entity).insert(ScrollContentElement {
            scroll_content,
            scroll_wrapper,
        });

        for n in 1..51 {
            let scroll_content_row = commands.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            }).id();

            commands
                .entity(scroll_content)
                .add_child(scroll_content_row);

            for i in 1..51 {
                generate_filler_content(&mut commands, scroll_content_row, n * i, 51.0 * 51.0);
            }
        }
    }
}

pub fn extract_scroll_content(
    q_scroll_content_elements: Query<
        (&ScrollContentElement, &ScrollContentWidget),
        Or<(Changed<ScrollContentWidget>, Added<ScrollContentElement>)>,
    >,
    mut q_nodes: Query<&mut Style>,
) {
    for (elem, widget) in q_scroll_content_elements.iter() {
        let mut scroll_content_style = q_nodes
            .get_mut(elem.scroll_content)
            .expect("ScrollContentElement.scroll_content should have been a (Style, Node).");

        let scroll_margins = widget.current_offset.neg();

        match widget.scroll_direction {
            ScrollDirection::Vertical => {
                scroll_content_style.margin.top = Val::Px(scroll_margins.y);
            }
            ScrollDirection::Horizontal => {
                scroll_content_style.margin.left = Val::Px(scroll_margins.x);
            }
            ScrollDirection::Both => {
                scroll_content_style.margin.top = Val::Px(scroll_margins.y);
                scroll_content_style.margin.left = Val::Px(scroll_margins.x);
            }
            ScrollDirection::Neither => {}
        }
    }
}

pub fn scroll_content_scroll_publisher(
    q_scroll_content_nodes: Query<(&ControlledByElement<ScrollContentElement>, &Interaction)>,
    mut er_mousewheel: EventReader<MouseWheel>,
    mut ew_scroll_content_mousewheel: EventWriter<UiEvent<ScrollContentWidget, ScrollInput>>,
) {
    for (controller, scroll_content_interaction) in q_scroll_content_nodes.iter() {
        if *scroll_content_interaction != Interaction::Hovered {
            continue;
        }

        for mousewheel_event in er_mousewheel.iter() {
            let distance = match mousewheel_event.unit {
                MouseScrollUnit::Line => {
                    PIXELS_SCROLLED_PER_LINE
                        * Vec2 {
                            x: mousewheel_event.x,
                            y: mousewheel_event.y,
                        }
                }
                MouseScrollUnit::Pixel => Vec2 {
                    x: mousewheel_event.x,
                    y: mousewheel_event.y,
                },
            };

            ew_scroll_content_mousewheel.send(UiEvent {
                widget: controller.element,
                action: ScrollInput { distance },
                element_kind: PhantomData::<ScrollContentWidget>,
            });
        }
    }
}

pub fn scroll_content_indicator_publisher(
    q_scroll_content_nodes: Query<(&ControlledByElement<ScrollContentElement>, &Interaction)>,
    mouse_button_input: ResMut<Input<MouseButton>>,
    window_query: Query<&Window>,
    mut ew_scroll_content_indicator_enabled: EventWriter<
        UiEvent<ScrollContentWidget, ScrollIndicatorEnabled>,
    >
) {

    for (controller, scroll_content_interaction) in q_scroll_content_nodes.iter() {
        if *scroll_content_interaction != Interaction::Hovered {
            continue;
        }

        if mouse_button_input.just_pressed(MouseButton::Middle) {
            let window = window_query.single();

            let coords = if let Some(world_position) = window.cursor_position() {
                Vec2 {
                    x: world_position.x,
                    y: window.height() - world_position.y,
                }
            } else {
                return;
            };

            ew_scroll_content_indicator_enabled.send(UiEvent {
                widget: controller.element,
                action: ScrollIndicatorEnabled { position: coords },
                element_kind: PhantomData::<ScrollContentWidget>,
            });

            return;
        }
    }
}

pub fn scroll_content_scroll_subscriber(
    mut er_scroll_content_scroll: EventReader<UiEvent<ScrollContentWidget, ScrollInput>>,
    mut q_scroll_content_widgets: Query<&mut ScrollContentWidget, With<ScrollContentElement>>,
) {
    for event in er_scroll_content_scroll.iter() {
        let mut widget = q_scroll_content_widgets
            .get_mut(event.widget)
            .expect("ScrollContentWidget event should have contained a ScrollContentWidget.");

        widget.current_offset -= event.action.distance;
    }
}

pub fn scroll_content_indcator_enabled_subscriber(
    mut er_scroll_content_indicator_enabled: EventReader<UiEvent<ScrollContentWidget, ScrollIndicatorEnabled>>,
    q_scroll_content_widgets: Query<&ScrollContentWidget, With<ScrollContentElement>>,
    mut commands: Commands,
) {
    for event in er_scroll_content_indicator_enabled.iter() {
        let widget = q_scroll_content_widgets
            .get(event.widget)
            .expect("ScrollContentWidget event should have contained a ScrollContentWidget.");

        commands.spawn(ScrollIndicatorWidget {
            scroll_direction: widget.scroll_direction,
            absolute_position: event.action.position - Vec2 { x: 16.0, y: 16.0 },   // dirty, get the actual node halfsize instead
            scroll_content: event.widget,
        });
    }
}

pub fn scroll_from_scroll_indicator(
    window_query: Query<&Window>,
    q_scroll_indicator: Query<&ScrollIndicatorWidget>,
    mut ew_scroll_content_scroll_input: EventWriter<UiEvent<ScrollContentWidget, ScrollInput>>
) {
    let window = window_query.single();
    
    let cursor_coords = if let Some(world_position) = window.cursor_position() {
        world_position
    } else {
        return;
    };

    for widget in q_scroll_indicator.iter()
    {
        let node_half_size = Vec2 { x: 16.0, y: -16.0 }; // dirty, get the actual node halfsize instead
        let indicator_coords = Vec2 {
            x: widget.absolute_position.x,
            y: window.height() - widget.absolute_position.y
        } + node_half_size;
        

        let easing = CubicBezierEasing::new([0.35, 0.01], [0.97, 0.79]);
        let delta = cursor_coords - indicator_coords;

        let window_dimensions = Vec2 { x: window.width(), y: window.height() };
        let normalized_delta = delta / window_dimensions;

        let eased_delta_y =  delta.y * easing.ease(normalized_delta.y.abs().max(0.2));
        let eased_delta_x = delta.x * easing.ease(normalized_delta.x.abs().max(0.2));

        ew_scroll_content_scroll_input.send(UiEvent {
            widget: widget.scroll_content,
            action: ScrollInput { distance: Vec2 { x: -eased_delta_x, y: eased_delta_y } },
            element_kind: PhantomData::<ScrollContentWidget>,
        });
    }
}

pub fn constrain_scroll_content(
    mut q_scroll_content_widget: Query<
        (&mut ScrollContentWidget, &ScrollContentElement),
        Changed<ScrollContentWidget>,
    >,
    q_nodes: Query<&Node>,
) {
    for (mut widget, element) in q_scroll_content_widget.iter_mut() {
        let scroll_content_node = q_nodes
            .get(element.scroll_content)
            .expect("ScrollContentElement.scroll_bar should have contained a Node.");
        let scroll_wrapper_node = q_nodes
            .get(element.scroll_wrapper)
            .expect("ScrollContentElement.scroll_handle should have contained a Node.");

        let scroll_threshold = scroll_content_node.size() - scroll_wrapper_node.size();

        let new_offset = scroll_threshold.min(widget.current_offset).max(Vec2::ZERO);

        if new_offset != widget.current_offset {
            widget.current_offset = new_offset;
        }
    }
}

fn generate_filler_content(commands: &mut Commands, parent: Entity, number: u16, biggest_number: f32) {
    let fixed_tick_node = NodeBundle {
        style: Style {
            min_size: Size {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
            },
            size: Size {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
            },
            margin: UiRect {
                top: Val::Px(2.0),
                left: Val::Px(2.0),
                ..default()
            },
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    commands.entity(parent).with_children(|container_panel_p| {
        container_panel_p
            .spawn(fixed_tick_node)
            .insert(BackgroundColor(Color::rgb(
                0.5,
                1.0 - (f32::from(number) / biggest_number),
                f32::from(number) / biggest_number,
            )));
    });
}
