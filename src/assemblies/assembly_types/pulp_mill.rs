use crate::*;

#[derive(Component)]
pub struct PulpMill;

#[derive(Bundle)]
pub struct PulpMillBundle {
    pub assembly: AssemblyBundle,
    pub marker: PulpMill
}
impl Default for PulpMillBundle {
    fn default() -> PulpMillBundle {
        PulpMillBundle {
            assembly: AssemblyBundle::default(),
            marker: PulpMill,
        }
    }
}