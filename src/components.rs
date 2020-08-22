//! Contains the game's components.
//!
//! The game consists of entities that represents a single object. A component
//! represents one aspect of an entity and store the data related to that
//! aspect. Entities do not store any actual data but instead are associated
//! with one or more components.

use amethyst::ecs;
use open_ttt_lib as ttt;

/// The Player component stores if the player is playing as X or as O.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    /// Gets the other, or opposite, of the current player.
    pub fn opposite_player(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

impl ecs::Component for Player {
    type Storage = ecs::DenseVecStorage<Self>;
}

/// Tag component that indicates the player is local, e.g. using the keyboard
/// and mouse to make mark selections.
#[derive(Clone, Default)]
pub struct LocalPlayer;

impl ecs::Component for LocalPlayer {
    type Storage = ecs::NullStorage<Self>;
}

pub struct AiPlayer {
    /// The underlying AI opponent that performs the actual move logic.
    pub ai_opponent: ttt::ai::Opponent,

    /// The delay to wait before trying to move into a position.
    ///
    /// The move delay allows the game to simulate the AI taking time to think
    /// of where to place its mark. Otherwise, the AI would instantly choose a
    /// location.
    pub move_delay: core::time::Duration,
}

impl AiPlayer {
    pub fn new(difficulty: ttt::ai::Difficulty) -> Self {
        Self {
            ai_opponent: ttt::ai::Opponent::new(difficulty),
            move_delay: core::time::Duration::new(0, 0),
        }
    }
}

impl ecs::Component for AiPlayer {
    type Storage = ecs::DenseVecStorage<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_opposite_player_when_x_should_be_o() {
        let player = Player::X;

        let opposite_player = player.opposite_player();

        assert_eq!(opposite_player, Player::O);
    }

    #[test]
    fn player_opposite_player_when_o_should_be_x() {
        let player = Player::O;

        let opposite_player = player.opposite_player();

        assert_eq!(opposite_player, Player::X);
    }
}