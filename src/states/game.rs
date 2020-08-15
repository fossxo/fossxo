use amethyst::{core::timing::Time, prelude::*};
use open_ttt_lib as ttt;

use crate::components;
use crate::events;
use crate::resources;

/// Responsible for managing single-player and multiplayer games.
#[derive(Default)]
pub struct Game;

impl<'a, 'b> Game {
    fn handle_player_event(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
        player_event: events::PlayerEvent,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        let events::PlayerEvent::RequestMark(player, position) = player_event;
        let mut game_data = data.world.fetch_mut::<resources::GameData>();

        // Before doing the move, ensure it is the player's turn and the position selected is valid.
        if game_data.game.can_move(position) && game_data.is_players_move(&player) {
            // Update the game with the player's position and let systems know the time of this update.
            game_data.game.do_move(position).unwrap();
            game_data.last_move_time = data.world.fetch::<Time>().absolute_time();

            // TODO: Update the display.
            // TODO: check for game over?
            println!("Player: {:?} moved to position {:?}", player, position);
            println!("\n{}\n", game_data.game.board());
            println!("Game state: {:?}", game_data.game.state());
        }

        Trans::None
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, events::StateEvent> for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        log::info!("Started game.");

        // New game data is created ensuring any leftover in progress games are
        // destroyed.
        let game = resources::GameData::default();
        data.world.insert(game);

        // TODO: create players from the provided game settings.
        data.world
            .create_entity()
            .with(components::Player::X)
            .with(components::LocalPlayer)
            .build();

        // data.world
        //     .create_entity()
        //     .with(components::Player::O)
        //     .with(components::LocalPlayer)
        //     .build();

        // TODO: example of an AI player.
        data.world
            .create_entity()
            .with(components::Player::O)
            .with(components::AiPlayer::new(ttt::ai::Difficulty::Hard))
            .build();
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'a, 'b>>) {
        log::info!("Ended game.");
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
        event: events::StateEvent,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        // Determine which sub-event handler needs to be called.
        match event {
            events::StateEvent::Player(player_event) => {
                self.handle_player_event(data, player_event)
            }
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        data.data.update(&data.world);

        Trans::None
    }
}
