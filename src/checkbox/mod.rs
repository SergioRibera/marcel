//! Checkbox theme.

pub mod serial;
mod style;

pub use self::serial::{Checkbox as Serial, CheckboxState as StateSerial, Component};

pub use self::style::Theme;
