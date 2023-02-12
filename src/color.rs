//! Serialized color.

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
pub struct Color(u8, u8, u8, f32);

impl Color {
    /// Default color black.
    pub const BLACK: Color = Color(0, 0, 0, 1.0);

    /// Default color red.
    pub const RED: Color = Color(255, 0, 0, 1.0);

    /// Default color blue.
    pub const BLUE: Color = Color(0, 0, 255, 1.0);

    /// Default color white.
    pub const WHITE: Color = Color(255, 255, 255, 1.0);
}

fn color_f32_to_u8(value: f32) -> u8 {
    (value * 255.).round().clamp(0., 255.) as u8
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        let srgb = value.parse::<css_color::Srgb>().unwrap();
        let r = color_f32_to_u8(srgb.red);
        let g = color_f32_to_u8(srgb.green);
        let b = color_f32_to_u8(srgb.blue);
        let a = srgb.alpha;
        Self(r, g, b, a)
    }
}

impl Into<iced::Color> for Color {
    fn into(self) -> iced::Color {
        let Color(r, g, b, a) = self;

        iced::Color::from_rgba8(r, g, b, a)
    }
}

impl Into<iced::Color> for &Color {
    fn into(self) -> iced::Color {
        iced::Color::from_rgba8(self.0, self.1, self.2, self.3)
    }
}

impl Into<iced::theme::Text> for Color {
    fn into(self) -> iced::theme::Text {
        iced::theme::Text::Color(self.into())
    }
}

impl Into<iced::theme::Text> for &Color {
    fn into(self) -> iced::theme::Text {
        iced::theme::Text::Color(self.into())
    }
}

impl Into<iced::Background> for Color {
    fn into(self) -> iced::Background {
        iced::Background::Color(self.into())
    }
}

impl Into<iced::Background> for &Color {
    fn into(self) -> iced::Background {
        iced::Background::Color(self.into())
    }
}

impl iced_native::widget::text::StyleSheet for Color {
    type Style = Self;

    fn appearance(&self, _: Self::Style) -> iced::widget::text::Appearance {
        iced::widget::text::Appearance {
            color: Some(self.into()),
        }
    }
}

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&format!(
            "R: {:>3} | G: {:>3} | B: {:>3} | A: {:.3}",
            self.0, self.1, self.2, self.3
        ))
    }
}

impl Default for Color {
    fn default() -> Self {
        Color(0, 0, 0, 1.0)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format!(
            "rgba({} {} {} {})",
            self.0, self.1, self.2, self.3
        ))
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ColorVisitor)
    }
}

struct ColorVisitor;
impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Error to deserialize Color")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Color::from(v.as_str()))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Color::from(v))
    }
}
