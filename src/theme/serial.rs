//! Serial version of the theme.

use crate::serial::*;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Theme<'a> {
    /// Name of this theme.
    /// This can be used to index a set of themes inside a `Hashmap`.
    pub name: &'a str,

    /// Brief description of this theme.
    /// Used mainly as a helper in the serialized files.
    pub description: &'a str,

    /// General Application Theme
    pub application: Application<'a>,

    /// Maps name keys to border themes.
    pub border: HashMap<&'a str, Border<'a>>,

    // Maps name keys to button themes.
    pub button: HashMap<&'a str, Button<'a>>,

    /// Maps name keys to colors.
    pub color: HashMap<&'a str, Color>,

    /// Maps name keys to containers.
    pub container: HashMap<&'a str, Container<'a>>,

    /// Maps name keys to pane grids.
    pub panegrid: HashMap<&'a str, PaneGrid<'a>>,

    /// Maps name keys to picklists.
    pub picklist: HashMap<&'a str, Picklist<'a>>,

    /// Maps name keys to progress bar.
    pub progressbar: HashMap<&'a str, ProgressBar<'a>>,

    /// Maps name keys to scrollable.
    pub scrollable: HashMap<&'a str, Scrollable<'a>>,

    /// Maps name keys to text input.
    pub textinput: HashMap<&'a str, TextInput<'a>>,

    /// Maps name keys to tooltip.
    pub tooltip: HashMap<&'a str, Tooltip<'a>>,
}
