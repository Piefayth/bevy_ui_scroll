use bevy::prelude::*;
use crate::scroll::components::*;

pub fn scroll_wrapper_style(scroll_direction: ScrollDirection) -> Style {
    return Style {
        min_size:          Size {
            width: Val::Percent(30.0),
            height: Val::Percent(30.0),
        },
        size: Size {
            width: Val::Percent(30.0),
            height: Val::Percent(30.0),
        },
        max_size: Size {
            width: Val::Percent(30.0),
            height: Val::Percent(30.0),
        },
        flex_direction: match scroll_direction {
            ScrollDirection::Vertical => FlexDirection::Row,
            ScrollDirection::Horizontal => FlexDirection::Column,
            ScrollDirection::Both => FlexDirection::Row,
            ScrollDirection::Neither => todo!(),
        },
        overflow: Overflow::Hidden,
        align_items: AlignItems::Baseline,
        align_self: AlignSelf::Center,
        margin: UiRect {
            left: Val::Percent(10.0),
            ..default()
        },
        ..default()
    };
}