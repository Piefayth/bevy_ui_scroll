use std::marker::PhantomData;

use bevy::prelude::*;

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

#[derive(Component, Clone, Debug)]
pub struct ControlledByElement<C: Component> {
    pub element: Entity,
    pub kind: PhantomData<C>
}

#[derive(Clone, Copy, Default, Debug)]
pub struct PressInput {
    pub position: Vec2,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct ScrollInput {
    pub distance: Vec2,
}

pub struct ScrollIndicatorEnabled {
    pub position: Vec2,
}

pub struct ScrollIndicatorDisabled;

pub struct UiEvent<C: Component, A> {
    pub widget: Entity,
    pub action: A,
    pub element_kind: PhantomData<C>,
}