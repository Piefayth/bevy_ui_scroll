pub mod components;
pub mod styles;
pub mod systems;
pub mod bundles;

use systems::*;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy, SystemSet)]
pub enum ScrollSystemSet {
    Interact,
    Constrain,
    Bind,
    Extract,
}

use bevy::prelude::*;

pub struct UiScrollPlugin;

impl Plugin for UiScrollPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(handle_mousewheel.in_set(ScrollSystemSet::Interact))
            .add_system(scroll_bar_interaction.in_set(ScrollSystemSet::Interact))
            .add_system(handle_middle_click.in_set(ScrollSystemSet::Interact))
            .add_system(scroll_from_scroll_indicator.in_set(ScrollSystemSet::Interact))
            .add_system(
                scroll_content_that_controls_scroll_handles
                    .in_set(ScrollSystemSet::Bind)
                    .after(ScrollSystemSet::Interact),
            )
            .add_system(
                scroll_handles_that_control_scroll_content
                    .in_set(ScrollSystemSet::Bind)
                    .after(ScrollSystemSet::Interact),
            )
            .add_system(
                constrain_scroll_handles
                    .in_set(ScrollSystemSet::Constrain)
                    .after(ScrollSystemSet::Bind),
            )
            .add_system(
                constrain_scroll_content
                    .in_set(ScrollSystemSet::Constrain)
                    .after(ScrollSystemSet::Bind),
            )
            .add_system(
                extract_scroll_handle_styles
                    .in_set(ScrollSystemSet::Extract)
                    .after(ScrollSystemSet::Constrain),
            )
            .add_system(
                extract_scroll_content_styles
                    .in_set(ScrollSystemSet::Extract)
                    .after(ScrollSystemSet::Constrain),
            );
    }
}