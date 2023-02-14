//! Serial Button theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Button {
    /// Active state.
    pub(crate) active: ButtonComponent,

    /// Hovered state.
    pub(crate) hovered: ButtonComponent,

    /// Pressed state.
    pub(crate) pressed: ButtonComponent,

    /// Disabled state.
    pub(crate) disabled: ButtonComponent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ButtonState {
    /// Key to the background color.
    pub background: String,

    /// Key to the text color.
    pub text: String,

    /// Key to the border theme.
    pub border: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ButtonComponent {
    /// The theme is defined.
    Defined(ButtonState),

    /// The button state is inherited from another theme.
    Inherited(String),

    /// The theme is not defined.
    None,
}
