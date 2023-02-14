//! `serde` compatible version of the pane grid theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaneGrid {
    /// Picked state.
    pub picked: PaneGridComponent,

    /// Hovered state.
    pub hovered: PaneGridComponent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaneGridState {
    /// The color of the line.
    pub color: String,

    /// Width of the line.
    pub width: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum PaneGridComponent {
    /// The line state is defined.
    Defined(PaneGridState),

    /// The line state is inherited from another pane grid theme.
    Inherited(String),

    /// The line state is not defined.
    None,
}
