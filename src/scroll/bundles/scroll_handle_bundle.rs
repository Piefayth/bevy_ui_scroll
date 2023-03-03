use bevy::{prelude::*, ui::{FocusPolicy}};
use crate::scroll::styles::scroll_handle_style;

use super::super::components::*;

#[derive(Bundle, Clone, Debug)]
pub struct ScrollHandleBundle {
    pub node: Node,
    pub style: Style,
    pub background_color: BackgroundColor,
    pub focus_policy: FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub z_index: ZIndex,

    pub scroll_handle: ScrollHandle,
}

impl ScrollHandleBundle {
    pub fn new(scroll_handle: ScrollHandle) -> Self {
        Self {
            background_color: Color::NONE.into(),
            node: Default::default(),
            style: scroll_handle_style(),
            focus_policy: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            z_index: Default::default(),
            scroll_handle,
        }
    }
}