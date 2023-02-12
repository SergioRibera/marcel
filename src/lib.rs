//! `painter` is a dynamic theme library for the `iced` GUI framework.
//! It contains a collection of `Style` conertable structures that can be
//! serialized using `serde`.

mod border;
mod button;
//pub mod checkbox;
mod application;
mod color;
mod container;
mod panegrid;
mod picklist;
mod progressbar;
//pub mod radio;
//pub mod rule;
mod scrollable;
//pub mod slider;
mod textinput;
mod tooltip;

mod theme;

pub use application::*;
pub use border::*;
pub use button::*;
pub use color::Color;
pub use container::*;
pub use panegrid::*;
pub use picklist::*;
pub use progressbar::*;
pub use scrollable::*;
pub use textinput::*;
pub use tooltip::*;

pub use theme::Theme;

pub mod serial {
    pub use crate::{
        application::serial::*, border::serial::*, button::serial::*,
        color::Color, container::serial::*, panegrid::serial::*,
        picklist::serial::*, progressbar::serial::*,
        scrollable::serial::*, textinput::serial::*, tooltip::serial::*,
    };

    pub use crate::theme::serial::Theme;
}
