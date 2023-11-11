use std::marker::PhantomData;

use crate::*;

mod jobs;
pub use jobs::*;

mod worker;
pub use worker::*;

mod error;
pub use error::*;
pub struct WorkerPlugin;

impl Plugin for WorkerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayerState::Workers),
                |mut ev_show_ghost: EventWriter<ShowHoverGhost<WorkerBundle>>| {
                    ev_show_ghost.send(ShowHoverGhost::<WorkerBundle> {
                        bundle: PhantomData::<WorkerBundle>
                    });
                }
            )
            .add_systems(OnExit(PlayerState::Workers),
                |mut ev_hide_ghost: EventWriter<HideHoverGhost>| {
                    ev_hide_ghost.send(HideHoverGhost);
                }
            )
            .add_systems(Update, show_hover_ghost::<WorkerBundle>)
            .add_event::<ShowHoverGhost::<WorkerBundle>>()
            .add_systems(Update, 
                (
                    (place_worker).run_if(in_state(PlayerState::Workers)),
                    input_toggle_worker_mode,
                    (job_mode_creation).run_if(in_state(PlayerState::Jobs)),
                    activate_job_mode_on_click,
                    worker_do_job,
                    move_towards_path,
                    set_path_to_tile,
                    iterate_path,
                    worker_iterate_jobs,
                    worker_path_to_next_job,
                    toggle_worker_state,
                    worker_pick_up_item,
                    worker_drop_item,
                    job_error_marker
                )
            )
            .add_systems(PreUpdate, 
                mouse_collision_system::<Worker>,
            )
            .add_event::<GenericMouseCollisionEvent::<Worker>>()
            .add_event::<MouseCollisionEvent>()
            .add_event::<WorkerPickUpItemEvent>()
            .add_event::<WorkerDropItemEvent>()
            .register_type::<Job>()
            .register_type::<JobStatus>()
            .register_type::<WorkerState>()
            .register_type::<MoveToTile>()
            .register_type::<JobError>()
            .insert_resource(SelectedWorker {
                selected: None
            });
    }
}
