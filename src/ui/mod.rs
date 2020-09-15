//! Provides support for creating and managing UI widgets.
//!
//! This module provides a high level API to construct game menus and the in
//! game controls.

mod entity_observers;
mod game_controls;
mod menu;
mod style;

use entity_observers::*;
pub use game_controls::*;
pub use menu::*;
pub use style::*;
