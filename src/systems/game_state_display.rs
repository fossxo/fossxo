use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    ui,
};
use open_ttt_lib as ttt;

use crate::components::*;
use crate::resources::*;

/// Updates UI text based on the state of the game.
#[derive(SystemDesc)]
pub struct GameStateDisplaySystem;

impl<'a> System<'a> for GameStateDisplaySystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, ui::UiText>,
        ReadStorage<'a, GameStateText>,
        Read<'a, GameLogic>,
    );

    fn run(&mut self, (mut ui_text, game_state_text_components, game_logic): Self::SystemData) {
        for (state_text_component,) in (&game_state_text_components,).join() {
            if let Some(text) = ui_text.get_mut(state_text_component.ui_text) {
                // Use vertical bars to separate each part of the text.
                let mut text_parts = state_text_component.extra_information.clone();
                let state_text = game_state_text(&game_logic.game).to_string();
                text_parts.push(state_text);
                text.text = text_parts.join(" | ");
            }
        }
    }
}

fn game_state_text(game: &ttt::game::Game) -> &str {
    match game.state() {
        ttt::game::State::PlayerXMove => "X's turn",
        ttt::game::State::PlayerOMove => "O's turn",
        ttt::game::State::PlayerXWin(_) => "Game Over: X wins!",
        ttt::game::State::PlayerOWin(_) => "Game Over: O wins!",
        ttt::game::State::CatsGame => "Game Over: Cat's game",
    }
}
