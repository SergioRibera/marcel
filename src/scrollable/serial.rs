//! `serde` compatible version of the scrollbar theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scrollable<'a> {
    /// Active state.
    #[serde(borrow)]
    pub active: Component<'a>,

    /// Hovered state.
    #[serde(borrow)]
    pub hovered: Component<'a>,

    /// Dragging state.
    #[serde(borrow)]
    pub dragging: Component<'a>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State<'a> {
    /// Key to the background color.
    #[serde(borrow)]
    pub color: &'a str,

    /// Key to the border theme.
    #[serde(borrow)]
    pub border: &'a str,

    /// Key to the scroller color.
    #[serde(borrow)]
    pub scolor: &'a str,

    /// Key to the scroller border.
    #[serde(borrow)]
    pub sborder: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Component<'a> {
    /// The button state is defined.
    #[serde(borrow)]
    Defined(State<'a>),

    /// The button state is inherited from another button theme.
    #[serde(borrow)]
    Inherited(&'a str),

    /// The button state is not defined.
    None,
}
