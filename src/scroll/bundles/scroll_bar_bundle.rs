use bevy::{prelude::*, ui::{FocusPolicy, RelativeCursorPosition}};
use crate::scroll::styles::scroll_bar_style;

use super::super::components::*;

#[derive(Bundle, Clone, Debug)]
pub struct ScrollBarBundle {
    pub node: Node,
    pub style: Style,
    pub background_color: BackgroundColor,
    pub focus_policy: FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub z_index: ZIndex,

    pub interaction: Interaction,
    pub relative_cursor_position: RelativeCursorPosition,
    pub scroll_bar: ScrollBar,
}

impl ScrollBarBundle {
    pub fn new(scroll_bar: ScrollBar) -> Self {
        Self {
            background_color: Color::NONE.into(),
            node: Default::default(),
            style: scroll_bar_style(scroll_bar.orientation, scroll_bar.girth),
            focus_policy: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            z_index: Default::default(),
            interaction: Default::default(),
            relative_cursor_position: Default::default(),
            scroll_bar,
        }
    }
}