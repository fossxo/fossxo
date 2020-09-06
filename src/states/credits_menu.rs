use amethyst::{core::ecs, input, prelude::*, ui};
use contracts::*;
use open_ttt_lib as ttt;

use super::MainMenu;
use crate::events;

/// Shows the game's credits.
pub struct CreditsMenu {
    // The UI root entity.
    ui_root: Option<ecs::Entity>,
}

impl<'a, 'b> State<GameData<'a, 'b>, events::StateEvent> for CreditsMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        log::info!("Opened credits.");

        // self.ui_root = Some(
        //     data.world
        //         .exec(|mut creator: ui::UiCreator<'_>| creator.create("ui/main_menu.ron", ())),
        // );
    }

    #[post(self.ui_root == None)]
    fn on_stop(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        if let Some(root_entity) = self.ui_root.take() {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to close credits.");
        }

        log::info!("Closed credits.");
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

impl<'a, 'b> CreditsMenu {
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

    fn on_open_library_license_information(
        &mut self,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        // TODO: Open the player manual to the third party library page.
        Trans::None
    }

    fn on_back_button_click(&mut self) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        Trans::Switch(Box::new(MainMenu::new()))
    }
}
