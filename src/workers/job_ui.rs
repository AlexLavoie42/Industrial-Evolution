use crate::*;

#[derive(Component, Debug)]
pub struct JobPathMarker {
    pub job_point: JobPoint,
}
impl Clickable for JobPathMarker {}

#[derive(Bundle)]
pub struct JobPathMarkerBundle {
    pub marker: JobPathMarker,
    pub sprite: SpriteBundle,
}
impl JobPathMarkerBundle {
    fn new(marker: JobPathMarker) -> Self {
        Self {
            marker,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
                ..default()
            },
        }
    }
}

pub fn spawn_job_path_markers(
    mut commands: Commands,
    selected_worker: Res<SelectedWorker>,
    q_jobs: Query<&Job>,
    q_job_markers: Query<(Entity, &JobPathMarker)>,
    q_tilemap: Query<(&Transform, &TilemapGridSize, &TilemapType)>
) {
    let Some(worker) = selected_worker.selected else { return };
    let Ok(job) = q_jobs.get(worker) else { return };

    q_job_markers
        .iter()
        .filter(|(_, marker)| {
            return !job.path.contains(&marker.job_point);
        })
        .for_each(|(entity, _)| commands.entity(entity).despawn());

    for job_point in job.path.iter() {
        let existing = q_job_markers
            .iter()
            .any(|(_, marker)| marker.job_point == *job_point);

        if existing { continue; }

        let (map_transform, grid_size, map_type) = q_tilemap.single();

        let job_pos = get_tile_world_pos(&job_point.point, map_transform, grid_size, map_type);
        let mut marker = JobPathMarkerBundle::new(JobPathMarker {
            job_point: job_point.clone()
        });
        marker.sprite.transform.translation = vec3(job_pos.x, job_pos.y, marker.sprite.transform.translation.z);
        commands.spawn(marker);
    }
}

pub fn job_path_lines(
    mut gizmos: Gizmos,
    q_jobs: Query<&Job>,
    selected_worker: Res<SelectedWorker>,
    q_tilemap: Query<(&Transform, &TilemapGridSize, &TilemapType)>,
) {
    let Some(worker) = selected_worker.selected else { return };
    let Ok(job) = q_jobs.get(worker) else { return };
    let (map_transform, grid_size, map_type) = q_tilemap.single();
    if job.path.len() < 2 { return; }
    for i in 0..job.path.len() - 1 {
        let start = get_tile_world_pos(&job.path[i].point, map_transform, grid_size, map_type);
        let end = get_tile_world_pos(&job.path[i + 1].point, map_transform, grid_size, map_type);
        gizmos.line_2d(Vec2 { x: start.x, y: start.y }, Vec2 { x: end.x, y: end.y }, Color::GREEN);

    }
}

pub fn despawn_job_path_markers(
    mut commands: Commands,
    q_job_markers: Query<(Entity, &JobPathMarker)>
) {
    q_job_markers
        .iter()
        .for_each(|(entity, _)| commands.entity(entity).despawn_recursive());
}

pub fn remove_job_point_click(
    mut commands: Commands,
    mut hover_job_point: EventReader<GenericMouseCollisionEvent<JobPathMarker>>,
    q_job_markers: Query<&JobPathMarker>,
    mut q_jobs: Query<&mut Job>,
    selected_worker: Res<SelectedWorker>,
    input: Res<Input<MouseButton>>,
) {
    for event in hover_job_point.iter() {
        // TODO: Hover interaction
        if input.just_pressed(MouseButton::Right) {
            if let Some((_, entity)) = event.collision {
                let Ok(marker) = q_job_markers.get(entity) else { continue };
                let Some(selected) = selected_worker.selected else { continue };
                let Ok(mut job) = q_jobs.get_mut(selected) else { continue };
    
                job.path.retain(|x| x != &marker.job_point);
            }
        }
    }
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct JobUIContainerProps;
impl Widget for JobUIContainerProps {}

#[derive(Bundle)]
pub struct JobUIContainerBundle {
    pub props: JobUIContainerProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for JobUIContainerBundle {
    fn default() -> Self {
        Self {
            props: JobUIContainerProps,
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            on_event: OnEvent::default(),
            widget_name: JobUIContainerProps::default().get_name(),
        }
    }
}
