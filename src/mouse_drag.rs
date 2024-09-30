use bevy::app::App;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{
    Camera, Event, EventWriter, GlobalTransform, Local, MouseButton, Plugin, Query, Res, Update,
    Window, With,
};
use bevy::window::PrimaryWindow;

fn mouse_drag(
    mouse_button: Res<ButtonInput<MouseButton>>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
    mut event_writer: EventWriter<MouseDrag>,
    mut drag_start: Local<Vec2>,
    mut drag_last: Local<Vec2>,
) {
    if mouse_button.pressed(MouseButton::Left) || mouse_button.just_released(MouseButton::Left) {
        let (camera, camera_transform) = query_camera.single();

        let cursor_pos = query_window
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate());

        if mouse_button.just_pressed(MouseButton::Left) {
            if let Some(pos) = cursor_pos {
                *drag_start = pos;
                *drag_last = pos;
                event_writer.send(MouseDrag {
                    start: pos,
                    end: pos,
                    done: false,
                });
            }
        } else if mouse_button.pressed(MouseButton::Left) {
            if let Some(pos) = cursor_pos {
                *drag_last = pos;
                event_writer.send(MouseDrag {
                    start: *drag_start,
                    end: pos,
                    done: false,
                });
            }
        } else if mouse_button.just_released(MouseButton::Left) {
            event_writer.send(MouseDrag {
                start: *drag_start,
                end: cursor_pos.unwrap_or(*drag_last),
                done: true,
            });
        }
    }
}

#[derive(Event, Default, Debug)]
pub struct MouseDrag {
    pub start: Vec2,
    pub end: Vec2,
    pub done: bool,
}

pub struct MouseDragPlugin;

impl Plugin for MouseDragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_drag).add_event::<MouseDrag>();
    }
}
