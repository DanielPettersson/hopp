use bevy::app::App;
use bevy::input::mouse::MouseMotion;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{
    Event, EventReader, EventWriter, KeyCode, Local, MouseButton, Plugin, Query, Res, Update,
    Window, With,
};
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
    mut query_window: Query<&mut Window, With<PrimaryWindow>>,
    mut event_writer: EventWriter<Drag>,
    mut drag_last: Local<Vec2>,
    mut evr_motion: EventReader<MouseMotion>,
) {
    if mouse_button.pressed(MouseButton::Left)
        || mouse_button.just_released(MouseButton::Left)
        || !evr_motion.is_empty()
    {
        let mut window = query_window.single_mut();
        let mouse_move: Vec2 = evr_motion.read().map(|e| e.delta).sum();

        if mouse_button.just_pressed(MouseButton::Left) {
            *drag_last = Vec2::ZERO;
            event_writer.send(Drag {
                start: Vec2::ZERO,
                end: Vec2::ZERO,
                done: false,
            });
            window.cursor.visible = false;
        } else if mouse_button.pressed(MouseButton::Left) {
            *drag_last += Vec2::new(mouse_move.x, -mouse_move.y);
            event_writer.send(Drag {
                start: Vec2::ZERO,
                end: *drag_last,
                done: false,
            });
        } else if mouse_button.just_released(MouseButton::Left) {
            event_writer.send(Drag {
                start: Vec2::ZERO,
                end: *drag_last,
                done: true,
            });
            window.cursor.visible = true;
        }
    }
}
