use bevy::{prelude::*};

use bevy_ui_scroll::background_scene::*;
use bevy_ui_scroll::scroll::components::*;
use bevy_ui_scroll::scroll::UiScrollPlugin;
use bevy_ui_scroll::scroll::scroll_container::ScrollContainerWidget;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UiScrollPlugin)
        .add_startup_system(setup_scene)
        .add_startup_system(spawn_scroll_container)
        .run();
}

fn spawn_scroll_container(mut commands: Commands) {
    commands.spawn(ScrollContainerWidget {
        scroll_direction: ScrollDirection::Both, 
    });
}