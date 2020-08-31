use amethyst::prelude::*;

use crate::components;

/// Defines a game environment.
///
/// Environments are responsible for managing required assets and spawning /
/// destroying entities as the game progresses.
pub trait Environment {
    /// Creates the initial set of entities in the environment.
    fn create(&mut self, world: &mut World);

    /// Deletes all entities from the environment.
    fn delete(&mut self, world: &mut World);

    /// Adds a mark to the environment.
    fn add_mark(&mut self, world: &mut World, mark: &components::Mark);

    /// Shows the game over related entities.
    fn game_over(&mut self, world: &mut World, outcome: OutcomeAffinity);

    /// Indicates if the environment is alive, that is has at least one entity.
    fn is_alive(&self) -> bool;
}

/// Holds environment related options.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct EnvironmentOptions {}

/// Represents the outcome of the game from the player's perspective.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OutcomeAffinity {
    /// The player won the game.
    // //Win,
    // /// The player lost the game.
    //Loss,
    // /// The game is a draw, no one has won.
    //CatsGame,

    /// The outcome is inconclusive.
    ///
    /// For example a multiplayer game always has a neutral outcome.
    Neutral,
}
