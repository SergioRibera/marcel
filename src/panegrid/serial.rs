//! `serde` compatible version of the pane grid theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaneGrid<'a> {
    /// Picked state.
    #[serde(borrow)]
    pub picked: Component<'a>,

    /// Hovered state.
    #[serde(borrow)]
    pub hovered: Component<'a>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State<'a> {
    /// The color of the line.
    #[serde(borrow)]
    pub color: &'a str,

    /// Width of the line.
    pub width: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Component<'a> {
    /// The line state is defined.
    #[serde(borrow)]
    Defined(State<'a>),

    /// The line state is inherited from another pane grid theme.
    #[serde(borrow)]
    Inherited(&'a str),

    /// The line state is not defined.
    None,
}
