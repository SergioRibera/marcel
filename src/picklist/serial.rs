//! `serde` compatible version of the pick list theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Picklist<'a> {
    /// Active state.
    #[serde(borrow)]
    pub active: StateComponent<'a>,

    /// Hovered state.
    #[serde(borrow)]
    pub hovered: StateComponent<'a>,

    /// Menu theme.
    #[serde(borrow)]
    pub menu: MenuComponent<'a>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State<'a> {
    /// Key to the background color.
    #[serde(borrow)]
    pub background: &'a str,

    /// Key to the text color.
    #[serde(borrow)]
    pub text: &'a str,

    /// Key to the placeholder color.
    #[serde(borrow)]
    pub placeholder: &'a str,

    /// Key to the border theme.
    #[serde(borrow)]
    pub border: &'a str,

    /// Handle color.
    #[serde(borrow)]
    pub handle: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Menu<'a> {
    /// Key to the background color.
    #[serde(borrow)]
    pub background: &'a str,

    /// Key to the text color.
    #[serde(borrow)]
    pub text: &'a str,

    /// Key to the border theme.
    #[serde(borrow)]
    pub border: &'a str,

    /// Key to the selected background color.
    #[serde(borrow)]
    pub sbackground: &'a str,

    /// Key to the selected text color.
    #[serde(borrow)]
    pub stext: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StateComponent<'a> {
    /// The button state is defined.
    #[serde(borrow)]
    Defined(State<'a>),

    /// The button state is inherited from another button theme.
    #[serde(borrow)]
    Inherited(&'a str),

    /// The button state is not defined.
    None,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MenuComponent<'a> {
    /// The button state is defined.
    #[serde(borrow)]
    Defined(Menu<'a>),

    /// The button state is inherited from another button theme.
    #[serde(borrow)]
    Inherited(&'a str),
}
