
use crate::*;

mod assembly;
pub use assembly::*;

mod power;
pub use power::*;

mod assembly_types;
pub use assembly_types::*;

mod assembly_production;
pub use assembly_production::*;

use self::assembly_types::assembly_templates::*;

pub struct AssembliesPlugin;
impl Plugin for AssembliesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayerState::Assemblies), selected_assembly_hover)
            .add_systems(Update, update_assembly_ghost.run_if(in_state(PlayerState::Assemblies)))
            .add_systems(OnExit(PlayerState::Assemblies),
                |mut ev_hide_ghost: EventWriter<HideHoverGhost>| {
                    ev_hide_ghost.send(HideHoverGhost);
                }
            )
            // TODO: Macro
            .add_systems(Update, show_hover_ghost::<WoodChipperBundle>)
            .add_systems(Update, show_hover_ghost::<PulpMachineBundle>)
            .add_systems(Update, show_hover_ghost::<PaperMachineBundle>)
            .add_systems(Update, show_hover_ghost::<SawMillBundle>)
            .add_event::<ShowHoverGhost::<WoodChipperBundle>>()
            .add_event::<ShowHoverGhost::<PulpMachineBundle>>()
            .add_event::<ShowHoverGhost::<PaperMachineBundle>>()
            .add_event::<ShowHoverGhost::<SawMillBundle>>()
            .add_systems(Update,
            (
                    (place_assembly).run_if(in_state(PlayerState::Assemblies)).run_if(in_state(PlacementState::Allowed)),
                    // input_toggle_assembly_mode,
                    refund_assembly,
                ).run_if(in_state(DayCycleState::Day)),
            )
            .add_systems(Update,
                (
                    produce_goods,
                    add_assembly_power_input,
                    show_assembly_progress_bars,
                    update_assembly_progress_bars,
                    assembly_power_display
                ).run_if(in_state(DayCycleState::Day)),
            )
            .add_systems(PreUpdate, (
                mouse_collision_system::<Assembly>,
                mouse_collision_system::<ContainerInputSelector>,
                mouse_collision_system::<ContainerOutputSelector>,
            ))
            .add_event::<GenericMouseCollisionEvent::<Assembly>>()
            .add_event::<GenericMouseCollisionEvent::<ContainerInputSelector>>()
            .add_event::<GenericMouseCollisionEvent::<ContainerOutputSelector>>()
            .add_event::<AssemblyPowerInput>()
            .register_type::<ItemIOContainer>()
            .register_type::<AssemblyPower>()
            .insert_resource(SelectedAssembly::default())
            .init_resource::<SelectedAssembly>()
        ;
    }
}
