use crate::*;

mod assembly;
pub use assembly::*;

mod power;
pub use power::*;

mod ghost;
pub use ghost::*;

mod assembly_types;
pub use assembly_types::*;

pub struct AssembliesPlugin;
impl Plugin for AssembliesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayerState::Assemblies),
                |mut ev_show_ghost: EventWriter<ShowAssemblyGhost>| {
                    ev_show_ghost.send(ShowAssemblyGhost);
                }
            )
            .add_systems(OnExit(PlayerState::Assemblies),
                |mut ev_hide_ghost: EventWriter<HideAssemblyGhost>| {
                    ev_hide_ghost.send(HideAssemblyGhost);
                }
            )
            .add_systems(Update,
            (
                (place_assembly, assembly_ghost_tracking).run_if(in_state(PlayerState::Assemblies)),
                input_toggle_assembly_mode,
                show_assembly_ghost,
                hide_assembly_ghost
            ))
            .add_systems(Update,
                (produce_goods, add_assembly_power_input)
            )
            .add_systems(PreUpdate, (
                mouse_collision_system::<Assembly>,
            ))
            .add_event::<GenericMouseCollisionEvent::<Assembly>>()
            .add_event::<AssemblyPowerInput>()
            .add_event::<HideAssemblyGhost>()
            .add_event::<ShowAssemblyGhost>();
    }
}
