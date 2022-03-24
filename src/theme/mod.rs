//! `Theme` represents a serializable collection of a theme.



use crate::{
    border::Serial as Border,
    button::Serial as Button,
    color::Color,
    container::Serial as Container,
};

use serde::{ Deserialize, Serialize };

use std::collections::HashMap;



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Theme {
    /// Name of this theme.
    /// This can be used to index a set of themes inside a `Hashmap`.
    pub name: String,

    /// Brief description of this theme.
    /// Used mainly as a helper in the serialized files.
    pub description: String,

    /// Maps all styles and themes to their component keys.
    /// By using this map, many different styles and themes can point to the 
    /// same base style or themes.
    /// Each key points to a name resolver and a brief decription of what 
    /// effects it produces in the GUI.
    pub styles: HashMap<String, (String, String)>,

    /// Maps name keys to border themes.
    pub border: HashMap<String, Border>,

    /// Maps name keys to button themes.
    pub button: HashMap<String, Button>,

    /// Maps name keys to colors.
    pub color: HashMap<String, Color>,

    /// Maps name keys to container themes.
    pub container: HashMap<String, Container>,
}
