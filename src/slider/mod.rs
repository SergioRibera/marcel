//! Slider theme.

pub(crate) mod serial;
mod style;

pub use self::serial::{Component, Slider as Serial, SliderState as StateSerial};

pub use self::style::Theme;
