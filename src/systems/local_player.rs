use amethyst::{
    core::shrev::EventChannel,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, Write},
    input::InputHandler,
};

use open_ttt_lib as ttt;

use crate::components::*;
use crate::events::{ActionBinding, InputBindingTypes, PlayerEvent};
use crate::resources::*;

/// Responsible for translating mouse clicks and keyboard button presses into
/// player events.
#[derive(SystemDesc)]
pub struct LocalPlayerSystem;

impl<'a> System<'a> for LocalPlayerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, LocalPlayer>,
        Read<'a, InputHandler<InputBindingTypes>>,
        Read<'a, GameData>,
        Read<'a, Grid>,
        Write<'a, EventChannel<PlayerEvent>>,
    );

    fn run(
        &mut self,
        (players, local_players, input, game_data, grid, mut channel): Self::SystemData,
    ) {
        for (player, _) in (&players, &local_players).join() {
            // Check if the key corresponding to a specific position is being pressed.
            for (position, _owner) in game_data.game.board().iter() {
                let request_move_at_position = input
                    .action_is_down(&position_to_action_binding(&position))
                    .unwrap_or(false);
                if request_move_at_position {
                    channel.single_write(PlayerEvent::RequestMark(*player, position));
                }
            }

            // Check if the player is requesting a move be placed at the mouse cursor.
            if input
                .action_is_down(&ActionBinding::PlaceMarkAtMouse)
                .unwrap_or(false)
            {
                if let Some(mouse_position) = input.mouse_position() {
                    let position = grid.position(&mouse_position);
                    channel.single_write(PlayerEvent::RequestMark(*player, position));
                }
            }
        }
    }
}

/// Converts the provided position to an action binding.
fn position_to_action_binding(position: &ttt::game::Position) -> ActionBinding {
    ActionBinding::PlaceMark(position.row, position.column)
}
