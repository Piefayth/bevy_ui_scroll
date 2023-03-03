use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

const FIXED_TIMESTEP: f32 = 0.1;

use bevy_ui_scroll::background_scene::*;
use bevy_ui_scroll::scroll::UiScrollPlugin;
use bevy_ui_scroll::scroll::bundles::*;
use bevy_ui_scroll::scroll::components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UiScrollPlugin)
        .add_startup_system(setup_scene)
        //.add_startup_system(setup_ui_single)
         .add_startup_system(setup_ui_multi)
        .add_system(fixed_update.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}

type FixedUpdateTickCount = u16;
#[derive(Component)]
pub struct ScrollMultiContainer {
    index: u16,
}

fn fixed_update(
    mut commands: Commands,
    mut count: Local<FixedUpdateTickCount>,
    scroll_multi_container_query: Query<(Entity, &ScrollMultiContainer)>,
    scroll_content_query: Query<Entity, With<ScrollContent>>,
) {
    if *count > 60 {
        return;
    }

    *count += 1;
    let mut any_multi_containers = false;

    for (multi_container_entity, multi_container) in scroll_multi_container_query.iter() {
        any_multi_containers = true;
        let number = count.saturating_mul(multi_container.index);
        generate_filler_content(&mut commands, multi_container_entity, number);
    }

    if !any_multi_containers {
        for scroll_content_entity in scroll_content_query.iter() {
            generate_filler_content(&mut commands, scroll_content_entity, *count);
        }
    }
}

