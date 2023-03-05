use bevy::prelude::*;

use super::{
    components::{ScrollBarOrientation, ScrollDirection},
    scroll_bar::{ScrollBarElement, ScrollBarWidget},
    scroll_content::{ScrollContentElement, ScrollContentWidget},
    PropagateSchedule, ScrollSystemSet,
};

pub struct ScrollContainerPlugin;

impl Plugin for ScrollContainerPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_system(synchronize_bar_with_content.in_schedule(PropagateSchedule))
            .add_system(synchronize_bar_with_content.in_set(ScrollSystemSet::Propagate))
            .add_system(init_scroll_container.in_set(ScrollSystemSet::Create));
    }
}

#[derive(Component, Clone, Debug)]
pub struct ScrollContainerWidget {
    pub scroll_direction: ScrollDirection,
}

#[derive(Component, Clone, Debug)]
pub struct ScrollContainerElement {
    pub scroll_bar_widgets: Vec<Entity>,
    pub scroll_content_widget: Entity,
}

pub fn init_scroll_container(
    q_uninitialized_widgets: Query<
        (Entity, &ScrollContainerWidget),
        Without<ScrollContainerElement>,
    >,
    mut commands: Commands,
) {
    for (entity, widget) in q_uninitialized_widgets.iter() {
        let v_scroll_bar_widget = commands
            .spawn(ScrollBarWidget {
                orientation: ScrollBarOrientation::Vertical,
                girth: Val::Px(20.0),
                handle_girth: Val::Px(18.0),
                handle_length: Val::Px(40.0),
                handle_visibility: true,
                current_offset: 0.0,
            })
            .id();

        let h_scroll_bar_widget = commands
            .spawn(ScrollBarWidget {
                orientation: ScrollBarOrientation::Horizontal,
                girth: Val::Px(20.0),
                handle_girth: Val::Px(18.0),
                handle_length: Val::Px(40.0),
                handle_visibility: true,
                current_offset: 0.0,
            })
            .id();

        let scroll_bar_widgets = match widget.scroll_direction {
            ScrollDirection::Vertical => vec![v_scroll_bar_widget],
            ScrollDirection::Horizontal => vec![h_scroll_bar_widget],
            ScrollDirection::Both => vec![v_scroll_bar_widget, h_scroll_bar_widget],
            ScrollDirection::Neither => todo!(),
        };

        let scroll_content_widget = commands
            .spawn(ScrollContentWidget {
                scroll_direction: widget.scroll_direction,
                current_offset: Vec2::ZERO,
            })
            .id();

        for widget in &scroll_bar_widgets {
            commands.entity(entity).add_child(*widget);
        }

        commands
            .entity(entity)
            .add_child(scroll_content_widget)
            .insert(ScrollContainerElement {
                scroll_bar_widgets,
                scroll_content_widget,
            });
    }
}

