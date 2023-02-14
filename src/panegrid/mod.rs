//! Pick list theme.

pub mod serial;

use crate::{Color, Theme};

use iced_native::widget::pane_grid::{Line, StyleSheet};

use serial::Component;

#[derive(Clone, Copy, Debug)]
pub struct PaneGrid {
    /// Pane Grid states.
    pub state: [State; 2],
}

impl PaneGrid {
    /// Attempts to create a theme from its &serialized version.
    pub fn create(serial: &serial::PaneGrid, theme: &Theme) -> Result<Self, ()> {
        // Get all the themes.
        let picked = Self::state(&serial.picked, theme, 0)?;
        let hovered = Self::state(&serial.hovered, theme, 1)?;

        // Find the first state theme that is not None.
        let default = match (picked, hovered) {
            (Some(d), _) => d,
            (_, Some(d)) => d,

            _ => return Err(()),
        };

        Ok(PaneGrid {
            state: [
                if picked.is_some() {
                    picked.unwrap()
                } else {
                    default
                },
                if hovered.is_some() {
                    hovered.unwrap()
                } else {
                    default
                },
            ],
        })
    }

    fn state(serial: &serial::Component, theme: &Theme, index: usize) -> Result<Option<State>, ()> {
        match &serial {
            Component::Defined(state) => Ok(Some(State::from(&state, &theme)?)),

            Component::Inherited(name) => match theme.panegrid.get(name.as_str()) {
                Some(panegrid) => Ok(Some(panegrid.state[index].clone())),
                _ => Err(()),
            },

            Component::None => Ok(None),
        }
    }
}

impl StyleSheet for PaneGrid {
    type Style = iced::Theme;

    fn picked_split(&self, _: &Self::Style) -> Option<Line> {
        Some(Line {
            color: self.state[0].color.into(),
            width: self.state[0].width.into(),
        })
    }

    fn hovered_split(&self, _: &Self::Style) -> Option<Line> {
        Some(Line {
            color: self.state[1].color.into(),
            width: self.state[1].width.into(),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    /// Line color.
    pub color: Color,

    /// Line width.
    pub width: f32,
}

impl State {
    /// Attempts to create a theme from its &serialized version.
    fn from(serial: &serial::State, theme: &Theme) -> Result<Self, ()> {
        // Get the background color.
        let color = match theme.color.get(serial.color.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        Ok(State {
            color,
            width: serial.width,
        })
    }
}
