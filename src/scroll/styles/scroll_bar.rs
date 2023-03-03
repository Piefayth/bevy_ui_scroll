use bevy::prelude::*;
use crate::scroll::components::*;

pub fn scroll_bar_style(scroll_direction: ScrollBarOrientation, girth: Val) -> Style {
    let size = match scroll_direction {
        ScrollBarOrientation::Vertical => Size {
            height: Val::Percent(100.0),
            width: girth,
            ..default()
        },
        ScrollBarOrientation::Horizontal => Size {
            width: Val::Percent(100.0),
            height: girth,
            ..default()
        },
    };

    return Style {
        position_type: PositionType::Absolute,
        size,
        max_size: size,
        flex_direction: match scroll_direction {
            ScrollBarOrientation::Vertical => FlexDirection::Column,
            ScrollBarOrientation::Horizontal => FlexDirection::Row,
        },
        align_items: AlignItems::Center,
        ..default()
    };
}