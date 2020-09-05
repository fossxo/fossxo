use amethyst::{core::ecs, core::timing::Time, input, prelude::*, ui};
use contracts::*;
use open_ttt_lib as ttt;
use std::time::Duration;

use crate::components;
use crate::environments::*;
use crate::events;
use crate::resources;

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
    // The UI root entity.
    // status_text: Option<ui::UiLabel>,
    menu_button: Option<ecs::Entity>,
}

impl<'a, 'b> Game {
    /// Creates a new game using the given the options.
    pub fn new(options: GameStateOptions) -> Self {
        Self {
            options,
            owned_entities: Vec::new(),
            // status_text: None,
            menu_button: None,
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

    fn create_status_text(&mut self, world: &mut World) {
        let (_id, label) = ui::UiLabelBuilder::<u32>::new("")
            .with_anchor(ui::Anchor::TopLeft)
            .with_align(ui::Anchor::MiddleLeft)
            .with_size(600.0, 84.0)
            // The position is slightly past the 1/2 way part so the text is not
            // right at the edge of the screen.
            .with_position(310.0, -20.0)
            .with_font_size(20.0)
            .with_text_color([1.0, 1.0, 1.0, 1.0])
            .build_from_world(world);
        self.owned_entities.push(label.text_entity);

        // Create the game state entity that gets updated to reflect the state of the game.
        let extra_information = self.game_state_extra_information();
        self.owned_entities.push(
            world
                .create_entity()
                .with(components::GameStateText {
                    ui_text: label.text_entity,
                    extra_information,
                })
                .build(),
        );
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

    fn create_menu_button(&mut self, world: &mut World) {
        // TODO: load a hamburger menu icon here instead of text.
        let size = 48.0;
        let (_button_id, button) = ui::UiButtonBuilder::<(), u32>::new("")
            .with_anchor(ui::Anchor::TopRight)
            .with_position(-size / 2.0, -size / 2.0)
            .with_size(size, size)
            .with_image(ui::UiImage::SolidColor([0.5, 0.5, 0.5, 1.0]))
            .with_hover_image(ui::UiImage::SolidColor([0.8, 0.8, 0.8, 1.0]))
            .build_from_world(&world);
        self.owned_entities.push(button.text_entity);
        self.owned_entities.push(button.image_entity);
        self.menu_button = Some(button.image_entity);
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
        }

        Trans::None
    }

    // Handles window related events.
    fn handle_window_event(
        &mut self,
        _data: StateData<'_, GameData<'a, 'b>>,
        window_event: &events::WindowEvent,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        if input::is_close_requested(window_event) {
            Trans::Quit
        } else if input::is_key_down(window_event, input::VirtualKeyCode::Escape) {
            Trans::Switch(Box::new(MainMenu::new()))
        } else {
            Trans::None
        }
    }

    // Handles UI related events.
    fn handle_ui_event(
        &mut self,
        _data: StateData<'_, GameData<'a, 'b>>,
        ui_event: &events::UiEvent,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        if ui_event.event_type == ui::UiEventType::Click {
            // Determine which handler to call by comparing the target entity with
            // the those corresponding to our buttons.
            if Some(ui_event.target) == self.menu_button {
                return self.on_menu_button_click();
            }
        }

        Trans::None
    }

    fn on_menu_button_click(&mut self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        Trans::Switch(Box::new(MainMenu::new()))
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
        self.create_status_text(data.world);
        self.create_menu_button(data.world);
    }

    #[post(self.owned_entities.is_empty())]
    fn on_stop(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        // Remove entities we created from the world.
        self.delete_owned_entities(data.world);

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
