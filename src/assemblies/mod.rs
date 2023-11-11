use std::marker::PhantomData;

use crate::*;

mod assembly;
pub use assembly::*;

mod power;
pub use power::*;

mod assembly_types;
pub use assembly_types::*;

mod assembly_production;
pub use assembly_production::*;

pub struct AssembliesPlugin;
impl Plugin for AssembliesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayerState::Assemblies),
                |mut ev_show_ghost: EventWriter<ShowHoverGhost<AssemblyBundle>>| {
                    ev_show_ghost.send(ShowHoverGhost::<AssemblyBundle> {
                        bundle: PhantomData::<AssemblyBundle>
                    });
                }
            )
            .add_systems(OnExit(PlayerState::Assemblies),
                |mut ev_hide_ghost: EventWriter<HideHoverGhost>| {
                    ev_hide_ghost.send(HideHoverGhost);
                }
            )
            .add_systems(Update, show_hover_ghost::<AssemblyBundle>)
            .add_event::<ShowHoverGhost::<AssemblyBundle>>()
            .add_systems(Update,
            (
                (place_assembly).run_if(in_state(PlayerState::Assemblies)),
                input_toggle_assembly_mode,
                refund_assembly,
            ))
            .add_systems(Update,
                (produce_goods, add_assembly_power_input)
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
            .insert_resource(SelectedAssembly { selected: AssemblyType::default() })
            .init_resource::<SelectedAssembly>()
        ;
    }
}
