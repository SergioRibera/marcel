//! `painter` is a dynamic theme library for the `iced` GUI framework.
//! It contains a collection of `Style` conertable structures that can be
//! serialized using `serde`.

mod border;
mod button;
//pub mod checkbox;
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
    pub use crate::border::serial::Border;
    pub use crate::button::serial::Button;
    pub use crate::color::Color;
    pub use crate::container::serial::Container;
    pub use crate::panegrid::serial::PaneGrid;
    pub use crate::picklist::serial::Picklist;
    pub use crate::progressbar::serial::ProgressBar;
    pub use crate::scrollable::serial::Scrollable;
    pub use crate::textinput::serial::TextInput;
    pub use crate::tooltip::serial::Tooltip;

    pub use crate::theme::serial::Theme;
}
