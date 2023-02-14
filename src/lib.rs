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
pub use button::{Button, ButtonState};
pub use color::Color;
pub use container::Container;
pub use panegrid::{PaneGrid, PaneGridState};
pub use picklist::{Picklist, PicklistMenu, PicklistState};
pub use progressbar::ProgressBar;
pub use scrollable::{Scrollable, ScrollableState};
pub use textinput::{TextInput, TextInputState};
pub use tooltip::Tooltip;

pub use theme::Theme;

pub mod serial {

    pub use crate::{
        application::serial::Application,
        border::serial::Border,
        button::serial::{Button, ButtonComponent, ButtonState},
        container::serial::Container,
        panegrid::serial::{PaneGrid, PaneGridComponent, PaneGridState},
        picklist::serial::{
            Picklist, PicklistMenu, PicklistMenuComponent, PicklistState, PicklistStateComponent,
        },
        progressbar::serial::ProgressBar,
        scrollable::serial::{Scrollable, ScrollableComponent, ScrollableState},
        textinput::serial::{TextInput, TextInputComponent, TextInputState},
        tooltip::serial::Tooltip,
    };

    pub use crate::theme::serial::Theme;
}
