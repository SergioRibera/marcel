//! `serde` compatible version of the scrollbar theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scrollable {
    /// Active state.
    pub active: ScrollableComponent,

    /// Hovered state.
    pub hovered: ScrollableComponent,

    /// Dragging state.
    pub dragging: ScrollableComponent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScrollableState {
    /// Key to the background color.
    pub color: String,

    /// Key to the border theme.
    pub border: String,

    /// Key to the scroller color.
    pub scolor: String,

    /// Key to the scroller border.
    pub sborder: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ScrollableComponent {
    /// The button state is defined.
    Defined(ScrollableState),

    /// The button state is inherited from another button theme.
    Inherited(String),

    /// The button state is not defined.
    None,
}
