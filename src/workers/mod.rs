use crate::*;

mod jobs;
use jobs::*;

mod worker;
use worker::*;
pub struct WorkerPlugin;

impl Plugin for WorkerPlugin {
    fn build(&self, app: &mut App) {
        app
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
                    worker_iterate_job,
                    worker_path_to_next_job,
                    toggle_worker_state,
                    worker_pick_up_item,
                    worker_drop_item
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
            .insert_resource(SelectedWorker {
                selected: None
            });
    }
}
