use bevy::prelude::*;
use crate::scroll::components::*;

pub fn scroll_bar_style(scroll_direction: ScrollBarOrientation, girth: Val) -> Style {
    let size = match scroll_direction {
        ScrollBarOrientation::Vertical => Size {
            height: Val::Percent(90.0),
            width: girth,
            ..default()
        },
        ScrollBarOrientation::Horizontal => Size {
            width: Val::Percent(90.0),
            height: girth,
            ..default()
        },
    };

    let align_self = match scroll_direction {
        ScrollBarOrientation::Vertical => AlignSelf::Center,
        ScrollBarOrientation::Horizontal => AlignSelf::Auto
    };

    let margin_left = match scroll_direction {
        ScrollBarOrientation::Vertical => Val::Auto,
        ScrollBarOrientation::Horizontal => Val::Percent(5.0),
    };

    return Style {
        position_type: PositionType::Absolute,
        size,
        max_size: size,
        position: UiRect { left: margin_left, ..default()},
        flex_direction: match scroll_direction {
            ScrollBarOrientation::Vertical => FlexDirection::Column,
            ScrollBarOrientation::Horizontal => FlexDirection::Row,
        },
        align_items: AlignItems::Center,
        align_self,
        ..default()
    };
}