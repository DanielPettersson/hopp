use bevy::app::{App, Update};
use bevy::math::{Quat, Vec3};
use bevy::prelude::{default, Commands, Component, Entity, EventReader, Plugin, Query, Res, Transform, With, Without};
use bevy::sprite::MaterialMesh2dBundle;
use crate::{MaterialHandles, MeshHandles};
use crate::mouse_drag::MouseDrag;
use crate::player::Player;

#[derive(Component)]
struct DragIndicator;

pub struct DragIndicatorPlugin;

impl Plugin for DragIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, drag_indicator);
    }
}

fn drag_indicator(
    mut commands: Commands,
    mut mouse_drag_event: EventReader<MouseDrag>,
    mut query_drag_indicator: Query<(&mut Transform, Entity), With<DragIndicator>>,
    query_jumper: Query<&Transform, (With<Player>, Without<DragIndicator>)>,
    material_handles: Res<MaterialHandles>,
    mesh_handles: Res<MeshHandles>,
) {
    let mut drag_done = false;
    for drag in mouse_drag_event.read() {
        drag_done = drag.done || drag_done;

        for jumper in query_jumper.iter() {
            let drag_length = (drag.end - drag.start).length().max(10.);
            let drag_mid = (drag.end - drag.start) / 2.;
            let drag_indicator_transform =
                Transform::from_translation(jumper.translation - drag_mid.extend(1.))
                    .with_scale(Vec3::new(drag_length, 2., 1.))
                    .with_rotation(Quat::from_rotation_z(if drag_length > 0. {
                        drag_mid.to_angle()
                    } else {
                        0.
                    }));

            if query_drag_indicator.is_empty() {
                commands.spawn((
                    DragIndicator,
                    MaterialMesh2dBundle {
                        mesh: mesh_handles.rectangle.clone(),
                        material: material_handles.red.clone(),
                        transform: drag_indicator_transform,
                        ..default()
                    },
                ));
            }
            for (mut tranform, _) in query_drag_indicator.iter_mut() {
                *tranform = drag_indicator_transform;
            }
        }
    }
    if drag_done {
        for (_, entity) in query_drag_indicator.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}