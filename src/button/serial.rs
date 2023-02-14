//! Serial Button theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Button {
    /// Active state.
    pub(crate) active: Component,

    /// Hovered state.
    pub(crate) hovered: Component,

    /// Pressed state.
    pub(crate) pressed: Component,

    /// Disabled state.
    pub(crate) disabled: Component,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    /// Key to the background color.
    pub(crate) background: String,

    /// Key to the text color.
    pub(crate) text: String,

    /// Key to the border theme.
    pub(crate) border: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Component {
    /// The theme is defined.
    Defined(State),

    /// The button state is inherited from another theme.
    Inherited(String),

    /// The theme is not defined.
    None,
}
