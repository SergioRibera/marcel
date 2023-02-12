use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Application<'a> {
    pub background_color: &'a str,
    pub text_color: &'a str,
}
