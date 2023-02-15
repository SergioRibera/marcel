//! Pick list theme.

pub mod serial;

use crate::{Border, Color, Theme};

use iced::widget::pick_list::{Appearance, StyleSheet};

use serial::{PicklistMenuComponent, PicklistStateComponent};

#[derive(Clone, Copy, Debug)]
pub struct Picklist {
    /// Active state.
    pub state: [PicklistState; 2],

    /// Menu style.
    pub menu: PicklistMenu,
}

impl Picklist {
    /// Attempts to create a theme from its &serialized version.
    pub fn create(serial: &serial::Picklist, theme: &Theme) -> Result<Self, ()> {
        // Get all the themes.
        let active = Self::state(&serial.active, theme, 0)?;
        let hovered = Self::state(&serial.hovered, theme, 1)?;

        // Get the menu style.
        let menu = Self::menu(&serial.menu, theme)?;

        // Find the first state theme that is not None.
        let default = match (active, hovered) {
            (Some(d), _) => d,
            (_, Some(d)) => d,

            _ => return Err(()),
        };

        Ok(Picklist {
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
            ],

            menu,
        })
    }

    fn state(
        serial: &serial::PicklistStateComponent,
        theme: &Theme,
        index: usize,
    ) -> Result<Option<PicklistState>, ()> {
        match &serial {
            PicklistStateComponent::Defined(state) => Ok(Some(PicklistState::from(&state, &theme)?)),

            PicklistStateComponent::Inherited(name) => match theme.picklist.get(name.as_str()) {
                Some(picklist) => Ok(Some(picklist.state[index].clone())),
                _ => Err(()),
            },

            PicklistStateComponent::None => Ok(None),
        }
    }

    fn menu(serial: &serial::PicklistMenuComponent, theme: &Theme) -> Result<PicklistMenu, ()> {
        match &serial {
            PicklistMenuComponent::Defined(state) => Ok(PicklistMenu::from(&state, &theme)?),

            PicklistMenuComponent::Inherited(name) => match theme.picklist.get(name.as_str()) {
                Some(picklist) => Ok(picklist.menu.clone()),
                _ => Err(()),
            },
        }
    }
}

impl StyleSheet for Picklist {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            text_color: self.state[0].text.into(),
            placeholder_color: self.state[0].placeholder.into(),
            background: self.state[0].background.into(),
            border_radius: self.state[0].border.radius,
            border_width: self.state[0].border.width,
            border_color: self.state[0].border.color.into(),
            handle_color: self.state[0].handle.into(),
        }
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        Appearance {
            text_color: self.state[1].text.into(),
            placeholder_color: self.state[1].placeholder.into(),
            background: self.state[1].background.into(),
            border_radius: self.state[1].border.radius,
            border_width: self.state[1].border.width,
            border_color: self.state[1].border.color.into(),
            handle_color: self.state[1].handle.into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PicklistState {
    /// Background color.
    pub background: Color,

    /// Text color.
    pub text: Color,

    /// Placeholder color.
    pub placeholder: Color,

    /// Border theme.
    pub border: Border,

    /// Handle color.
    pub handle: Color,
}

impl PicklistState {
    /// Attempts to create a theme from its &serialized version.
    fn from(serial: &serial::PicklistState, theme: &Theme) -> Result<Self, ()> {
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

        // Get the placeholder color.
        let placeholder = match theme.color.get(serial.placeholder.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the background color.
        let border = match theme.border.get(serial.border.as_str()) {
            Some(border) => *border,
            _ => return Err(()),
        };

        let handle = match theme.color.get(serial.handle.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        Ok(PicklistState {
            background,
            text,
            placeholder,
            border,
            handle,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PicklistMenu {
    /// Key to the background color.
    pub background: [Color; 2],

    /// Key to the text color.
    pub text: [Color; 2],

    /// Key to the border theme.
    pub border: Border,
}

impl PicklistMenu {
    /// Attempts to create a theme from its &serialized version.
    fn from(serial: &serial::PicklistMenu, theme: &Theme) -> Result<Self, ()> {
        // Get the background colors.
        let background = match theme.color.get(serial.background.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        let sbackground = match theme.color.get(serial.sbackground.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the text color.
        let text = match theme.color.get(serial.text.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        let stext = match theme.color.get(serial.stext.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the background color.
        let border = match theme.border.get(serial.border.as_str()) {
            Some(border) => *border,
            _ => return Err(()),
        };

        Ok(PicklistMenu {
            background: [background, sbackground],
            text: [text, stext],
            border,
        })
    }
}
