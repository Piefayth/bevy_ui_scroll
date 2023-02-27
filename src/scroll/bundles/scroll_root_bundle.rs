use bevy::{prelude::*, ui::{FocusPolicy}};
use crate::scroll::styles::scroll_root_style;

use super::super::components::*;

#[derive(Bundle, Clone, Debug)]
pub struct ScrollRootBundle {
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
    pub scroll_root: ScrollRoot,
}

impl ScrollRootBundle {
    pub fn new(scroll_root: ScrollRoot) -> Self {
        Self {
            background_color: Color::NONE.into(),
            node: Default::default(),
            style: scroll_root_style(scroll_root.direction),
            focus_policy: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            z_index: Default::default(),
            interaction: Default::default(),
            scroll_root,
        }
    }
}