fn generate_filler_content(
    commands: &mut Commands,
    parent: Entity,
    number: u16,
) {
    let fixed_tick_node = NodeBundle {
        style: Style {
            min_size: Size {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
            },
            size: Size {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
            },
            margin: UiRect {
                top: Val::Px(2.0),
                left: Val::Px(2.0),
                ..default()
            },
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    commands.entity(parent).with_children(|container_panel_p| {
        container_panel_p
            .spawn(fixed_tick_node)
            .insert(BackgroundColor(Color::rgb(
                0.5,
                1.0 - (f32::from(number) / 3000.0),
                f32::from(number) / 3000.0,
            )));
    });
}

fn setup_ui_multi(mut commands: Commands) {
    setup_camera(&mut commands);

    let vertical_scroll_handle = commands
        .spawn(ScrollHandleBundle::new(ScrollHandle {
            position: 0.0,
            length: 40.0,
            girth: 18.0,
            visible: true,
        }))
        .insert(BackgroundColor(Color::rgb(1.0, 1.0, 0.0)))
        .id();

    let vertical_scroll_bar = commands
        .spawn(ScrollBarBundle::new(ScrollBar {
            orientation: ScrollBarOrientation::Vertical,
            scroll_handle: vertical_scroll_handle,
            girth: Val::Px(20.0),
        }))
        .insert(BackgroundColor(Color::rgb(0.15, 0.15, 0.15)))
        .id();

    let horizontal_scroll_handle = commands
        .spawn(ScrollHandleBundle::new(ScrollHandle {
            position: 0.0,
            length: 40.0,
            girth: 18.0,
            visible: true,
        }))
        .insert(BackgroundColor(Color::rgb(1.0, 1.0, 0.0)))
        .id();

    let horizontal_scroll_bar = commands
        .spawn(ScrollBarBundle::new(ScrollBar {
            orientation: ScrollBarOrientation::Horizontal,
            scroll_handle: horizontal_scroll_handle,
            girth: Val::Px(20.0),
        }))
        .insert(BackgroundColor(Color::rgb(0.15, 0.15, 0.15)))
        .id();

    let multi_scroll_container = commands
        .spawn(ScrollContentBundle::new(ScrollContent {
            direction: ScrollDirection::Both,
            ..default()
        }))
        .insert(BackgroundColor(Color::rgb(0.25, 0.25, 0.25)))
        .id();

    let multi_scroll_root = commands
        .spawn(ScrollRootBundle::new(ScrollRoot {
            direction: ScrollDirection::Both,
        }))
        .insert(BackgroundColor(Color::rgb(1.0, 0.5, 0.5)))
        .id();

    commands
        .entity(vertical_scroll_bar)
        .add_child(vertical_scroll_handle);

    commands
        .entity(horizontal_scroll_bar)
        .add_child(horizontal_scroll_handle);

    commands
        .entity(multi_scroll_root)
        .add_child(multi_scroll_container);

    commands
        .entity(multi_scroll_container)
        .with_children(|p_multi_scroll_container| {
            // put whatever inside the scroll container...
            for index in 1..50 {
                p_multi_scroll_container
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(ScrollMultiContainer { index });
            }
        })
        .insert(ControlledByScrollBars {
            scroll_bars: vec![vertical_scroll_bar, horizontal_scroll_bar],
        });

    commands
        .entity(vertical_scroll_handle)
        .insert(ControlsScrollContent {
            scroll_content: multi_scroll_container,
        });

    commands
        .entity(horizontal_scroll_handle)
        .insert(ControlsScrollContent {
            scroll_content: multi_scroll_container,
        });

    commands
        .spawn(panel_ui_root())
        .add_child(horizontal_scroll_bar)
        .add_child(vertical_scroll_bar)
        .add_child(multi_scroll_root);
}

fn setup_ui_single(mut commands: Commands) {
    setup_camera(&mut commands);

    let vertical_scroll_handle = commands
        .spawn(ScrollHandleBundle::new(ScrollHandle {
            position: 0.0,
            length: 40.0,
            girth: 18.0,
            visible: true,
        }))
        .insert(BackgroundColor(Color::rgb(1.0, 1.0, 0.0)))
        .id();

    let vertical_scroll_bar = commands
        .spawn(ScrollBarBundle::new(ScrollBar {
            orientation: ScrollBarOrientation::Vertical,
            scroll_handle: vertical_scroll_handle,
            girth: Val::Px(20.0),
        }))
        .insert(BackgroundColor(Color::rgb(0.15, 0.15, 0.15)))
        .id();

    let vertical_scroll_container = commands
        .spawn(ScrollContentBundle::new(ScrollContent {
            direction: ScrollDirection::Vertical,
            ..default()
        }))
        .insert(BackgroundColor(Color::rgb(0.5, 0.5, 0.5)))
        .id();

    let vertical_scroll_root = commands
        .spawn(ScrollRootBundle::new(ScrollRoot {
            direction: ScrollDirection::Vertical,
        }))
        .insert(BackgroundColor(Color::rgb(0.5, 0.5, 1.0)))
        .id();

    let horizontal_scroll_handle = commands
        .spawn(ScrollHandleBundle::new(ScrollHandle {
            position: 0.0,
            length: 40.0,
            girth: 18.0,
            visible: true,
        }))
        .insert(BackgroundColor(Color::rgb(1.0, 1.0, 0.0)))
        .id();

    let horizontal_scroll_bar = commands
        .spawn(ScrollBarBundle::new(ScrollBar {
            orientation: ScrollBarOrientation::Horizontal,
            scroll_handle: horizontal_scroll_handle,
            girth: Val::Px(20.0),
        }))
        .insert(BackgroundColor(Color::rgb(0.15, 0.15, 0.15)))
        .id();

    let horizontal_scroll_container = commands
        .spawn(ScrollContentBundle::new(ScrollContent {
            direction: ScrollDirection::Horizontal,
            ..default()
        }))
        .insert(BackgroundColor(Color::rgb(0.25, 0.25, 0.25)))
        .id();

    let horizontal_scroll_root = commands
        .spawn(ScrollRootBundle::new(ScrollRoot {
            direction: ScrollDirection::Horizontal,
        }))
        .insert(BackgroundColor(Color::rgb(1.0, 0.5, 0.5)))
        .id();
    
    commands
        .entity(vertical_scroll_bar)
        .add_child(vertical_scroll_handle);

    commands
        .entity(vertical_scroll_root)
        .add_child(vertical_scroll_container);

    commands
        .entity(horizontal_scroll_bar)
        .add_child(horizontal_scroll_handle);

    commands
        .entity(horizontal_scroll_root)
        .add_child(horizontal_scroll_container);

    commands
        .entity(vertical_scroll_container)
        .insert(ControlledByScrollBars {
            scroll_bars: vec![vertical_scroll_bar],
        });

    commands
        .entity(horizontal_scroll_container)
        .insert(ControlledByScrollBars {
            scroll_bars: vec![horizontal_scroll_bar],
        });

    commands
        .entity(vertical_scroll_handle)
        .insert(ControlsScrollContent {
            scroll_content: vertical_scroll_container,
        });

    commands
        .entity(horizontal_scroll_handle)
        .insert(ControlsScrollContent {
            scroll_content: horizontal_scroll_container,
        });

    commands
        .spawn(panel_ui_root())
        .add_child(horizontal_scroll_root)
        .add_child(horizontal_scroll_bar)
        .add_child(vertical_scroll_root)
        .add_child(vertical_scroll_bar);
}

fn panel_ui_root() -> NodeBundle {
    return NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
            },
            min_size: Size {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
            },
            ..default()
        },
        ..default()
    };
}

fn setup_camera(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            camera: Camera {
                order: 1,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(UiCamera);
}
