use bevy::reflect::Enum;

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

#[derive(Component, Clone, PartialEq, Default)]
pub struct WorkerMenuHUDProps;
impl Widget for WorkerMenuHUDProps {}

#[derive(Bundle)]
pub struct WorkerMenuHUDBundle {
    pub props: WorkerMenuHUDProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
}
impl Default for WorkerMenuHUDBundle {
    fn default() -> Self {
        Self {
            props: Default::default(),
            styles: KStyle {
                ..default()
            },
            computed_styles: Default::default(),
            widget_name: WorkerMenuHUDProps::default().get_name(),
        }
    }
}

pub fn worker_menu_hud_render(
    In(entity): In<Entity>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    widget_context: Res<KayakWidgetContext>,
    mut query: Query<(&mut WorkerMenuHUDProps, &mut ComputedStyles, &KStyle)>,
    player_state: Res<State<PlayerState>>,
    selected_worker: Res<SelectedWorker>,
    mut q_jobs: Query<(&mut WorkerState, &Job)>,
    time: Res<Time>,
) -> bool {
    if let Ok((mut props, mut computed_styles, style)) = query.get_mut(entity) {
        *computed_styles = KStyle {
            ..Default::default()
        }
        .with_style(style)
        .into();
        if player_state.get() == &PlayerState::Jobs {
            let parent_id = Some(entity);

            let background = assets.load("Worker Menu.png");

            let Some(selected_worker) = selected_worker.selected else { return true };

            let Ok((worker_state, job)) = q_jobs.get_mut(selected_worker) else { return true };

            let current_job = job.current_job.unwrap_or_default();
            let current_job_name = job.path.get(current_job)
                .map_or("None", |job| job.action.variant_name());
            rsx!(
                <NinePatchBundle
                    nine_patch={NinePatch {
                        handle: background,
                        ..default()
                    }}
                    on_event={
                        OnEvent::new(
                            move |In(_entity): In<Entity>, event: ResMut<KEvent>, mut placement_state: ResMut<NextState<PlacementState>> | {
                                if let EventType::Hover(_) = event.event_type {
                                    placement_state.set(PlacementState::Blocked);
                                }
                                if let EventType::MouseOut(_) = event.event_type {
                                    placement_state.set(PlacementState::Allowed);
                                }
                            }
                        )
                    }
                >
                    <ElementBundle 
                        styles={KStyle {
                            height: Units::Pixels(15.0).into(),       
                            ..default()
                        }}
                    />
                    <ElementBundle 
                        styles={KStyle {
                            layout_type: LayoutType::Row.into(),
                            height: Units::Pixels(32.0).into(),
                            ..default()
                        }}    
                    >
                        <TextWidgetBundle
                            text={TextProps {
                                content: format!("Current Job: {:}: {:}", current_job, current_job_name),
                                ..default()      
                            }}
                        />
                        {
                            if *worker_state == WorkerState::Paused {
                                constructor!(
                                    <NinePatchBundle
                                        styles={KStyle {
                                            width: Units::Pixels(32.0).into(),
                                            height: Units::Pixels(32.0).into(),
                                            ..default()
                                        }}
                                        nine_patch={NinePatch {
                                            handle: assets.load("Start Icon.png"),
                                            ..default()
                                        }}
                                        on_event={OnEvent::new(
                                            |
                                                entity: In<Entity>,
                                                selected_worker: Res<SelectedWorker>,
                                                mut q_workers: Query<&mut WorkerState>,
                                                event: ResMut<KEvent>,
                                                mut placement_state: ResMut<NextState<PlacementState>>,
                                            | {
                                                if let EventType::Click(_) = event.event_type {
                                                    if let Some(selected_worker) = selected_worker.selected {
                                                        if let Ok(mut worker) = q_workers.get_mut(selected_worker) {
                                                            *worker = WorkerState::Working;
                                                        }
                                                    }
                                                }
                                            }
                                        )}
                                    />
                                );
                            } else {
                                constructor!(
                                    <NinePatchBundle
                                        styles={KStyle {
                                            width: Units::Pixels(32.0).into(),
                                            height: Units::Pixels(32.0).into(),
                                            ..default()
                                        }}
                                        nine_patch={NinePatch {
                                            handle: assets.load("Pause Icon.png"),
                                            ..default()
                                        }}
                                        on_event={OnEvent::new(
                                            |
                                                entity: In<Entity>,
                                                selected_worker: Res<SelectedWorker>,
                                                mut q_workers: Query<&mut WorkerState>,
                                                event: ResMut<KEvent>,
                                                mut placement_state: ResMut<NextState<PlacementState>>,
                                            | {
                                                if let EventType::Click(_) = event.event_type {
                                                    if let Some(selected_worker) = selected_worker.selected {
                                                        if let Ok(mut worker) = q_workers.get_mut(selected_worker) {
                                                            *worker = WorkerState::Paused;
                                                        }
                                                    }
                                                }
                                            }
                                        )}
                                    />
                                );
                            }
                        }
                        <NinePatchBundle
                            styles={KStyle {
                                width: Units::Pixels(32.0).into(),
                                height: Units::Pixels(32.0).into(),
                                ..default()
                            }}
                            nine_patch={NinePatch {
                                handle: assets.load("Skip Icon.png"),
                                ..default()
                            }}
                            on_event={OnEvent::new(
                                |
                                    entity: In<Entity>,
                                    selected_worker: Res<SelectedWorker>,
                                    mut q_jobs: Query<&mut Job>,
                                    event: ResMut<KEvent>,
                                    mut placement_state: ResMut<NextState<PlacementState>>,
                                | {
                                    if let EventType::Click(_) = event.event_type {
                                        if let Some(selected_worker) = selected_worker.selected {
                                            if let Ok(mut job) = q_jobs.get_mut(selected_worker) {
                                                let current_job = job.current_job.unwrap_or_default();
                                                if let Some(active_job) = job.path.get_mut(current_job) {
                                                    active_job.job_status = JobStatus::Completed;
                                                }
                                            }
                                        }
                                    }
                                }
                            )}
                        />
                    </ElementBundle>
                    {
                        for job_path in job.path.iter() {
                            constructor!(
                                <TextWidgetBundle
                                    text={TextProps {
                                        content: job_path.action.variant_name().to_owned(),
                                        ..default()
                                    }}
                                    styles={KStyle {
                                        color: Color::rgb(0.0, 0.0, 0.0).into(),
                                        font_size: StyleProp::Value(22.0),
                                        left: StyleProp::Value(Units::Stretch(1.0)),
                                        right: StyleProp::Value(Units::Stretch(1.0)),
                                        ..default()
                                    }}
                                />
                            );
                        }
                    }
                </NinePatchBundle>
            );
        }
    }
    true
}
