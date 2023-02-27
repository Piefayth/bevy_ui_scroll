use bevy::prelude::*;

use crate::scroll::components::*;

pub fn scroll_content_that_controls_scroll_handles(
    changed_scroll_content_query: Query<
        (&ScrollContent, &Node, &ControlledByScrollBars, &Parent),
        Or<(Changed<ScrollContent>, Changed<Node>)>,
    >,
    scroll_bars_query: Query<(&ScrollBar, &Node)>,
    nodes_query: Query<&Node>,
    mut scroll_handles_query: Query<&mut ScrollHandle>,
) {
    for (scroll_content, scroll_content_node, controllers, scroll_parent) in
        changed_scroll_content_query.iter()
    {
        for scroll_bar_entity in controllers.scroll_bars.iter() {
            let (scroll_bar, scroll_bar_node) = scroll_bars_query
                    .get(*scroll_bar_entity)
                    .expect("ControlledByScrollBars.scroll_bars should have contained Entities with ScrollBar components.");

            let mut _scroll_handle = scroll_handles_query
                    .get_mut(scroll_bar.scroll_handle)
                    .expect("ScrollBar.scroll_handle should have contained a reference to an Entity with a ScrollHandle component.");
            let mut scroll_handle = _scroll_handle.bypass_change_detection();

            let root_node = nodes_query
                .get(scroll_parent.get())
                .expect("ScrollContent should not be orphaned in the UI hierarchy.");

            let content_size = scroll_content_node.size();
            let root_size = root_node.size();
            let scroll_bar_size = scroll_bar_node.size();

            let scroll_threshold = content_size - root_node.size();
            let ratio = scroll_bar_size / scroll_content_node.size();

            match scroll_bar.orientation {
                ScrollBarOrientation::Vertical => {
                    if scroll_threshold.y != 0.0 {
                        scroll_handle.visible = true;

                        let handle_length_ratio = root_size.y / content_size.y;
                        let handle_length = handle_length_ratio * scroll_bar_size.y;

                        scroll_handle.length = handle_length;
                    } else {
                        scroll_handle.visible = false;
                    }

                    scroll_handle.position = scroll_content.current_offset.y * ratio.y;
                }
                ScrollBarOrientation::Horizontal => {
                    if scroll_threshold.x != 0.0 {
                        scroll_handle.visible = true;

                        let handle_length_ratio = root_size.x / content_size.x;
                        let handle_length = handle_length_ratio * scroll_bar_size.x;

                        scroll_handle.length = handle_length;
                    } else {
                        scroll_handle.visible = false;
                    }

                    scroll_handle.position = scroll_content.current_offset.x * ratio.x;
                }
            }
        }
    }
}

pub fn scroll_handles_that_control_scroll_content(
    changed_scroll_handles: Query<
        (&ScrollHandle, &ControlsScrollContent, &Parent),
        Or<(Changed<ScrollHandle>, Changed<Node>)>,
    >,
    scroll_bars_query: Query<(&ScrollBar, &Node)>,
    mut scroll_content_query: Query<(&mut ScrollContent, &Node)>,
) {
    for (scroll_handle, controller, scroll_handle_parent) in changed_scroll_handles.iter() {
        let (mut scroll_content, scroll_content_node) = scroll_content_query
            .get_mut(controller.scroll_content)
            .expect("ControlsScrollContent.scroll_content should have contained an Entity with a ScrollContent component.");

        let quiet_update_scroll_content = scroll_content.bypass_change_detection();

        let (scroll_bar, scroll_bar_node) = scroll_bars_query
            .get(scroll_handle_parent.get())
            .expect("ScrollHandle should have had a ScrollBar parent.");

        let ratio = scroll_bar_node.size() / scroll_content_node.size();

        match scroll_bar.orientation {
            ScrollBarOrientation::Vertical => {
                quiet_update_scroll_content.current_offset.y = scroll_handle.position / ratio.y;
            }
            ScrollBarOrientation::Horizontal => {
                quiet_update_scroll_content.current_offset.x = scroll_handle.position / ratio.x;
            }
        };
    }
}