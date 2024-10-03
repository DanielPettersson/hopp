use bevy::app::App;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Camera, Event, EventWriter, GlobalTransform, KeyCode, Local, MouseButton, Plugin, Query, Res, Update, Vec3Swizzles, Window, With};
use bevy::window::PrimaryWindow;

#[derive(Event, Default, Debug)]
pub struct Drag {
    pub start: Vec2,
    pub end: Vec2,
    pub done: bool,
}

pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mouse_drag, keyboard_drag))
            .add_event::<Drag>();
    }
}

fn keyboard_drag(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<Drag>,
    mut drag_len: Local<f32>,
    mut drag_rot: Local<f32>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        *drag_len = 0.;
        *drag_rot = 0.;
        event_writer.send(Drag {
            start: Vec2::ZERO,
            end: Vec2::ZERO,
            done: false,
        });
    } else if keyboard.pressed(KeyCode::Space) {
        *drag_len = 1. + *drag_len * 1.1;
        event_writer.send(Drag {
            start: Vec2::ZERO,
            end: Vec2::new(0., -*drag_len),
            done: false,
        });
    } else if keyboard.just_released(KeyCode::Space) {
        event_writer.send(Drag {
            start: Vec2::ZERO,
            end: Vec2::new(0., -*drag_len),
            done: true,
        });
    }
    
}

fn mouse_drag(
    mouse_button: Res<ButtonInput<MouseButton>>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    mut event_writer: EventWriter<Drag>,
    mut drag_start: Local<Vec2>,
    mut drag_last: Local<Vec2>,
) {
    if mouse_button.pressed(MouseButton::Left) || mouse_button.just_released(MouseButton::Left) {
        let (camera, camera_transform) = query_camera.single();

        let cursor_pos = query_window
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate()).map(|p| p - camera_transform.translation().xy());

        if mouse_button.just_pressed(MouseButton::Left) {
            if let Some(pos) = cursor_pos {
                *drag_start = pos;
                *drag_last = pos;
                event_writer.send(Drag {
                    start: pos,
                    end: pos,
                    done: false,
                });
            }
        } else if mouse_button.pressed(MouseButton::Left) {
            if let Some(pos) = cursor_pos {
                *drag_last = pos;
                event_writer.send(Drag {
                    start: *drag_start,
                    end: pos,
                    done: false,
                });
            }
        } else if mouse_button.just_released(MouseButton::Left) {
            event_writer.send(Drag {
                start: *drag_start,
                end: cursor_pos.unwrap_or(*drag_last),
                done: true,
            });
        }
    }
}
