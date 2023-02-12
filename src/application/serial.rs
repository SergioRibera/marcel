use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Application {
    pub background_color: String,
    pub text_color: String,
}
