//! `serde` compatible version of the text input theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextInput<'a> {
    /// Active state.
    pub active: Component<'a>,

    /// Hovered state.
    pub hovered: Component<'a>,

    /// Focused state.
    pub focused: Component<'a>,

    /// Placeholder color.
    pub placeholder: &'a str,

    /// Value color.
    pub value: &'a str,

    /// Selection color.
    pub selection: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State<'a> {
    /// Key to the background color.
    pub background: &'a str,

    /// Key to the border theme.
    pub border: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Component<'a> {
    /// The button state is defined.
    Defined(State<'a>),

    /// The button state is inherited from another button theme.
    Inherited(&'a str),

    /// The button state is not defined.
    None,
}
