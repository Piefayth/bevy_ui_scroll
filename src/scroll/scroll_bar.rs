use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

use super::components::*;
use super::styles::scroll_bar_style;
use super::styles::scroll_handle_style;
use super::ScrollSystemSet;

use std::marker::PhantomData;

#[derive(Component, Clone, Debug)]
pub struct ScrollBarWidget {
    pub orientation: ScrollBarOrientation,
    pub girth: Val,
    pub handle_girth: Val,
    pub handle_length: Val,
    pub handle_visibility: bool,
    pub current_offset: f32,
}

#[derive(Component, Clone, Debug)]
pub struct ScrollBarElement {
    pub scroll_bar: Entity,
    pub scroll_handle: Entity,
}

pub struct ScrollBarPlugin;

impl Plugin for ScrollBarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UiEvent<ScrollBarWidget, PressInput>>()
            .add_system(init_scroll_bar.in_set(ScrollSystemSet::Create))
            .add_system(scroll_bar_left_click_publisher.in_set(ScrollSystemSet::Interact))
            .add_system(scroll_bar_left_click_subscriber.in_set(ScrollSystemSet::Update))
            .add_system(constrain_scroll_bar.in_set(ScrollSystemSet::Constrain))
            .add_system(extract_scroll_bar.in_set(ScrollSystemSet::Extract));
    }
}

pub fn init_scroll_bar(
    q_uninitialized_widgets: Query<(Entity, &ScrollBarWidget), Without<ScrollBarElement>>,
    mut commands: Commands,
) {
    for (widget_entity, widget) in q_uninitialized_widgets.iter() {
        let scroll_handle = commands
            .spawn(NodeBundle {
                style: scroll_handle_style(),
                background_color: BackgroundColor(Color::rgb(1.0, 1.0, 0.0)),
                ..default()
            })
            .insert(ControlledByElement {
                element: widget_entity,
                kind: PhantomData::<ScrollBarElement>,
            })
            .id();

        let scroll_bar = commands
            .spawn(NodeBundle {
                style: scroll_bar_style(widget.orientation, widget.girth),
                background_color: BackgroundColor(Color::rgb(1.0, 1.0, 0.0)),
                ..default()
            })
            .insert(BackgroundColor(Color::rgb(0.15, 0.15, 0.15)))
            .insert(Interaction::None)
            .insert(RelativeCursorPosition::default())
            .insert(ControlledByElement {
                element: widget_entity,
                kind: PhantomData::<ScrollBarElement>,
            })
            .id();

        commands.entity(scroll_bar).add_child(scroll_handle);

        commands.entity(widget_entity).insert(ScrollBarElement {
            scroll_bar,
            scroll_handle,
        });
    }
}

pub fn scroll_bar_left_click_publisher(
    q_scroll_bar_nodes: Query<(
        &ControlledByElement<ScrollBarElement>,
        &Interaction,
        &RelativeCursorPosition,
        &Node,
    )>,
    q_scroll_bar_elements: Query<Entity, With<ScrollBarElement>>,
    mut ew_scroll_bar_click: EventWriter<UiEvent<ScrollBarWidget, PressInput>>,
) {
    for (controller, scroll_bar_interaction, scroll_bar_rel_cursor, scroll_bar_node) in
        q_scroll_bar_nodes.iter()
    {
        if *scroll_bar_interaction != Interaction::Clicked {
            continue;
        }

        let cursor_pos = if let Some(cursor_pos) = scroll_bar_rel_cursor.normalized {
            cursor_pos
        } else {
            continue;
        };

        let scroll_bar_cursor_position_px = cursor_pos * scroll_bar_node.size();
        let scroll_bar_entity = q_scroll_bar_elements.get(controller.element).expect(
            "ControlledByElement<ScrollBarElement> should have referenced a ScrollBarElement.",
        );

        ew_scroll_bar_click.send(UiEvent {
            widget: scroll_bar_entity,
            action: PressInput {
                position: scroll_bar_cursor_position_px,
            },
            element_kind: PhantomData::<ScrollBarWidget>,
        });
    }
}

pub fn scroll_bar_left_click_subscriber(
    mut er_scroll_bar_event: EventReader<UiEvent<ScrollBarWidget, PressInput>>,
    mut q_scroll_bar_widgets: Query<&mut ScrollBarWidget, With<ScrollBarElement>>,
) {
    for event in er_scroll_bar_event.iter() {
        let mut widget = q_scroll_bar_widgets
            .get_mut(event.widget)
            .expect("ScrollBarWidget event should have contained a ScrollBarWidget.");

        let handle_length = match widget.handle_length {
            Val::Undefined => 0.0,
            Val::Auto => 0.0,
            Val::Px(num) => num,
            Val::Percent(num) => num,
        };

        let new_offset = match widget.orientation {
            ScrollBarOrientation::Vertical => event.action.position.y - (handle_length / 2.0),
            ScrollBarOrientation::Horizontal => event.action.position.x - (handle_length / 2.0),
        };

        if widget.current_offset != new_offset {
            widget.current_offset = new_offset;
        }
    }
}

pub fn constrain_scroll_bar(
    mut q_scroll_bar_widget: Query<
        (&mut ScrollBarWidget, &ScrollBarElement),
        Changed<ScrollBarWidget>,
    >,
    q_nodes: Query<&Node>,
) {
    for (mut widget, element) in q_scroll_bar_widget.iter_mut() {
        let scroll_bar_node = q_nodes
            .get(element.scroll_bar)
            .expect("ScrollBarElement.scroll_bar should have contained a Node.");
        let scroll_handle_node = q_nodes
            .get(element.scroll_handle)
            .expect("ScrollBarElement.scroll_handle should have contained a Node.");

        let scroll_clamp = scroll_bar_node.size() - scroll_handle_node.size();

        let clamped_offset = match widget.orientation {
            ScrollBarOrientation::Vertical => scroll_clamp.y.min(widget.current_offset).max(0.0),
            ScrollBarOrientation::Horizontal => scroll_clamp.x.min(widget.current_offset).max(0.0),
        };

        if widget.current_offset != clamped_offset {
            widget.current_offset = clamped_offset;
        }
    }
}

pub fn extract_scroll_bar(
    q_scroll_bar_elements: Query<
        (&ScrollBarElement, &ScrollBarWidget),
        Or<(Changed<ScrollBarWidget>, Added<ScrollBarElement>)>,
    >,
    mut q_nodes: Query<&mut Style>,
) {
    for (elem, widget) in q_scroll_bar_elements.iter() {
        let mut scroll_handle_style = q_nodes
            .get_mut(elem.scroll_handle)
            .expect("ScrollBarElement.scroll_handle should have been a (Style, Node).");

        scroll_handle_style.size = match widget.orientation {
            ScrollBarOrientation::Vertical => Size {
                width: widget.handle_girth,
                height: widget.handle_length,
            },
            ScrollBarOrientation::Horizontal => Size {
                height: widget.handle_girth,
                width: widget.handle_length,
            },
        };

        scroll_handle_style.display = match widget.handle_visibility {
            true => Display::DEFAULT,
            false => Display::None,
        };

        match widget.orientation {
            ScrollBarOrientation::Vertical => {
                scroll_handle_style.margin.top = Val::Px(widget.current_offset);
            }
            ScrollBarOrientation::Horizontal => {
                scroll_handle_style.margin.left = Val::Px(widget.current_offset);
            }
        }
    }
}
