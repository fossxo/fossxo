//! Holds the game's systems.
//!
//! Systems contain small pieces of the game’s logic that is applied to
//! collections of components or resources.

use amethyst::{core::bundle::SystemBundle, ecs};

mod ai_player;
mod local_player;
mod mouse_hover_debug_box;
mod mouse_raycast;

use self::ai_player::*;
use self::local_player::*;
use self::mouse_hover_debug_box::*;
use self::mouse_raycast::*;

/// Bundle containing the game's main systems.
///
/// These systems provide the game's core logic.
#[derive(Debug)]
pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(
        self,
        _world: &mut ecs::World,
        builder: &mut ecs::DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst::Error> {
        builder.add(AiPlayerSystem, "ai_player_system", &[]);
        builder.add(
            MouseRaycastSystem,
            "mouse_raycast_system",
            &["input_system"],
        );
        builder.add(
            LocalPlayerSystem,
            "local_player_system",
            &["input_system", "mouse_raycast_system"],
        );
        Ok(())
    }
}

/// Bundle containing systems used by environments.
///
/// These systems provide the environment specific effects.
#[derive(Debug)]
pub struct EnvironmentsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for EnvironmentsBundle {
    fn build(
        self,
        _world: &mut ecs::World,
        builder: &mut ecs::DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst::Error> {
        builder.add(
            MouseHoverDebugBoxSystem,
            "mouse_hover_debug_box_system",
            &["mouse_raycast_system"],
        );
        Ok(())
    }
}
