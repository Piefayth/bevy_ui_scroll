use bevy::prelude::*;

use crate::scroll::components::*;
use std::ops::Neg;

pub fn extract_scroll_content_styles(
    mut changed_scroll_content_query: Query<(&ScrollContent, &mut Style)>,
) {
    for (scroll_content, mut scroll_content_style) in changed_scroll_content_query.iter_mut() {
        let scroll_margins = scroll_content.current_offset.neg();

        match scroll_content.direction {
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

pub fn extract_scroll_handle_styles(
    scroll_bar_query: Query<&ScrollBar>,
    mut scroll_handle_query: Query<(&ScrollHandle, &mut Style)>,
) {
    for scroll_bar in scroll_bar_query.iter() {
        if !scroll_handle_query.contains(scroll_bar.scroll_handle) {
            continue;
        };

        let (scroll_handle, mut scroll_handle_style) = scroll_handle_query
            .get_mut(scroll_bar.scroll_handle)
            .unwrap();

        scroll_handle_style.size = match scroll_bar.orientation {
            ScrollBarOrientation::Vertical => Size {
                width: Val::Px(scroll_handle.girth),
                height: Val::Px(scroll_handle.length),
            },
            ScrollBarOrientation::Horizontal => Size {
                height: Val::Px(scroll_handle.girth),
                width: Val::Px(scroll_handle.length),
            },
        };

        scroll_handle_style.display = match scroll_handle.visible {
            true => Display::DEFAULT,
            false => Display::None,
        };

        match scroll_bar.orientation {
            ScrollBarOrientation::Vertical => {
                scroll_handle_style.margin.top = Val::Px(scroll_handle.position);
            }
            ScrollBarOrientation::Horizontal => {
                scroll_handle_style.margin.left = Val::Px(scroll_handle.position);
            }
        }
    }
}
