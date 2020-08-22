use amethyst::{core::ecs, core::timing::Time, prelude::*};
use contracts::*;
use open_ttt_lib as ttt;

use crate::components;
use crate::events;
use crate::resources;

// Number of players the game expects to work with.
const NUM_PLAYERS: usize = 2;

/// Holds the options for the game state.
pub enum GameStateOptions {
    /// Play a single player game with the provided difficulty and player mark.
    SinglePlayer(ttt::ai::Difficulty, components::Player),

    /// Play a multiplayer game.
    Multiplayer,
}

impl Default for GameStateOptions {
    fn default() -> Self {
        Self::SinglePlayer(ttt::ai::Difficulty::Medium, components::Player::X)
    }
}

/// Responsible for managing single-player and multiplayer games.
pub struct Game {
    options: GameStateOptions,
    players: Vec<ecs::Entity>,
}

impl<'a, 'b> Game {
    /// Creates a new game using the given the options.
    pub fn new(options: GameStateOptions) -> Self {
        Self {
            options,
            players: Vec::new(),
        }
    }

    // Adds a local player to the world.
    fn create_local_player(&mut self, world: &mut World, player: components::Player) {
        let player_entity = world
            .create_entity()
            .with(player)
            .with(components::LocalPlayer)
            .build();

        self.players.push(player_entity);
    }

    // Adds an AI player to the world.
    fn create_ai_player(
        &mut self,
        world: &mut World,
        player: components::Player,
        difficulty: ttt::ai::Difficulty,
    ) {
        let ai_player_entity = world
            .create_entity()
            .with(player)
            .with(components::AiPlayer::new(difficulty))
            .build();

        self.players.push(ai_player_entity);
    }

    // Deletes all players from the game.
    fn delete_payers(&mut self, world: &mut World) {
        let result = world.delete_entities(self.players.as_slice());
        if let Err(e) = result {
            log::error!("Unable to delete player entities from game. Details: {}", e);
        }
        self.players.clear();
    }

    // Updates the game based on the player event.
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
    #[post(self.players.len() == NUM_PLAYERS)]
    fn on_start(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        // Create the game's players based on the given options.
        match self.options {
            GameStateOptions::SinglePlayer(difficulty, player) => {
                log::info!(
                    "Started {:?} difficulty single-player game for player {:?}.",
                    difficulty,
                    player
                );
                self.create_local_player(data.world, player);
                self.create_ai_player(data.world, player.opposite_player(), difficulty);
            }
            GameStateOptions::Multiplayer => {
                log::info!("Started multiplayer game.");
                self.create_local_player(data.world, components::Player::X);
                self.create_local_player(data.world, components::Player::O);
            }
        };

        // New game data is created ensuring any leftover in progress games are
        // destroyed.
        let game = resources::GameData::default();
        data.world.insert(game);
    }

    #[post(self.players.is_empty())]
    fn on_stop(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        // Remove entities we created from the world.
        self.delete_payers(data.world);

        // Make a log entry of the game stopping.
        match self.options {
            GameStateOptions::SinglePlayer(difficulty, player) => {
                log::info!(
                    "Ended {:?} difficulty single-player game for player {:?}.",
                    difficulty,
                    player
                );
            }
            GameStateOptions::Multiplayer => {
                log::info!("Ended multiplayer game.");
            }
        }
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
