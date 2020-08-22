use amethyst::prelude::*;

use crate::events;
use crate::states;

/// Loads the assets needed for the game.
///
/// When the assets have finished loading the `Game` state switched to.
pub struct Loading;

impl<'a, 'b> State<GameData<'a, 'b>, events::StateEvent> for Loading {
    fn on_start(&mut self, _data: StateData<'_, GameData<'a, 'b>>) {
        log::info!("Started loading.");
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'a, 'b>>) {
        log::info!("Finished loading.");
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        data.data.update(&data.world);

        Trans::Switch(Box::new(states::Game::new(
            states::GameStateOptions::default(),
        )))
    }
}
