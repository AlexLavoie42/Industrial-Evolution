use std::borrow::BorrowMut;

use crate::*;

#[derive(Component, Debug, Reflect)]
pub struct JobError {
    pub job_index: usize,
    pub error: bool,
    pub message: String
}

impl JobError {
    pub fn new() -> JobError {
        JobError {
            job_index: 0,
            error: false,
            message: String::new()
        }
    }

    pub fn set_error(&mut self, message: &str) {
        self.error = true;
        self.message = message.into();
    }

    pub fn clear_error(&mut self) {
        self.error = false;
        self.message = String::new();
    }
}

#[derive(Component)]
pub struct JobErrorMarker;

#[derive(Bundle)]
pub struct JobErrorMarkerBundle {
    marker: JobErrorMarker,
    sprite: SpriteBundle
}
impl Default for JobErrorMarkerBundle {
    fn default() -> Self {
        JobErrorMarkerBundle {
            marker: JobErrorMarker,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(6.0, 6.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 6.0),
                ..default()
            }
        }
    }
}

pub fn job_error_marker(
    mut commands: Commands,
    mut job_error: Query<(Entity, &mut JobError, &Children)>,
    mut q_error_marker: Query<&JobErrorMarker>,
) {
    for (worker_entity, mut job_error, children) in job_error.iter_mut() {
        if job_error.error {
            let exists: bool = children.iter().filter(|child| { q_error_marker.get(**child).is_ok() }).peekable().peek().is_some();
            if exists { continue }
            let mut marker_bundle = JobErrorMarkerBundle::default();
            marker_bundle.sprite.transform.translation.x = 18.0;
            marker_bundle.sprite.transform.translation.y = 26.0;
            let marker = commands.spawn(marker_bundle).id();
            commands.entity(worker_entity).push_children(&[marker]);
        } else {
            let children: Vec<Entity> = children.iter().filter(|child| { q_error_marker.get(**child).is_ok() }).map(|child| { *child }).collect();
            if children.is_empty() { continue; }

            commands.entity(worker_entity).remove_children(children.as_slice());
            for child in children {
                commands.entity(child).despawn_recursive();
            }
        }
    }
}
