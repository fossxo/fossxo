use amethyst::{core::ecs, core::timing::Time, prelude::*, ui::UiCreator};
use contracts::*;
use open_ttt_lib as ttt;

use crate::components;
// use crate::environments;
use crate::environments::*;
use crate::events;
use crate::resources;
use amethyst::core::ecs::LazyUpdate;
use std::borrow::BorrowMut;
use std::ops::DerefMut;

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
    // The UI root entity.
    ui_root: Option<ecs::Entity>,
}

impl<'a, 'b> Game {
    /// Creates a new game using the given the options.
    pub fn new(options: GameStateOptions) -> Self {
        Self {
            options,
            players: Vec::new(),
            ui_root: None,
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
        let mark_added = {
            let events::PlayerEvent::RequestMark(player, position) = player_event;
            let mut game_logic = data.world.fetch_mut::<resources::GameLogic>();

            // Before doing the move, ensure it is the player's turn and the position selected is valid.
            if game_logic.game.can_move(position) && game_logic.is_players_move(&player) {
                // Update the game with the player's position and let systems know the time of this update.
                game_logic.game.do_move(position).unwrap();
                game_logic.last_move_time = data.world.fetch::<Time>().absolute_time();

                // TODO: Update the display.
                // TODO: check for game over?
                println!("Player: {:?} moved to position {:?}", player, position);
                println!("\n{}\n", game_logic.game.board());
                println!("Game state: {:?}", game_logic.game.state());

                Some((
                    components::Mark {
                        position,
                        owner: player,
                    },
                    game_logic.game.state(),
                ))
            } else {
                None
            }
        };

        if let Some((mark, state)) = mark_added {
            let mut environments = { data.world.write_resource::<Option<Environments>>().take() };

            if let Some(mut environments) = environments {
                environments.add_mark(data.world, &mark);
                if state.is_game_over() {
                    environments.game_over(data.world, OutcomeAffinity::Neutral);
                }
                // Be sure to return the environment when done.
                data.world
                    .write_resource::<Option<Environments>>()
                    .replace(environments);
            }
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
        let game_logic = resources::GameLogic::default();
        data.world.insert(game_logic);

        // Show the next environment. Note, this has to occur after replacing the game
        // resource as this is used by the created environment.
        let mut environments = { data.world.write_resource::<Option<Environments>>().take() };
        if let Some(mut environments) = environments {
            environments.show_random(data.world);
            data.world
                .write_resource::<Option<Environments>>()
                .replace(environments);
        }

        self.ui_root = Some(
            data.world
                .exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())),
        );
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
