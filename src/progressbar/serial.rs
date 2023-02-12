//! `serde` compatible version of the container theme.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProgressBar<'a> {
    /// Key to the background color.
    pub background: &'a str,

    /// Key to the bar color.
    pub bar: &'a str,

    /// Border radius.
    pub radius: f32,
}
