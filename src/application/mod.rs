use crate::{Color, Theme};

pub mod serial;

#[derive(Clone, Default, Debug)]
pub struct Application {
    pub background_color: Color,
    pub text_color: Color,
}

impl Application {
    pub(crate) fn create(serial: &serial::Application, theme: &Theme) -> Result<Self, ()> {
        let bg = theme
            .color
            .get(&serial.background_color)
            .ok_or_else(|| ())?;
        let text = theme
            .color
            .get(&serial.background_color)
            .ok_or_else(|| ())?;

        Ok(Self {
            background_color: bg.clone(),
            text_color: text.clone(),
        })
    }
}
