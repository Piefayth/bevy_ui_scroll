use bevy::prelude::*;

use crate::scroll::components::*;

pub fn constrain_scroll_handles(
    scroll_bar_query: Query<(&ScrollBar, &Node)>,
    mut scroll_handle_query: Query<(&mut ScrollHandle, &Node)>,
) {
    for (scroll_bar, scroll_bar_node) in scroll_bar_query.iter() {
        let (mut scroll_handle, scroll_handle_node) =
            if let Ok(handle) = scroll_handle_query.get_mut(scroll_bar.scroll_handle) {
                handle
            } else {
                // no changed scroll handles
                continue;
            };

        let scroll_handle_size = scroll_handle_node.size();
        let scroll_bar_size = scroll_bar_node.size();

        let scroll_clamp = scroll_bar_size - scroll_handle_size;

        let scroll_handle = scroll_handle.bypass_change_detection();
        match scroll_bar.orientation {
            ScrollBarOrientation::Vertical => {
                let y_offset = scroll_clamp.y.min(scroll_handle.position).max(0.0);
                if scroll_handle.position != y_offset {
                    scroll_handle.position = y_offset;
                }
            }
            ScrollBarOrientation::Horizontal => {
                let x_offset = scroll_clamp.x.min(scroll_handle.position).max(0.0);
                if scroll_handle.position != x_offset {
                    scroll_handle.position = x_offset;
                }
            }
        }
    }
}

pub fn constrain_scroll_content(
    mut changed_scroll_content_query: Query<
        (&mut ScrollContent, &Node, &Parent),
    >,
    nodes_query: Query<&Node>,
) {
    for (mut scroll_content, scroll_content_node, scroll_parent) in
        changed_scroll_content_query.iter_mut()
    {
        let root_node = nodes_query
            .get(scroll_parent.get())
            .expect("ScrollContent should not be orphaned in the UI hierarchy.");

        let scroll_threshold = scroll_content_node.size() - root_node.size();

        let new_offset = scroll_threshold
            .min(scroll_content.current_offset)
            .max(Vec2::ZERO);

        let scroll_content = scroll_content.bypass_change_detection();
        if new_offset != scroll_content.current_offset {
            scroll_content.current_offset = new_offset;
        }
    }
}