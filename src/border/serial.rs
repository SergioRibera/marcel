//! `serde` compatible version of the border theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Border<'a> {
    /// Key to the border color.
    pub color: &'a str,

    /// Radius of the border.
    pub radius: f32,

    /// Width of the border.
    pub width: f32,
}
