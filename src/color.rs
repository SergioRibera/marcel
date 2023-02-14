//! Serialized color.

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8, pub f32);

impl Color {
    pub const ALICEBLUE: Color = Color(240, 248, 255, 1.);
    pub const ANTIQUEWHITE: Color = Color(250, 235, 215, 1.);
    pub const AQUA: Color = Color(0, 255, 255, 1.);
    pub const AQUAMARINE: Color = Color(127, 255, 212, 1.);
    pub const AZURE: Color = Color(240, 255, 255, 1.);
    pub const BEIGE: Color = Color(245, 245, 220, 1.);
    pub const BISQUE: Color = Color(255, 228, 196, 1.);
    pub const BLACK: Color = Color(0, 0, 0, 1.);
    pub const BLANCHEDALMOND: Color = Color(255, 235, 205, 1.);
    pub const BLUE: Color = Color(0, 0, 255, 1.);
    pub const BLUEVIOLET: Color = Color(138, 43, 226, 1.);
    pub const BROWN: Color = Color(165, 42, 42, 1.);
    pub const BURLYWOOD: Color = Color(222, 184, 135, 1.);
    pub const CADETBLUE: Color = Color(95, 158, 160, 1.);
    pub const CHARTREUSE: Color = Color(127, 255, 0, 1.);
    pub const CHOCOLATE: Color = Color(210, 105, 30, 1.);
    pub const CORAL: Color = Color(255, 127, 80, 1.);
    pub const CORNFLOWERBLUE: Color = Color(100, 149, 237, 1.);
    pub const CORNSILK: Color = Color(255, 248, 220, 1.);
    pub const CRIMSON: Color = Color(220, 20, 60, 1.);
    pub const CYAN: Color = Color(0, 255, 255, 1.);
    pub const DARKBLUE: Color = Color(0, 0, 139, 1.);
    pub const DARKCYAN: Color = Color(0, 139, 139, 1.);
    pub const DARKGOLDENROD: Color = Color(184, 134, 11, 1.);
    pub const DARKGRAY: Color = Color(169, 169, 169, 1.);
    pub const DARKGREEN: Color = Color(0, 100, 0, 1.);
    pub const DARKGREY: Color = Color(169, 169, 169, 1.);
    pub const DARKKHAKI: Color = Color(189, 183, 107, 1.);
    pub const DARKMAGENTA: Color = Color(139, 0, 139, 1.);
    pub const DARKOLIVEGREEN: Color = Color(85, 107, 47, 1.);
    pub const DARKORANGE: Color = Color(255, 140, 0, 1.);
    pub const DARKORCHID: Color = Color(153, 50, 204, 1.);
    pub const DARKRED: Color = Color(139, 0, 0, 1.);
    pub const DARKSALMON: Color = Color(233, 150, 122, 1.);
    pub const DARKSEAGREEN: Color = Color(143, 188, 143, 1.);
    pub const DARKSLATEBLUE: Color = Color(72, 61, 139, 1.);
    pub const DARKSLATEGRAY: Color = Color(47, 79, 79, 1.);
    pub const DARKSLATEGREY: Color = Color(47, 79, 79, 1.);
    pub const DARKTURQUOISE: Color = Color(0, 206, 209, 1.);
    pub const DARKVIOLET: Color = Color(148, 0, 211, 1.);
    pub const DEEPPINK: Color = Color(255, 20, 147, 1.);
    pub const DEEPSKYBLUE: Color = Color(0, 191, 255, 1.);
    pub const DIMGRAY: Color = Color(105, 105, 105, 1.);
    pub const DIMGREY: Color = Color(105, 105, 105, 1.);
    pub const DODGERBLUE: Color = Color(30, 144, 255, 1.);
    pub const FIREBRICK: Color = Color(178, 34, 34, 1.);
    pub const FLORALWHITE: Color = Color(255, 250, 240, 1.);
    pub const FORESTGREEN: Color = Color(34, 139, 34, 1.);
    pub const FUCHSIA: Color = Color(255, 0, 255, 1.);
    pub const GAINSBORO: Color = Color(220, 220, 220, 1.);
    pub const GHOSTWHITE: Color = Color(248, 248, 255, 1.);
    pub const GOLD: Color = Color(255, 215, 0, 1.);
    pub const GOLDENROD: Color = Color(218, 165, 32, 1.);
    pub const GRAY: Color = Color(128, 128, 128, 1.);
    pub const GREEN: Color = Color(0, 128, 0, 1.);
    pub const GREENYELLOW: Color = Color(173, 255, 47, 1.);
    pub const GREY: Color = Color(128, 128, 128, 1.);
    pub const HONEYDEW: Color = Color(240, 255, 240, 1.);
    pub const HOTPINK: Color = Color(255, 105, 180, 1.);
    pub const INDIANRED: Color = Color(205, 92, 92, 1.);
    pub const INDIGO: Color = Color(75, 0, 130, 1.);
    pub const IVORY: Color = Color(255, 255, 240, 1.);
    pub const KHAKI: Color = Color(240, 230, 140, 1.);
    pub const LAVENDER: Color = Color(230, 230, 250, 1.);
    pub const LAVENDERBLUSH: Color = Color(255, 240, 245, 1.);
    pub const LAWNGREEN: Color = Color(124, 252, 0, 1.);
    pub const LEMONCHIFFON: Color = Color(255, 250, 205, 1.);
    pub const LIGHTBLUE: Color = Color(173, 216, 230, 1.);
    pub const LIGHTCORAL: Color = Color(240, 128, 128, 1.);
    pub const LIGHTCYAN: Color = Color(224, 255, 255, 1.);
    pub const LIGHTGOLDENRODYELLOW: Color = Color(250, 250, 210, 1.);
    pub const LIGHTGRAY: Color = Color(211, 211, 211, 1.);
    pub const LIGHTGREEN: Color = Color(144, 238, 144, 1.);
    pub const LIGHTGREY: Color = Color(211, 211, 211, 1.);
    pub const LIGHTPINK: Color = Color(255, 182, 193, 1.);
    pub const LIGHTSALMON: Color = Color(255, 160, 122, 1.);
    pub const LIGHTSEAGREEN: Color = Color(32, 178, 170, 1.);
    pub const LIGHTSKYBLUE: Color = Color(135, 206, 250, 1.);
    pub const LIGHTSLATEGRAY: Color = Color(119, 136, 153, 1.);
    pub const LIGHTSLATEGREY: Color = Color(119, 136, 153, 1.);
    pub const LIGHTSTEELBLUE: Color = Color(176, 196, 222, 1.);
    pub const LIGHTYELLOW: Color = Color(255, 255, 224, 1.);
    pub const LIME: Color = Color(0, 255, 0, 1.);
    pub const LIMEGREEN: Color = Color(50, 205, 50, 1.);
    pub const LINEN: Color = Color(250, 240, 230, 1.);
    pub const MAGENTA: Color = Color(255, 0, 255, 1.);
    pub const MAROON: Color = Color(128, 0, 0, 1.);
    pub const MEDIUMAQUAMARINE: Color = Color(102, 205, 170, 1.);
    pub const MEDIUMBLUE: Color = Color(0, 0, 205, 1.);
    pub const MEDIUMORCHID: Color = Color(186, 85, 211, 1.);
    pub const MEDIUMPURPLE: Color = Color(147, 112, 219, 1.);
    pub const MEDIUMSEAGREEN: Color = Color(60, 179, 113, 1.);
    pub const MEDIUMSLATEBLUE: Color = Color(123, 104, 238, 1.);
    pub const MEDIUMSPRINGGREEN: Color = Color(0, 250, 154, 1.);
    pub const MEDIUMTURQUOISE: Color = Color(72, 209, 204, 1.);
    pub const MEDIUMVIOLETRED: Color = Color(199, 21, 133, 1.);
    pub const MIDNIGHTBLUE: Color = Color(25, 25, 112, 1.);
    pub const MINTCREAM: Color = Color(245, 255, 250, 1.);
    pub const MISTYROSE: Color = Color(255, 228, 225, 1.);
    pub const MOCCASIN: Color = Color(255, 228, 181, 1.);
    pub const NAVAJOWHITE: Color = Color(255, 222, 173, 1.);
    pub const NAVY: Color = Color(0, 0, 128, 1.);
    pub const OLDLACE: Color = Color(253, 245, 230, 1.);
    pub const OLIVE: Color = Color(128, 128, 0, 1.);
    pub const OLIVEDRAB: Color = Color(107, 142, 35, 1.);
    pub const ORANGE: Color = Color(255, 165, 0, 1.);
    pub const ORANGERED: Color = Color(255, 69, 0, 1.);
    pub const ORCHID: Color = Color(218, 112, 214, 1.);
    pub const PALEGOLDENROD: Color = Color(238, 232, 170, 1.);
    pub const PALEGREEN: Color = Color(152, 251, 152, 1.);
    pub const PALETURQUOISE: Color = Color(175, 238, 238, 1.);
    pub const PALEVIOLETRED: Color = Color(219, 112, 147, 1.);
    pub const PAPAYAWHIP: Color = Color(255, 239, 213, 1.);
    pub const PEACHPUFF: Color = Color(255, 218, 185, 1.);
    pub const PERU: Color = Color(205, 133, 63, 1.);
    pub const PINK: Color = Color(255, 192, 203, 1.);
    pub const PLUM: Color = Color(221, 160, 221, 1.);
    pub const POWDERBLUE: Color = Color(176, 224, 230, 1.);
    pub const PURPLE: Color = Color(128, 0, 128, 1.);
    pub const REBECCAPURPLE: Color = Color(102, 51, 153, 1.);
    pub const RED: Color = Color(255, 0, 0, 1.);
    pub const ROSYBROWN: Color = Color(188, 143, 143, 1.);
    pub const ROYALBLUE: Color = Color(65, 105, 225, 1.);
    pub const SADDLEBROWN: Color = Color(139, 69, 19, 1.);
    pub const SALMON: Color = Color(250, 128, 114, 1.);
    pub const SANDYBROWN: Color = Color(244, 164, 96, 1.);
    pub const SEAGREEN: Color = Color(46, 139, 87, 1.);
    pub const SEASHELL: Color = Color(255, 245, 238, 1.);
    pub const SIENNA: Color = Color(160, 82, 45, 1.);
    pub const SILVER: Color = Color(192, 192, 192, 1.);
    pub const SKYBLUE: Color = Color(135, 206, 235, 1.);
    pub const SLATEBLUE: Color = Color(106, 90, 205, 1.);
    pub const SLATEGRAY: Color = Color(112, 128, 144, 1.);
    pub const SLATEGREY: Color = Color(112, 128, 144, 1.);
    pub const SNOW: Color = Color(255, 250, 250, 1.);
    pub const SPRINGGREEN: Color = Color(0, 255, 127, 1.);
    pub const STEELBLUE: Color = Color(70, 130, 180, 1.);
    pub const TAN: Color = Color(210, 180, 140, 1.);
    pub const TEAL: Color = Color(0, 128, 128, 1.);
    pub const THISTLE: Color = Color(216, 191, 216, 1.);
    pub const TOMATO: Color = Color(255, 99, 71, 1.);
    pub const TURQUOISE: Color = Color(64, 224, 208, 1.);
    pub const VIOLET: Color = Color(238, 130, 238, 1.);
    pub const WHEAT: Color = Color(245, 222, 179, 1.);
    pub const WHITE: Color = Color(255, 255, 255, 1.);
    pub const WHITESMOKE: Color = Color(245, 245, 245, 1.);
    pub const YELLOW: Color = Color(255, 255, 0, 1.);
    pub const YELLOWGREEN: Color = Color(154, 205, 50, 1.);
    pub const TRANSPARENT: Color = Color(0, 0, 0, 0.);
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
            "rgba({}, {}, {}, {})",
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
