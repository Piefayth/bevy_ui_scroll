use bevy::prelude::*;

#[derive(Component)]
pub struct UiCamera;

#[derive(Component)]
pub struct ScrollIndicator {
    pub scroll_target: Entity,
}

#[derive(Component, Clone, Debug)]
pub struct ScrollRoot {
    pub direction: ScrollDirection,
}

#[derive(Component, Default, Clone, Debug)]
pub struct ScrollContent {
    pub direction: ScrollDirection,
    pub current_offset: Vec2,
}

#[derive(Component)]
pub struct ControlledByScrollBars {
    pub scroll_bars: Vec<Entity>,
}

#[derive(Component)]
pub struct ControlsScrollContent {
    pub scroll_content: Entity,
}

#[derive(Component, Clone, Debug)]
pub struct ScrollBar {
    pub scroll_handle: Entity,
    pub orientation: ScrollBarOrientation,
    pub girth: Val,
}

#[derive(Component, Default, Clone, Debug)]
pub struct ScrollHandle {
    pub position: f32,
    pub length: f32,
    pub girth: f32,
    pub visible: bool,
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum ScrollDirection {
    #[default]
    Vertical,
    Horizontal,
    Both,
    Neither,
}

#[derive(Default, Clone, Copy, Debug)]
pub enum ScrollBarOrientation {
    #[default]
    Vertical,
    Horizontal,
}