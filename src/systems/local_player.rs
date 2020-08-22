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
        Read<'a, GameLogic>,
        Read<'a, MousePosition>,
        Write<'a, EventChannel<PlayerEvent>>,
    );

    fn run(
        &mut self,
        (players, local_players, input, game_logic, mouse_position, mut channel): Self::SystemData,
    ) {
        for (player, _) in (&players, &local_players).join() {
            // Only allow player's whose turn it is to generate player events.
            if game_logic.is_players_move(player) {
                // Process the keyboard and mouse input for the player.
                if let Some(player_event) = process_keyboard_input(player, &game_logic, &input) {
                    channel.single_write(player_event);
                }
                if let Some(player_event) = process_mouse_input(player, &mouse_position, &input) {
                    channel.single_write(player_event);
                }
            }
        }
    }
}

fn process_keyboard_input(
    player: &Player,
    game_logic: &GameLogic,
    input: &InputHandler<InputBindingTypes>,
) -> Option<PlayerEvent> {
    // Check if the key corresponding to a specific position is being pressed, returning
    // a player event corresponding to that position.
    for (position, _owner) in game_logic.game.board().iter() {
        let request_move_at_position = input
            .action_is_down(&position_to_action_binding(&position))
            .unwrap_or(false);
        if request_move_at_position {
            return Some(PlayerEvent::RequestMark(*player, position));
        }
    }

    None
}

fn process_mouse_input(
    player: &Player,
    mouse_position: &MousePosition,
    input: &InputHandler<InputBindingTypes>,
) -> Option<PlayerEvent> {
    // Check if the player is requesting a move be placed at the mouse cursor.
    if input
        .action_is_down(&ActionBinding::PlaceMarkAtMouse)
        .unwrap_or(false)
    {
        Some(PlayerEvent::RequestMark(*player, mouse_position.grid))
    } else {
        None
    }
}

// Converts the provided position to an action binding.
fn position_to_action_binding(position: &ttt::game::Position) -> ActionBinding {
    ActionBinding::PlaceMark(position.row, position.column)
}
