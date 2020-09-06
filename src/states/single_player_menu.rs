use amethyst::{core::ecs, input, prelude::*, ui};
use contracts::*;
use open_ttt_lib as ttt;

use crate::components::Player;
use crate::events;

use super::{Game, GameStateOptions, MainMenu};

/// Shows the single-player option UI widgets.
pub struct SinglePlayerMenu {
    // The UI root entity.
    ui_root: Option<ecs::Entity>,
    // The mark the player wishes to use.
    selected_player: Player,
}

impl<'a, 'b> State<GameData<'a, 'b>, events::StateEvent> for SinglePlayerMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        log::info!("Opened single-player menu.");

        // self.ui_root = Some(
        //     data.world
        //         .exec(|mut creator: ui::UiCreator<'_>| creator.create("ui/main_menu.ron", ())),
        // );
    }

    #[post(self.ui_root == None)]
    fn on_stop(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to close single-player menu.");
        }
        self.ui_root = None;

        log::info!("Closed single-player menu.");
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
        event: events::StateEvent,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        // Determine which sub-event handler needs to be called.
        match event {
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

impl<'a, 'b> SinglePlayerMenu {
    pub fn new() -> Self {
        Self {
            ui_root: None,
            selected_player: Player::X,
        }
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
        data: StateData<'_, GameData<'a, 'b>>,
        ui_event: &events::UiEvent,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        Trans::None
    }

    fn on_easy_button_click(&mut self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        self.start_single_player_game(ttt::ai::Difficulty::Easy)
    }

    fn on_medium_button_click(&mut self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        self.start_single_player_game(ttt::ai::Difficulty::Medium)
    }

    fn on_hard_button_click(&mut self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        self.start_single_player_game(ttt::ai::Difficulty::Hard)
    }

    fn on_back_button_click(&mut self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        Trans::Switch(Box::new(MainMenu::new()))
    }

    fn start_single_player_game(
        &mut self,
        difficulty: ttt::ai::Difficulty,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        let game_state = Game::new(GameStateOptions::SinglePlayer(
            difficulty,
            self.selected_player,
        ));
        Trans::Switch(Box::new(game_state))
    }
}
