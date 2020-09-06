//! Holds the game's states.

mod game;
mod loading;
mod main_menu;
mod single_player_menu;

pub use self::loading::*;

use self::game::*;
use self::main_menu::*;
use self::single_player_menu::*;
