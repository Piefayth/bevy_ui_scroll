use bevy::{prelude::*, ui::FocusPolicy};

use crate::scroll::styles::scroll_container_style;

use super::super::components::*;

#[derive(Bundle, Clone, Debug)]
pub struct ScrollContentBundle {
    pub node: Node,
    pub style: Style,
    pub background_color: BackgroundColor,
    pub focus_policy: FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub z_index: ZIndex,

    pub scroll_content: ScrollContent,
}

impl ScrollContentBundle {
    pub fn new(scroll_content: ScrollContent) -> Self {
        Self {
            node: Default::default(),
            style: scroll_container_style(scroll_content.direction),
            background_color: Default::default(),
            focus_policy: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            z_index: Default::default(),
            scroll_content: scroll_content,
        }
    }
}
