//! Contains the game's environments.

mod debug_environment;
mod environment;

pub use self::debug_environment::DebugOptions;
pub use self::environment::*;

use amethyst::prelude::*;
use rand::seq::SliceRandom;

use self::debug_environment::DebugEnvironment;
use crate::components;

/// Structure responsible for providing access and managing the all of the games environments.
pub struct Environments {
    // All the environments.
    environments: Vec<Box<dyn Environment + Send + Sync>>,

    // Index into the environments vector of the currently active environment.
    // When the index reaches the end the environments are re-shuffled.
    current_index: usize,

    // Environment used for showing on screen debug information. This is None if
    // the debug environment is not currently enabled.
    debug_environment: DebugEnvironment,
}

impl Environments {
    /// Creates the environments structure.
    pub fn new() -> Self {
        // For now, we use just the debug environment with all options enabled.
        // let environments :  = vec![Box::new(DebugEnvironment::new(DebugOptions::enable_all()))];

        Self {
            environments: vec![Box::new(DebugEnvironment::new(DebugOptions::enable_all()))],
            current_index: 0,
            debug_environment: DebugEnvironment::new(DebugOptions::disable_all()),
        }
    }

    /// Loads assets required for all the environments.
    pub fn load(&mut self, _world: &mut World) {
        // TODO: currently there is nothing to load. This will change as
        // environments become more interesting.
    }

    /// Shows a random environment.
    ///
    /// If there is an environment currently being shown, it is deleted before
    /// the new environment is created. A shuffle short is used that ensures
    /// every environment is shown once before allowing a repeat environment
    /// being shown.
    pub fn show_random(&mut self, world: &mut World) {
        self.delete_current(world);
        self.select_next_environment();
        self.current_environment().create(world);
        self.debug_environment.create(world);
    }

    /// Deletes the current environment and all its owned entities.
    ///
    /// This is useful when switching to the game menu. It is safe to call this
    /// method multiple times.
    pub fn delete_current(&mut self, world: &mut World) {
        self.current_environment().delete(world);
        self.debug_environment.delete(world);
    }

    /// Adds a mark to the current environment.
    pub fn add_mark(&mut self, world: &mut World, mark: &components::Mark) {
        self.current_environment().add_mark(world, mark);
        self.debug_environment.add_mark(world, mark);
    }

    /// Shows the game over related entities.
    pub fn game_over(&mut self, world: &mut World, outcome: OutcomeAffinity) {
        self.current_environment().game_over(world, outcome);
        self.debug_environment.game_over(world, outcome);
    }

    /// Allows usage of the special debug environment.
    ///
    /// When the debug environment is enabled, grid lines, marks, and other
    /// annotations are drawn on top of the current environment.
    pub fn debug(&mut self, world: &mut World, debug_options: &DebugOptions) {
        // Delete the old debug environment and build a new one.
        let was_alive = self.debug_environment.is_alive();
        self.debug_environment.delete(world);
        self.debug_environment = DebugEnvironment::new(*debug_options);

        // The new environment is only created if the old one was alive.
        if was_alive {
            self.debug_environment.create(world);
        }
    }

    // Gets a mutable reference to the current environment.
    fn current_environment(&mut self) -> &mut dyn Environment {
        self.environments[self.current_index].as_mut()
    }

    // Advances the `current_index` to the next environment, shuffling the
    // environments vector if necessary.
    fn select_next_environment(&mut self) {
        self.current_index += 1;
        // If the end of the environments have been reached, shuffle the
        // environments and start gain at the beginning of the collection.
        if self.current_index >= self.environments.len() {
            let mut rng = rand::thread_rng();
            self.environments.shuffle(&mut rng);
            self.current_index = 0;
        }
    }
}