pub fn synchronize_bar_with_content(
    q_container_widget: Query<&ScrollContainerElement>,
    mut pq_bar: ParamSet<(
        Query<
            (&ScrollBarWidget, &ScrollBarElement),
            Or<(
                Changed<ScrollBarWidget>,
                Added<ScrollBarElement>,
            )>,
        >,
        Query<(&mut ScrollBarWidget, &ScrollBarElement)>,
    )>,
    mut pq_content: ParamSet<(
        Query<
            (&ScrollContentWidget, &ScrollContentElement),
            Or<(
                Changed<ScrollContentWidget>,
                Added<ScrollContentElement>,
            )>,
        >,
        Query<(&mut ScrollContentWidget, &ScrollContentElement)>,
    )>,
    q_nodes: Query<&Node>,
) {
    // Apply scroll new bar offsets to content offsets
    for elem in q_container_widget.iter() {
        for scroll_bar_widget in elem.scroll_bar_widgets.iter() {
            if let Ok((changed_bar_widget, changed_bar_element)) =
                pq_bar.p0().get(*scroll_bar_widget)
            {
                if let Ok((mut stored_content_widget, stored_content_element)) =
                    pq_content.p1().get_mut(elem.scroll_content_widget)
                {
                    let bar_node = q_nodes
                        .get(changed_bar_element.scroll_bar)
                        .expect("shouldve been a node");
                    let content_node = q_nodes
                        .get(stored_content_element.scroll_content)
                        .expect("also shouldve been a node");
                    let handle_node = q_nodes
                        .get(changed_bar_element.scroll_handle)
                        .expect("also should have been a node");
                    let wrapper_node = q_nodes.get(stored_content_element.scroll_wrapper).unwrap();

                    let content_scrollable_space = content_node.size() - wrapper_node.size();
                    let bar_scrollable_space = bar_node.size() - handle_node.size();
                    let ratio = bar_scrollable_space / content_scrollable_space;

                    match changed_bar_widget.orientation {
                        ScrollBarOrientation::Vertical => {
                            let new_offset = changed_bar_widget.current_offset / ratio.y;
                            if (new_offset - stored_content_widget.current_offset.y).abs()
                                > f32::EPSILON
                            {
                                stored_content_widget.current_offset.y = new_offset;
                            }
                        }
                        ScrollBarOrientation::Horizontal => {
                            let new_offset = changed_bar_widget.current_offset / ratio.x;
                            if (new_offset - stored_content_widget.current_offset.x).abs()
                                > f32::EPSILON
                            {
                                stored_content_widget.current_offset.x = new_offset;
                            }
                        }
                    };
                }
            };
        }

        // Apply new scroll content offset to bar offsets
        if let Ok((changed_content_widget, changed_content_element)) =
            pq_content.p0().get(elem.scroll_content_widget)
        {
            for scroll_bar_widget in elem.scroll_bar_widgets.iter() {
                if let Ok((mut stored_bar_widget, stored_bar_element)) =
                    pq_bar.p1().get_mut(*scroll_bar_widget)
                {
                    let bar_node = q_nodes
                        .get(stored_bar_element.scroll_bar)
                        .expect("shouldve been a node");
                    let content_node = q_nodes
                        .get(changed_content_element.scroll_content)
                        .expect("also shouldve been a node");
                    let handle_node = q_nodes
                        .get(stored_bar_element.scroll_handle)
                        .expect("also should have been a node");
                    let wrapper_node = q_nodes.get(changed_content_element.scroll_wrapper).unwrap();

                    let content_scrollable_space = content_node.size() - wrapper_node.size();
                    let bar_scrollable_space = bar_node.size() - handle_node.size();
                    let bar_to_content_ratio = bar_scrollable_space / content_scrollable_space;
                    let wrapper_to_content_ratio = wrapper_node.size() / content_node.size();
                    let new_handle_size = wrapper_to_content_ratio * bar_node.size();

                    match stored_bar_widget.orientation {
                        ScrollBarOrientation::Vertical => {
                            let is_scrollable = content_scrollable_space.y > 0.0;

                            if is_scrollable != stored_bar_widget.handle_visibility {
                                stored_bar_widget.handle_visibility = is_scrollable;
                            }

                            if (new_handle_size.y - handle_node.size().y).abs() > f32::EPSILON {
                                stored_bar_widget.handle_length = Val::Px(new_handle_size.y);
                            }

                            let new_offset = changed_content_widget.current_offset.y * bar_to_content_ratio.y;
                            if (new_offset - stored_bar_widget.current_offset).abs() > f32::EPSILON
                            {
                                stored_bar_widget.current_offset = new_offset;
                            }
                        }
                        ScrollBarOrientation::Horizontal => {
                            let is_scrollable = content_scrollable_space.x > 0.0;

                            if is_scrollable != stored_bar_widget.handle_visibility {
                                stored_bar_widget.handle_visibility = is_scrollable;
                            }

                            
                            if (new_handle_size.x - handle_node.size().x).abs() > f32::EPSILON {
                                stored_bar_widget.handle_length = Val::Px(new_handle_size.x);
                            }

                            let new_offset = changed_content_widget.current_offset.x * bar_to_content_ratio.x;
                            if (new_offset - stored_bar_widget.current_offset).abs() > f32::EPSILON
                            {
                                stored_bar_widget.current_offset = new_offset;
                            }
                        }
                    }
                }
            }
        }
    }
}
