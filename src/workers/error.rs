use crate::*;

#[derive(Component, Debug, Reflect)]
pub struct JobError {
    pub job_index: usize,
    pub error: bool,
    pub warning: bool,
    pub warn_message: String,
    pub message: String,
}

impl JobError {
    pub fn new() -> JobError {
        JobError {
            job_index: 0,
            error: false,
            warning: false,
            warn_message: String::new(),
            message: String::new()
        }
    }

    pub fn set_error(&mut self, message: &str) {
        self.error = true;
        self.message = message.into();
    }

    pub fn set_warning(&mut self, message: &str) {
        self.warning = true;
        self.warn_message = message.into();
    }

    pub fn clear_error(&mut self) {
        self.error = false;
        self.warning = false;
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
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(6.0, 6.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 5.0),
                ..default()
            }
        }
    }
}

pub fn job_error_marker(
    mut commands: Commands,
    job_error: Query<(Entity, &mut JobError, &Children)>,
    q_error_marker: Query<&JobErrorMarker>,
) {
    for (worker_entity, job_error, children) in job_error.iter() {
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

#[derive(Component)]
pub struct JobWarningMarker;

#[derive(Bundle)]
pub struct JobWarningMarkerBundle {
    marker: JobWarningMarker,
    sprite: SpriteBundle
}
impl Default for JobWarningMarkerBundle {
    fn default() -> Self {
        JobWarningMarkerBundle {
            marker: JobWarningMarker,
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

pub fn job_warning_marker(
    mut commands: Commands,
    job_warning: Query<(Entity, &mut JobError, &Children)>,
    q_warning_marker: Query<&JobWarningMarker>,
) {
    for (worker_entity, job_warning, children) in job_warning.iter() {
        if job_warning.warning && !job_warning.error {
            let exists: bool = children.iter().filter(|child| { q_warning_marker.get(**child).is_ok() }).peekable().peek().is_some();
            if exists { continue }
            let mut marker_bundle = JobErrorMarkerBundle::default();
            marker_bundle.sprite.transform.translation.x = 18.0;
            marker_bundle.sprite.transform.translation.y = 26.0;
            let marker = commands.spawn(marker_bundle).id();
            commands.entity(worker_entity).push_children(&[marker]);
        } else {
            let children: Vec<Entity> = children.iter().filter(|child| { q_warning_marker.get(**child).is_ok() }).map(|child| { *child }).collect();
            if children.is_empty() { continue; }

            commands.entity(worker_entity).remove_children(children.as_slice());
            for child in children {
                commands.entity(child).despawn_recursive();
            }
        }
    }
}
