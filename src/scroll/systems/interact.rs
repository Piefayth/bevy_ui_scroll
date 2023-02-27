use bevy::{
    ecs::change_detection::*,
    input::mouse::{MouseScrollUnit, MouseWheel},
    math::CubicBezierEasing,
    prelude::*,
    ui::RelativeCursorPosition,
};

use crate::scroll::components::*;

const PIXELS_SCROLLED_PER_LINE: f32 = 15.0;
pub fn handle_mousewheel(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    scroll_root_query: Query<(&Interaction, &Children), With<ScrollRoot>>,
    mut scroll_content_query: Query<&mut ScrollContent>,
) {
    let hovered_roots = scroll_root_query
        .iter()
        .filter(|(interaction, _)| **interaction == Interaction::Hovered);

    for (_, hovered_children) in hovered_roots {
        let scroll_content_entity = hovered_children
            .iter()
            .filter(|child| scroll_content_query.contains(**child))
            .last()
            .expect("ScrollRoot should have one direct descendant with ScrollContent.");

        let mut scroll_content = scroll_content_query
            .get_mut(*scroll_content_entity)
            .unwrap();

        for mousewheel_event in mouse_wheel_events.iter() {
            let scroll_distance: Vec2 = match mousewheel_event.unit {
                MouseScrollUnit::Line => {
                    let scroll_distance_x = mousewheel_event.x * PIXELS_SCROLLED_PER_LINE;
                    let scroll_distance_y = mousewheel_event.y * PIXELS_SCROLLED_PER_LINE;
                    Vec2 {
                        x: scroll_distance_x,
                        y: scroll_distance_y,
                    }
                }
                MouseScrollUnit::Pixel => Vec2 {
                    x: mousewheel_event.x,
                    y: mousewheel_event.y,
                },
            };

            scroll_content.current_offset -= scroll_distance;
        }
    }
}

pub fn scroll_bar_interaction(
    scroll_bar_query: Query<(&ScrollBar, &Interaction, &RelativeCursorPosition, &Node)>,
    mut scroll_handle_query: Query<(&mut ScrollHandle, &Node)>,
) {
    for (scroll_bar, scroll_bar_interaction, scroll_bar_rel_cursor, scroll_bar_node) in
        scroll_bar_query.iter()
    {
        if *scroll_bar_interaction != Interaction::Clicked {
            continue;
        }

        let cursor_pos = if let Some(cursor_pos) = scroll_bar_rel_cursor.normalized {
            cursor_pos
        } else {
            continue;
        };

        let (mut scroll_handle, scroll_handle_node) = scroll_handle_query
            .get_mut(scroll_bar.scroll_handle)
            .expect("The Entity in scroll_bar.scroll_handle should have a ScrollHandle component.");

        let scroll_handle_halfsize = scroll_handle_node.size() / 2.0;
        let scroll_bar_cursor_position_px = cursor_pos * scroll_bar_node.size();
        match scroll_bar.orientation {
            ScrollBarOrientation::Vertical => {
                scroll_handle.position = scroll_bar_cursor_position_px.y - scroll_handle_halfsize.y;
            }
            ScrollBarOrientation::Horizontal => {
                scroll_handle.position = scroll_bar_cursor_position_px.x - scroll_handle_halfsize.x;
            }
        }
    }
}

pub fn handle_middle_click(
    mut commands: Commands,
    mut cursor_marker_entity: Local<Option<Entity>>,
    asset_server: Res<AssetServer>,
    mouse_button_input: ResMut<Input<MouseButton>>,
    window_query: Query<&Window>,
    scroll_root_query: Query<(&Interaction, &Children), With<ScrollRoot>>,
    scroll_content_query: Query<&ScrollContent>,
) {
    if mouse_button_input.just_released(MouseButton::Middle) {
        match *cursor_marker_entity {
            Some(entity) => {
                commands.entity(entity).despawn();
                *cursor_marker_entity = None;
            }
            None => (),
        }
    }

    let hovered_roots = scroll_root_query
        .iter()
        .filter(|(interaction, _)| **interaction == Interaction::Hovered);

    for (_, hovered_children) in hovered_roots {
        let scroll_content_entity = hovered_children
            .iter()
            .filter(|child| scroll_content_query.contains(**child))
            .last()
            .expect("ScrollRoot should have one direct descendant with ScrollContent.");

        let scroll_content = scroll_content_query
            .get(*scroll_content_entity)
            .unwrap();

            if mouse_button_input.just_pressed(MouseButton::Middle) {
                let window = window_query.single();
        
                let coords = if let Some(world_position) = window.cursor_position() {
                    world_position
                } else {
                    return;
                };
        
                let texture = match scroll_content.direction {
                    ScrollDirection::Vertical => asset_server.load("sprites/scroll_cursor_vert.png"),
                    ScrollDirection::Horizontal => asset_server.load("sprites/scroll_cursor_horiz.png"),
                    ScrollDirection::Both => asset_server.load("sprites/scroll_cursor_multi.png"),
                    ScrollDirection::Neither => todo!(),
                };
        
                *cursor_marker_entity = Some(
                    commands
                        .spawn(ImageBundle {
                            image: UiImage {
                                texture: texture,
                                ..default()
                            },
                            style: Style {
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    left: Val::Px(coords.x - 16.0),
                                    right: Val::Px(0.0),
                                    top: Val::Px(window.height() - coords.y - 16.0),
                                    bottom: Val::Px(0.0),
                                },
                                size: Size {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(ScrollIndicator {
                            scroll_target: *scroll_content_entity,
                        })
                        .id(),
                );
            }
    }
}

pub fn scroll_from_scroll_indicator(
    window_query: Query<&Window>,
    scroll_indicator_query: Query<(&ScrollIndicator, &GlobalTransform)>,
    mut scroll_query: Query<&mut ScrollContent>,
) {
    let window = window_query.single();
    
    let cursor_coords = if let Some(world_position) = window.cursor_position() {
        world_position
    } else {
        return;
    };

    for (scroll_indicator, scroll_indicator_transform) in
        scroll_indicator_query.iter()
    {
        let mut scroll_content = scroll_query
            .get_mut(scroll_indicator.scroll_target)
            .expect("ScrollIndicator should not have held a reference to an invalid Scroll.");

        let indicator_coords = Vec2 {
            x: scroll_indicator_transform.translation().x,
            y: window.height() - scroll_indicator_transform.translation().y,
        };

        let easing = CubicBezierEasing::new([0.35, 0.01], [0.97, 0.79]);
        let delta = cursor_coords - indicator_coords;
        let window_dimensions = Vec2 { x: window.width(), y: window.height() };
        let normalized_delta = delta / window_dimensions;

        let eased_delta_y = delta.y * easing.ease(normalized_delta.y.abs().max(0.2));
        let eased_delta_x = delta.x * easing.ease(normalized_delta.x.abs().max(0.2));

        match scroll_content.direction {
            ScrollDirection::Vertical => {
                scroll_content.current_offset.y -= eased_delta_y;
            }
            ScrollDirection::Horizontal => {
                scroll_content.current_offset.x += eased_delta_x;
            }
            ScrollDirection::Both => {
                scroll_content.current_offset.y -= eased_delta_y;
                scroll_content.current_offset.x += eased_delta_x;
            }
            ScrollDirection::Neither => {}
        }
    }
}