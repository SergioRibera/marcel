//! Serial Button theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Button<'a> {
    /// Active state.
    #[serde(borrow)]
    pub(super) active: Component<'a>,

    /// Hovered state.
    #[serde(borrow)]
    pub(super) hovered: Component<'a>,

    /// Pressed state.
    #[serde(borrow)]
    pub(super) pressed: Component<'a>,

    /// Disabled state.
    #[serde(borrow)]
    pub(super) disabled: Component<'a>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State<'a> {
    /// Key to the background color.
    #[serde(borrow)]
    pub(super) background: &'a str,

    /// Key to the text color.
    #[serde(borrow)]
    pub(super) text: &'a str,

    /// Key to the border theme.
    #[serde(borrow)]
    pub(super) border: &'a str,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Component<'a> {
    /// The theme is defined.
    #[serde(borrow)]
    Defined(State<'a>),

    /// The button state is inherited from another theme.
    #[serde(borrow)]
    Inherited(&'a str),

    /// The theme is not defined.
    None,
}
