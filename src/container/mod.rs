//! Container theme.

pub mod serial;

use crate::{Border, Color, Theme};

use iced::widget::container::{Appearance, StyleSheet};

#[derive(Clone, Copy, Debug)]
pub struct Container {
    /// Background of the container.
    pub color: Color,

    /// Border of the container.
    pub border: Border,
}

impl Container {
    /// Attempts to create a theme from its &serialized version.
    pub fn create(serial: &serial::Container, theme: &Theme) -> Result<Self, ()> {
        // Get the color of the container.
        let color = match theme.color.get(serial.color.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the border of the container.
        let border = match theme.border.get(serial.border.as_str()) {
            Some(border) => *border,
            _ => return Err(()),
        };

        Ok(Container { color, border })
    }
}

impl Into<iced::theme::Container> for Container {
    fn into(self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(self))
    }
}

impl StyleSheet for Container {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            text_color: None,
            background: Some(self.color.into()),
            border_radius: self.border.radius,
            border_width: self.border.width,
            border_color: self.border.color.into(),
        }
    }
}
