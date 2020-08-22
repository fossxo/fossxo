//! Contains the game's environments.

mod debug_environment;
mod environment;

pub use self::environment::*;

// TODO: Make this private once the environments resource is implemented.
pub use self::debug_environment::*;

// pub struct Environments {}
