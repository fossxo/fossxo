//! Holds resources store global data that is not specific to any one entity.

use open_ttt_lib as ttt;

use crate::components;

/// Holds data associated with the tic-tac-toe game.
#[derive(Default)]
pub struct GameData {
    /// The tic-tac-toe game that contains the current game state.
    pub game: ttt::game::Game,

    /// Holds the time the game was last updated.
    pub last_move_time: core::time::Duration,
}

impl GameData {
    /// Helper function for knowing if it is the provided player's turn.
    pub fn is_players_move(&self, player: &components::Player) -> bool {
        match self.game.state() {
            ttt::game::State::PlayerXMove => *player == components::Player::X,
            ttt::game::State::PlayerOMove => *player == components::Player::O,
            _ => false,
        }
    }
}

/// Represents the game's grid.
#[derive(Default)]
pub struct Grid {}

impl Grid {
    /// Converts the provided screen coordinates into a game position.
    pub fn position(&self, _coordinates: &(f32, f32)) -> ttt::game::Position {
        ttt::game::Position { row: 0, column: 0 }
    }
}
