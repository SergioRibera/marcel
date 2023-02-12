//! `serde` compatible version of the tooltip theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tooltip<'a> {
    /// Key to the background color.
    pub background: &'a str,

    /// Key to the text color.
    pub text: &'a str,

    /// Key to the border definition.
    pub border: &'a str,
}
