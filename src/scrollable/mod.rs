//! Button theme.

pub mod serial;

use crate::{Border, Color, Theme};

use iced_native::widget::scrollable::{
    style::{Scrollbar, Scroller},
    StyleSheet,
};

use serial::ScrollableComponent;

#[derive(Clone, Copy, Debug)]
pub struct Scrollable {
    /// State Themes of the scrollable.
    /// In order: active, hovered, dragging.
    pub state: [ScrollableState; 3],
}

impl Scrollable {
    /// Attempts to create a theme from its &serialized version.
    pub fn create(serial: &serial::Scrollable, theme: &Theme) -> Result<Self, ()> {
        // Get all the themes.
        let active = Self::state(&serial.active, theme, 0)?;
        let hovered = Self::state(&serial.hovered, theme, 1)?;
        let dragging = Self::state(&serial.dragging, theme, 2)?;

        // Find the first state theme that is not None.
        let default = match (active, hovered, dragging) {
            (Some(d), _, _) => d,
            (_, Some(d), _) => d,
            (_, _, Some(d)) => d,

            _ => return Err(()),
        };

        Ok(Scrollable {
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
                if dragging.is_some() {
                    dragging.unwrap()
                } else {
                    default
                },
            ],
        })
    }

    fn state(serial: &ScrollableComponent, theme: &Theme, index: usize) -> Result<Option<ScrollableState>, ()> {
        match &serial {
            ScrollableComponent::Defined(state) => Ok(Some(ScrollableState::from(&state, &theme)?)),

            ScrollableComponent::Inherited(name) => match theme.scrollable.get(name.as_str()) {
                Some(scrollable) => Ok(Some(scrollable.state[index].clone())),
                _ => Err(()),
            },

            ScrollableComponent::None => Ok(None),
        }
    }
}

impl StyleSheet for Scrollable {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Scrollbar {
        Scrollbar {
            background: Some(self.state[0].color.into()),
            border_radius: self.state[0].border.radius,
            border_width: self.state[0].border.width,
            border_color: self.state[0].border.color.into(),

            scroller: Scroller {
                color: self.state[0].scolor.into(),
                border_radius: self.state[0].sborder.radius,
                border_width: self.state[0].sborder.width,
                border_color: self.state[0].sborder.color.into(),
            },
        }
    }

    fn hovered(&self, _: &Self::Style) -> Scrollbar {
        Scrollbar {
            background: Some(self.state[1].color.into()),
            border_radius: self.state[1].border.radius,
            border_width: self.state[1].border.width,
            border_color: self.state[1].border.color.into(),

            scroller: Scroller {
                color: self.state[1].scolor.into(),
                border_radius: self.state[1].sborder.radius,
                border_width: self.state[1].sborder.width,
                border_color: self.state[1].sborder.color.into(),
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ScrollableState {
    /// Background color.
    pub color: Color,

    /// Border theme.
    pub border: Border,

    /// Scroller color.
    pub scolor: Color,

    /// Scroller border theme.
    pub sborder: Border,
}

impl ScrollableState {
    /// Attempts to create a theme from its &serialized version.
    fn from(serial: &serial::ScrollableState, theme: &Theme) -> Result<Self, ()> {
        // Get the scrollable color.
        let color = match theme.color.get(serial.color.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the scrollable border.
        let border = match theme.border.get(serial.border.as_str()) {
            Some(border) => *border,
            _ => return Err(()),
        };

        // Get the scroller color.
        let scolor = match theme.color.get(serial.scolor.as_str()) {
            Some(color) => *color,
            _ => return Err(()),
        };

        // Get the scroller border.
        let sborder = match theme.border.get(serial.sborder.as_str()) {
            Some(border) => *border,
            _ => return Err(()),
        };

        Ok(ScrollableState {
            color,
            border,
            scolor,
            sborder,
        })
    }
}
