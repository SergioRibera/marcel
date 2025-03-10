//! Button theme.

pub mod serial;

use crate::{Border, Color, Theme};

use iced::{
    widget::button::{Appearance, StyleSheet},
    Vector,
};

use serial::ButtonComponent;

#[derive(Clone, Copy, Debug)]
pub struct Button {
    /// State Themes of the button.
    /// In order: active, hovered, pressed, disabled.
    pub state: [ButtonState; 4],
}

impl Button {
    /// Attempts to create a theme from its &serialized version.
    pub fn create(serial: &serial::Button, theme: &Theme) -> Result<Self, ()> {
        // Get all the themes.
        let active = Self::state(&serial.active, theme, 0)?;
        let hovered = Self::state(&serial.hovered, theme, 1)?;
        let pressed = Self::state(&serial.pressed, theme, 2)?;
        let disabled = Self::state(&serial.disabled, theme, 3)?;

        // Find the first state theme that is not None.
        let default = match (active, hovered, pressed, disabled) {
            (Some(d), _, _, _) => d,
            (_, Some(d), _, _) => d,
            (_, _, Some(d), _) => d,
            (_, _, _, Some(d)) => d,

            _ => return Err(()),
        };

        Ok(Button {
            state: [
                if active.is_some() {
                    active.unwrap()
                } else {
                    default
                },
                if hovered.is_some() {
                    hovered.unwrap()
                } else {
                    default
                },
                if pressed.is_some() {
                    pressed.unwrap()
                } else {
                    default
                },
                if disabled.is_some() {
                    disabled.unwrap()
                } else {
                    default
                },
            ],
        })
    }

    fn state(serial: &ButtonComponent, theme: &Theme, index: usize) -> Result<Option<ButtonState>, ()> {
        match &serial {
            ButtonComponent::Defined(state) => Ok(Some(ButtonState::from(&state, &theme)?)),

            ButtonComponent::Inherited(name) => match theme.button.get(name.as_str()) {
                Some(button) => Ok(Some(button.state[index].clone())),
                _ => Err(()),
            },

            ButtonComponent::None => Ok(None),
        }
    }
}

impl StyleSheet for Button {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(self.state[0].background.into()),
            border_radius: self.state[0].border.radius,
            border_width: self.state[0].border.width,
            border_color: self.state[0].border.color.into(),
            text_color: self.state[0].text.into(),
        }
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(self.state[1].background.into()),
            border_radius: self.state[1].border.radius,
            border_width: self.state[1].border.width,
            border_color: self.state[1].border.color.into(),
            text_color: self.state[1].text.into(),
        }
    }

    fn pressed(&self, _: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(self.state[2].background.into()),
            border_radius: self.state[2].border.radius,
            border_width: self.state[2].border.width,
            border_color: self.state[2].border.color.into(),
            text_color: self.state[2].text.into(),
        }
    }

    fn disabled(&self, _: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(self.state[3].background.into()),
            border_radius: self.state[3].border.radius,
            border_width: self.state[3].border.width,
            border_color: self.state[3].border.color.into(),
            text_color: self.state[3].text.into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ButtonState {
    /// Background color.
    pub background: Color,

    /// Text color.
    pub text: Color,

    /// Border theme.
    pub border: Border,
}

impl ButtonState {
    /// Attempts to create a theme from its &serialized version.
    fn from(serial: &serial::ButtonState, theme: &Theme) -> Result<Self, ()> {
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

        Ok(ButtonState {
            background,
            text,
            border,
        })
    }
}
