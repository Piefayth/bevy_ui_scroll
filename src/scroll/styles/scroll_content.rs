use bevy::prelude::*;
use crate::scroll::components::*;

pub fn scroll_container_style(scroll_direction: ScrollDirection) -> Style {
    return Style {
        min_size: Size {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            ..default()
        },
        flex_direction: match scroll_direction {
            ScrollDirection::Vertical => FlexDirection::Column,
            ScrollDirection::Horizontal => FlexDirection::DEFAULT,
            ScrollDirection::Both => FlexDirection::Column,
            ScrollDirection::Neither => FlexDirection::DEFAULT,
        },
        flex_grow: 1.0,
        flex_shrink: 0.0,
        overflow: Overflow::Hidden,
        ..default()
    };
}