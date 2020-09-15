use amethyst::{core::ecs, core::timing::Time, input, prelude::*};
use contracts::*;
use open_ttt_lib as ttt;
use std::time::Duration;

use crate::components;
use crate::environments::*;
use crate::events;
use crate::resources;
use crate::ui;

use super::MainMenu;

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
    // Entities the game state owns that need deleted when the state exits.
    owned_entities: Vec<ecs::Entity>,
    game_controls: Option<ui::GameControls<Self, NextState>>,
}

impl<'a, 'b> Game {
    /// Creates a new game using the given the options.
    pub fn new(options: GameStateOptions) -> Self {
        Self {
            options,
            owned_entities: Vec::new(),
            game_controls: None,
        }
    }

    // Adds a local player to the world.
    fn create_local_player(&mut self, world: &mut World, player: components::Player) {
        let player_entity = world
            .create_entity()
            .with(player)
            .with(components::LocalPlayer)
            .build();

        self.owned_entities.push(player_entity);
    }

    // Adds an AI player to the world.
    fn create_ai_player(
        &mut self,
        world: &mut World,
        player: components::Player,
        difficulty: ttt::ai::Difficulty,
    ) {
        let mut ai_player_component = components::AiPlayer::new(difficulty);
        // A little bit of delay is added to the AI player to give the impression thinking
        // about the next move.
        ai_player_component.move_delay = Duration::from_secs_f32(0.4);
        let ai_player_entity = world
            .create_entity()
            .with(player)
            .with(ai_player_component)
            .build();

        self.owned_entities.push(ai_player_entity);
    }

    fn game_state_extra_information(&mut self) -> Vec<String> {
        match self.options {
            GameStateOptions::Multiplayer => vec![String::from("Multiplayer")],
            GameStateOptions::SinglePlayer(difficulty, _) => {
                let mut single_player_info = vec!["Single-player".to_string()];
                match difficulty {
                    ttt::ai::Difficulty::Easy => single_player_info.push("Easy".to_string()),
                    ttt::ai::Difficulty::Medium => single_player_info.push("Medium".to_string()),
                    ttt::ai::Difficulty::Hard => single_player_info.push("Hard".to_string()),
                    _ => (),
                };
                single_player_info
            }
        }
    }

    // Deletes all owned entities from the world.
    fn delete_owned_entities(&mut self, world: &mut World) {
        world
            .delete_entities(self.owned_entities.as_slice())
            .expect("Unable to game state entities.");

        self.owned_entities.clear();
    }

    // Updates the game based on the player event.
    fn handle_player_event(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
        player_event: &events::PlayerEvent,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        let mark_added = {
            let events::PlayerEvent::RequestMark(player, position) = *player_event;
            let mut game_logic = data.world.fetch_mut::<resources::GameLogic>();

            // Before doing the move, ensure it is the player's turn and the position selected is valid.
            if game_logic.game.can_move(position) && game_logic.is_players_move(&player) {
                // Update the game with the player's position and let systems know the time of this update.
                game_logic.game.do_move(position).unwrap();
                game_logic.last_move_time = data.world.fetch::<Time>().absolute_time();
                log::debug!("player: {:?} moved to position {:?}", player, position);
                log::debug!("game state: {:?}", game_logic.game.state());

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
            let environments = { data.world.write_resource::<Option<Environments>>().take() };

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

            // Show the game over button if the game is complete.
            if let Some(game_controls) = self.game_controls.as_mut() {
                if state.is_game_over() {
                    game_controls.show_game_over_button(data.world, Self::on_start_next_game);
                }
            }
        }

        Trans::None
    }

    fn is_start_next_game_key_down(&self, window_event: &events::WindowEvent) -> bool {
        // TODO: NumpadEnter does not seem to be working on Debian, try on Windows.
        input::is_key_down(window_event, input::VirtualKeyCode::Return)
            || input::is_key_down(window_event, input::VirtualKeyCode::NumpadEnter)
            || input::is_key_down(window_event, input::VirtualKeyCode::Space)
    }

    fn can_start_next_game(&self, world: &World) -> bool {
        let game_logic = world.read_resource::<resources::GameLogic>();
        game_logic.game.state().is_game_over()
    }

    // Handles window related events.
    fn handle_window_event(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
        window_event: &events::WindowEvent,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        if input::is_close_requested(window_event) {
            Trans::Quit
        } else if input::is_key_down(window_event, input::VirtualKeyCode::Escape) {
            Trans::Switch(Box::new(MainMenu::new()))
        } else if self.is_start_next_game_key_down(window_event)
            && self.can_start_next_game(data.world)
        {
            self.on_start_next_game(data.world).as_trans()
        } else {
            Trans::None
        }
    }

    // Handles UI related events.
    fn handle_ui_event(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
        ui_event: &events::UiEvent,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        if let Some(game_controls) = self.game_controls.as_mut() {
            if let Some(callback) = game_controls.handle_ui_event(data.world, ui_event) {
                let next_state = callback(self, data.world);
                return next_state.as_trans();
            }
        }
        Trans::None
    }

    // Called when the user wishes to start the next game.
    fn on_start_next_game(&mut self, world: &mut World) -> NextState {
        // Tell the game logic to start the next game.
        {
            let mut game_logic = world.fetch_mut::<resources::GameLogic>();
            game_logic.last_move_time = world.fetch::<Time>().absolute_time();
            game_logic.game.start_next_game();
        }

        // Show the next environment.
        let environments = { world.write_resource::<Option<Environments>>().take() };
        if let Some(mut environments) = environments {
            environments.show_random(world);
            // Be sure to return the environment when done.
            world
                .write_resource::<Option<Environments>>()
                .replace(environments);
        }

        // Hide the game over button.
        if let Some(game_controls) = self.game_controls.as_mut() {
            game_controls.hide_game_over_button(world);
        }

        NextState::None
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, events::StateEvent> for Game {
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
        let environments = { data.world.write_resource::<Option<Environments>>().take() };
        if let Some(mut environments) = environments {
            environments.show_random(data.world);
            data.world
                .write_resource::<Option<Environments>>()
                .replace(environments);
        }

        // Create the UI elements.
        let mut game_controls = ui::GameControls::new();
        game_controls.set_menu_button(data.world, |_, _| NextState::MainMenu);
        game_controls.set_status(data.world, self.game_state_extra_information());
        self.game_controls = Some(game_controls);
    }

    #[post(self.owned_entities.is_empty())]
    fn on_stop(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        // Remove entities we created from the world.
        self.delete_owned_entities(data.world);

        // Delete the game controls.
        if let Some(mut game_controls) = self.game_controls.take() {
            game_controls.delete(data.world);
        }

        let mut environments = { data.world.write_resource::<Option<Environments>>().take() };
        if let Some(mut environments) = environments {
            environments.delete_current(data.world);
            data.world
                .write_resource::<Option<Environments>>()
                .replace(environments);
        }

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
                self.handle_player_event(data, &player_event)
            }
            events::StateEvent::Window(window_event) => {
                self.handle_window_event(data, &window_event)
            }
            events::StateEvent::Ui(ui_event) => self.handle_ui_event(data, &ui_event),
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

// Helper type for selecting the next state to transition to.
enum NextState {
    None,
    MainMenu,
}

impl<'a, 'b> NextState {
    // Converts the next state variant into a state transition.
    fn as_trans(&self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        match self {
            Self::None => Trans::None,
            Self::MainMenu => Trans::Switch(Box::new(MainMenu::new())),
        }
    }
}
