//! Tooltip theme.

pub(crate) mod serial;

use crate::{Border, Color, Theme};

use iced_native::widget::container::{Appearance, StyleSheet};

#[derive(Clone, Copy, Debug)]
pub struct Tooltip {
    /// Background color.
    pub background: Color,

    /// Text color.
    pub text: Color,

    /// Border theme.
    pub border: Border,
}

impl Tooltip {
    /// Attempts to create a theme from its &serialized version.
    pub(crate) fn create(serial: &serial::Tooltip, theme: &Theme) -> Result<Self, ()> {
        // Get the background color.
        let background = match theme.color.get(serial.background.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the text color.
        let text = match theme.color.get(serial.text.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the background color.
        let border = match theme.border.get(serial.border.as_str()) {
            Some(border) => *border,
            _ => return Err(()),
        };

        Ok(Tooltip {
            background,
            text,
            border,
        })
    }
}

impl StyleSheet for Tooltip {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            text_color: None,
            background: Some(self.background.into()),
            border_radius: self.border.radius,
            border_width: self.border.width,
            border_color: self.border.color.into(),
        }
    }
}
