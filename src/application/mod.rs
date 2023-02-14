use iced::application::StyleSheet;

use crate::{Color, Theme};

pub(crate) mod serial;

#[derive(Clone, Default, Debug)]
pub struct Application {
    pub background_color: Color,
    pub text_color: Color,
}

impl Application {
    pub(crate) fn create(serial: &serial::Application, theme: &Theme) -> Result<Self, ()> {
        let bg = theme
            .color
            .get(serial.background_color.as_str())
            .ok_or_else(|| ())?;
        let text = theme
            .color
            .get(serial.background_color.as_str())
            .ok_or_else(|| ())?;

        Ok(Self {
            background_color: bg.clone(),
            text_color: text.clone(),
        })
    }
}

impl StyleSheet for Application {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: self.background_color.into(),
            text_color: self.text_color.into(),
        }
    }
}
