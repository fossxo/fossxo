use amethyst::prelude::*;

use crate::events;

/// Responsible for managing single-player and multiplayer games.
#[derive(Default)]
pub struct Game;

impl<'a, 'b> State<GameData<'a, 'b>, events::StateEvent> for Game {
    fn on_start(&mut self, _data: StateData<'_, GameData<'a, 'b>>) {
        log::info!("Started game.");
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'a, 'b>>) {
        log::info!("Ended game.");
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, events::StateEvent> {
        data.data.update(&data.world);

        Trans::None
    }
}
