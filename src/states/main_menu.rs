use amethyst::{core::ecs, input, prelude::*, ui};
use contracts::*;

use crate::events;

use super::{Game, GameStateOptions};

const SINGLE_PLAYER_BUTTON_ID: &str = "single_player_button";
const MULTIPLAYER_BUTTON_ID: &str = "multiplayer_button";
const CREDITS_BUTTON_ID: &str = "credits_button";
const HELP_BUTTON_ID: &str = "help_button";
const EXIT_ID_BUTTON_ID: &str = "exit_button";

/// Shows the main menu UI widgets.
pub struct MainMenu {
    // The UI root entity.
    ui_root: Option<ecs::Entity>,
}

impl<'a, 'b> MainMenu {
    pub fn new() -> Self {
        Self { ui_root: None }
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
        if ui_event.event_type == ui::UiEventType::Click {
            data.world.exec(|ui_finder: ui::UiFinder<'_>| {
                // Determine which handler to call by comparing the target entity with
                // the those corresponding to our buttons.
                if Some(ui_event.target) == ui_finder.find(SINGLE_PLAYER_BUTTON_ID) {
                    self.on_single_player_button_click()
                } else if Some(ui_event.target) == ui_finder.find(MULTIPLAYER_BUTTON_ID) {
                    self.on_multiplayer_button_click()
                } else if Some(ui_event.target) == ui_finder.find(CREDITS_BUTTON_ID) {
                    self.on_credits_button_click()
                } else if Some(ui_event.target) == ui_finder.find(HELP_BUTTON_ID) {
                    self.on_help_button_click()
                } else if Some(ui_event.target) == ui_finder.find(EXIT_ID_BUTTON_ID) {
                    self.on_exit_button_click()
                } else {
                    Trans::None
                }
            })
        } else {
            Trans::None
        }
    }

    fn on_single_player_button_click(&mut self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        // TODO: Switch to the single player menus state so the user can configure
        // the single player options.
        Trans::None
    }

    fn on_multiplayer_button_click(&mut self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        // Start a new multiplayer game.
        Trans::Switch(Box::new(Game::new(GameStateOptions::Multiplayer)))
    }

    fn on_credits_button_click(&mut self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        // TODO: Switch to the credits state.
        Trans::None
    }

    fn on_help_button_click(&mut self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        // TODO: Show the player manual using the system's default browser.
        Trans::None
    }

    fn on_exit_button_click(&mut self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        // Exit the game.
        Trans::Quit
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, events::StateEvent> for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        log::info!("Opened main menu.");

        self.ui_root = Some(
            data.world
                .exec(|mut creator: ui::UiCreator<'_>| creator.create("ui/main_menu.ron", ())),
        );
    }

    #[post(self.ui_root == None)]
    fn on_stop(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to close main menu.");
        }
        self.ui_root = None;

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
