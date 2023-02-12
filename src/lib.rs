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

pub use application::Application;
pub use border::Border;
pub use button::Button;
pub use color::Color;
pub use container::Container;
pub use panegrid::PaneGrid;
pub use picklist::Picklist;
pub use progressbar::ProgressBar;
pub use scrollable::Scrollable;
pub use textinput::TextInput;
pub use tooltip::Tooltip;

pub use theme::Theme;

pub mod serial {
    pub use crate::{
        application::serial::Application, border::serial::Border, button::serial::Button,
        color::Color, container::serial::Container, panegrid::serial::PaneGrid,
        picklist::serial::Picklist, progressbar::serial::ProgressBar,
        scrollable::serial::Scrollable, textinput::serial::TextInput, tooltip::serial::Tooltip,
    };

    pub use crate::theme::serial::Theme;
}
