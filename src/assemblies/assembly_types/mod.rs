use bevy::ecs::system::EntityCommands;
use std::marker::PhantomData;
use paste::paste;

use crate::*;

pub mod assembly_templates;
use assembly_templates::*;

#[derive(Bundle)]
pub struct AssemblyBundle {
    pub marker: Assembly,
    pub solid: SolidEntity,
    pub tile_size: EntityTileSize,
    pub sprite: SpriteBundle
}
impl Default for AssemblyBundle {
    fn default() -> AssemblyBundle {
        AssemblyBundle {
            marker: Assembly,
            solid: SolidEntity,
            tile_size: EntityTileSize(IVec2::new(1, 1)),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            }
        }
    }
}

impl GetGhostBundle for AssemblyBundle {
    fn get_sprite_bundle(&self) -> SpriteBundle {
        self.sprite.clone()
    }
    fn get_tile_size(&self) -> Option<EntityTileSize> {
        Some(self.tile_size)
    }
}

pub trait AssemblySpawn<'a, 'w, 's> {
    fn spawn_bundle(
        &self,
        commands: &'a mut Commands<'w, 's>,
        position: Vec2
    ) -> EntityCommands<'w, 's, 'a>;
}

macro_rules! make_assembly_types {
    ($(($assembly_name:ident, $bundle:ident)),*) => {
        #[derive(Component, Debug, Resource, Reflect, Hash, PartialEq, Eq, Clone, Copy)]
        pub enum AssemblyType {
            $($assembly_name),*
        }
        impl Default for AssemblyType {
            fn default() -> Self {
                AssemblyType::PulpMill
            }
        }
        
        impl<'a, 'w, 's> AssemblySpawn<'a, 'w, 's> for AssemblyType {
            fn spawn_bundle(
                &self,
                commands: &'a mut Commands<'w, 's>,
                position: Vec2
            ) -> EntityCommands<'w, 's, 'a> {
                match self {
                    $(AssemblyType::$assembly_name => {
                        let mut bundle = $bundle::default();
                        bundle.sprite.transform.translation = Vec3::new(position.x, position.y, 1.0);
                        commands.spawn(bundle)
                    }),*
                }
            }
        }

        impl AssemblyType {
            pub fn get_tile_size(self) -> EntityTileSize {
                match self {
                    $(AssemblyType::$assembly_name => {
                        $bundle::default().tile_size
                    })*,
                }
            }
        }

        paste! {
            pub fn selected_assembly_hover(
                $(mut [<ev_ $assembly_name:snake>]: EventWriter<ShowHoverGhost<$bundle>>,)*
                selected: Res<SelectedAssembly>,
            ){
                match selected.selected {
                    $(AssemblyType::$assembly_name => {
                        println!("Selected {}", stringify!($assembly_name));
                        [<ev_ $assembly_name:snake>].send(ShowHoverGhost::<$bundle> {
                            bundle: PhantomData::<$bundle>
                        });
                    },)*
                }
            }
        }

        paste! {
            pub fn update_assembly_ghost(
                $(mut [<ev_ $assembly_name:snake>]: EventWriter<ShowHoverGhost<$bundle>>,)*
                mut ev_hide_ghost: EventWriter<HideHoverGhost>,
                selected: Res<SelectedAssembly>,
            ){
                if selected.is_changed() {
                    ev_hide_ghost.send(HideHoverGhost);
                    match selected.selected {
                        $(AssemblyType::$assembly_name => {
                            println!("Selected {}", stringify!($assembly_name));
                            [<ev_ $assembly_name:snake>].send(ShowHoverGhost::<$bundle> {
                                bundle: PhantomData::<$bundle>
                            });
                        },)*
                    }
                }
            }
        }
    };
}

make_assembly_types!(
    (PulpMill, PulpMillBundle),
    (PaperPress, PaperPressBundle),
    (PaperDrier, PaperDrierBundle)
);
