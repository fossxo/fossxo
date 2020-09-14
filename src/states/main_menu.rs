use amethyst::{core::ecs, input, prelude::*};
use contracts::*;

use crate::events;
use crate::file_io;
use crate::ui;

use super::*;

/// Shows the main menu UI widgets.
pub struct MainMenu {
    menu: Option<ui::Menu<Self, NextState>>,
}

impl<'a, 'b> MainMenu {
    pub fn new() -> Self {
        Self { menu: None }
    }

    // Handles window related events.
    fn handle_window_event(
        &mut self,
        _data: StateData<'_, GameData<'a, 'b>>,
        window_event: &events::WindowEvent,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        if input::is_close_requested(window_event) {
            Trans::Quit
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

    fn on_help_button_click(&mut self, _world: &mut ecs::World) -> NextState {
        match file_io::open_player_manual() {
            Ok(()) => log::info!("Opened player manual in the default browser."),
            Err(e) => log::error!("Unable to open the player manual. Error details: {}", e),
        }

        NextState::None
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, events::StateEvent> for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        log::info!("Opened main menu.");

        let mut menu = ui::Menu::new();
        menu.set_title(data.world, "FossXO");
        menu.set_close_button(data.world, "Exit", |_, _| NextState::Quit);
        menu.add_button(data.world, "Single-player", |_, _| {
            NextState::SinglePlayerMenu
        });
        menu.add_button(data.world, "Multiplayer", |_, _| NextState::MultiplayerGame);
        menu.add_separator(data.world);
        menu.add_button(data.world, "Credits", |_, _| NextState::CreditsMenu);
        menu.add_button(data.world, "Help", Self::on_help_button_click);
        self.menu = Some(menu);
    }

    #[post(self.menu.is_none())]
    fn on_stop(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        if let Some(mut menu) = self.menu.take() {
            menu.delete(data.world);
        }

        log::info!("Closed main menu.");
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

// Helper type for selecting the next state to transition to.
enum NextState {
    None,
    MultiplayerGame,
    SinglePlayerMenu,
    CreditsMenu,
    Quit,
}

impl<'a, 'b> NextState {
    // Converts the next state variant into a state transition.
    fn as_trans(&self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        match self {
            Self::None => Trans::None,
            Self::MultiplayerGame => {
                Trans::Switch(Box::new(Game::new(GameStateOptions::Multiplayer)))
            }
            Self::SinglePlayerMenu => Trans::Switch(Box::new(SinglePlayerMenu::new())),
            Self::CreditsMenu => Trans::Switch(Box::new(CreditsMenu::new())),
            Self::Quit => Trans::Quit,
        }
    }
}
