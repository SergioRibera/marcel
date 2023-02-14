//! `serde` compatible version of the pick list theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Picklist {
    /// Active state.
    pub active: PicklistStateComponent,

    /// Hovered state.
    pub hovered: PicklistStateComponent,

    /// Menu theme.
    pub menu: PicklistMenuComponent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PicklistState {
    /// Key to the background color.
    pub background: String,

    /// Key to the text color.
    pub text: String,

    /// Key to the placeholder color.
    pub placeholder: String,

    /// Key to the border theme.
    pub border: String,

    /// Handle color.
    pub handle: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PicklistMenu {
    /// Key to the background color.
    pub background: String,

    /// Key to the text color.
    pub text: String,

    /// Key to the border theme.
    pub border: String,

    /// Key to the selected background color.
    pub sbackground: String,

    /// Key to the selected text color.
    pub stext: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PicklistStateComponent {
    /// The button state is defined.
    Defined(PicklistState),

    /// The button state is inherited from another button theme.
    Inherited(String),

    /// The button state is not defined.
    None,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PicklistMenuComponent {
    /// The button state is defined.
    Defined(PicklistMenu),

    /// The button state is inherited from another button theme.
    Inherited(String),
}
