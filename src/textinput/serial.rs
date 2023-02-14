//! `serde` compatible version of the text input theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextInput {
    /// Active state.
    pub active: TextInputComponent,

    /// Hovered state.
    pub hovered: TextInputComponent,

    /// Focused state.
    pub focused: TextInputComponent,

    /// Placeholder color.
    pub placeholder: String,

    /// Value color.
    pub value: String,

    /// Selection color.
    pub selection: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextInputState {
    /// Key to the background color.
    pub background: String,

    /// Key to the border theme.
    pub border: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum TextInputComponent {
    /// The button state is defined.
    Defined(TextInputState),

    /// The button state is inherited from another button theme.
    Inherited(String),

    /// The button state is not defined.
    None,
}
