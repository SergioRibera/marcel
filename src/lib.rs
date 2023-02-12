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
pub use button::{Button, State as ButtonState};
pub use color::Color;
pub use container::Container;
pub use panegrid::{PaneGrid, State as PaneGridState};
pub use picklist::{Menu as PicklistMenu, Picklist, State as PicklistState};
pub use progressbar::ProgressBar;
pub use scrollable::{Scrollable, State as ScrollableState};
pub use textinput::{State as TextInputState, TextInput};
pub use tooltip::Tooltip;

pub use theme::Theme;

pub mod serial {

    pub use crate::{
        application::serial::Application,
        border::serial::Border,
        button::serial::{Button, Component as ButtonComponent, State as ButtonState},
        container::serial::Container,
        panegrid::serial::{Component as PaneGridComponent, PaneGrid, State as PaneGridState},
        picklist::serial::{
            Menu as PicklistMenu, MenuComponent as PicklistMenuComponent, Picklist,
            State as PicklistState, StateComponent as PicklistStateComponent,
        },
        progressbar::serial::ProgressBar,
        scrollable::serial::{Scrollable, State as ScrollableState, Component as ScrollableComponent},
        textinput::serial::{State as TextInputState, TextInput, Component as TextInputComponent},
        tooltip::serial::Tooltip,
    };

    pub use crate::theme::serial::Theme;
}
