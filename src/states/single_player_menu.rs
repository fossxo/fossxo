use amethyst::{core::ecs, input, prelude::*};
use contracts::*;
use open_ttt_lib as ttt;

use crate::components::Player;
use crate::events;
use crate::ui;

use super::{Game, GameStateOptions, MainMenu};

/// Shows the single-player option UI widgets.
pub struct SinglePlayerMenu {
    menu: Option<ui::Menu<Self, NextState>>,
    // The mark the player wishes to use.
    selected_player: Player,
}

impl<'a, 'b> State<GameData<'a, 'b>, events::StateEvent> for SinglePlayerMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        log::info!("Opened single-player menu.");

        let mut menu = ui::Menu::new();
        menu.set_title(data.world, "Single Player");
        menu.set_close_button(data.world, "Back", Self::on_back_button_click);
        menu.add_button(data.world, "Easy", Self::on_easy_button_click);
        menu.add_button(data.world, "Medium", Self::on_medium_button_click);
        menu.add_button(data.world, "Hard", Self::on_hard_button_click);
        self.menu = Some(menu);
    }

    #[post(self.menu.is_none())]
    fn on_stop(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        if let Some(mut menu) = self.menu.take() {
            menu.delete(data.world);
        }

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
            menu: None,
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
        if let Some(menu) = self.menu.as_mut() {
            if let Some(callback) = menu.handle_ui_event(data.world, ui_event) {
                let next_state = callback(self, data.world);
                return next_state.as_trans();
            }
        }
        Trans::None
    }

    fn on_easy_button_click(&mut self, _world: &mut ecs::World) -> NextState {
        NextState::SinglePlayerGame(ttt::ai::Difficulty::Easy, self.selected_player)
    }

    fn on_medium_button_click(&mut self, _world: &mut ecs::World) -> NextState {
        NextState::SinglePlayerGame(ttt::ai::Difficulty::Medium, self.selected_player)
    }

    fn on_hard_button_click(&mut self, _world: &mut ecs::World) -> NextState {
        NextState::SinglePlayerGame(ttt::ai::Difficulty::Hard, self.selected_player)
    }

    fn on_back_button_click(&mut self, _world: &mut ecs::World) -> NextState {
        NextState::MainMenu
    }
}

// Helper type for selecting the next state to transition to.
enum NextState {
    None,
    SinglePlayerGame(ttt::ai::Difficulty, Player),
    MainMenu,
}

impl<'a, 'b> NextState {
    // Converts the next state variant into a state transition.
    fn as_trans(&self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        match self {
            Self::None => Trans::None,
            Self::SinglePlayerGame(difficulty, player) => {
                let game_state = Game::new(GameStateOptions::SinglePlayer(*difficulty, *player));
                Trans::Switch(Box::new(game_state))
            }
            Self::MainMenu => Trans::Switch(Box::new(MainMenu::new())),
        }
    }
}
