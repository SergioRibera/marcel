//! `serde` compatible version of the container theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Container<'a> {
    /// Key to the background color.
    pub color: &'a str,

    /// Key to the border definition.
    pub border: &'a str,
}
