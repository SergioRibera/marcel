//! Progress bar theme.

pub(crate) mod serial;

use crate::{Color, Theme};

use iced_native::widget::progress_bar::{Appearance, StyleSheet};

#[derive(Clone, Copy, Debug)]
pub struct ProgressBar {
    /// Background color.
    pub background: Color,

    /// Bar color.
    pub bar: Color,

    /// Border radius.
    pub radius: f32,
}

impl ProgressBar {
    /// Attempts to create a theme from its &serialized version.
    pub(crate) fn create(serial: &serial::ProgressBar, theme: &Theme) -> Result<Self, ()> {
        // Get the color of the progress bar background.
        let background = match theme.color.get(serial.background.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the border of the progress bar bar.
        let bar = match theme.color.get(serial.bar.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        Ok(ProgressBar {
            background,
            bar,
            radius: serial.radius,
        })
    }
}

impl StyleSheet for ProgressBar {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: self.background.into(),
            bar: self.bar.into(),
            border_radius: self.radius,
        }
    }
}
