//! Holds types used in the game's event system.
//!
//! This includes input events, window events, and game specific events.

use std::fmt;

use amethyst::{
    core::{
        shrev::{EventChannel, ReaderId},
        EventReader,
    },
    derive::EventReader,
    ecs::{Read, SystemData},
    input::{BindingTypes, InputEvent},
    prelude::*,
    ui::UiEvent,
    winit::Event as WindowEvent,
};
use serde::{Deserialize, Serialize};

/// Holds all the events that are sent to states.
#[derive(Clone, Debug, EventReader)]
#[reader(StateEventReader)]
pub enum StateEvent<T = InputBindingTypes>
where
    T: BindingTypes + Clone,
{
    /// Events sent by the winit window.
    Window(WindowEvent),

    /// Events sent by the ui system.
    Ui(UiEvent),

    /// Events sent by the input system.
    Input(InputEvent<T>),

    /// Events caused by player actions.
    Player(PlayerEvent),
}

/// Events caused by player actions.
#[derive(Clone, Debug, PartialEq)]
pub enum PlayerEvent {}

/// Inputs for controller axis.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {}

impl fmt::Display for AxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Inputs for specific actions / button presses.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {
    /// Places the mark at the indicated row and column
    PlaceMark(i32, i32),

    /// Requests a mark be placed at the current mouse pointer position.
    PlaceMarkAtMouse,
}

impl fmt::Display for ActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Input binding types.
#[derive(Clone, Debug, Default)]
pub struct InputBindingTypes;

impl BindingTypes for InputBindingTypes {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}
