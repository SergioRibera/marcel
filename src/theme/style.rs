use super::Theme;
use iced::{
    widget::{button, container, text_input},
    Vector,
};

impl button::StyleSheet for Theme {
    type Style = String;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let t = self.button.get(style).unwrap();
        button::Appearance {
            shadow_offset: Vector::default(),
            background: Some(t.state[0].background.into()),
            border_radius: t.state[0].border.radius,
            border_width: t.state[0].border.width,
            border_color: t.state[0].border.color.into(),
            text_color: t.state[0].text.into(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let t = self.button.get(style).unwrap();
        button::Appearance {
            shadow_offset: Vector::default(),
            background: Some(t.state[1].background.into()),
            border_radius: t.state[1].border.radius,
            border_width: t.state[1].border.width,
            border_color: t.state[1].border.color.into(),
            text_color: t.state[1].text.into(),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let t = self.button.get(style).unwrap();
        button::Appearance {
            shadow_offset: Vector::default(),
            background: Some(t.state[2].background.into()),
            border_radius: t.state[2].border.radius,
            border_width: t.state[2].border.width,
            border_color: t.state[2].border.color.into(),
            text_color: t.state[2].text.into(),
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let t = self.button.get(style).unwrap();
        button::Appearance {
            shadow_offset: Vector::default(),
            background: Some(t.state[3].background.into()),
            border_radius: t.state[3].border.radius,
            border_width: t.state[3].border.width,
            border_color: t.state[3].border.color.into(),
            text_color: t.state[3].text.into(),
        }
    }
}

impl iced_native::widget::container::StyleSheet for Theme {
    type Style = String;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let t = self.container.get(style).unwrap();
        container::Appearance {
            text_color: None,
            background: None,
            border_radius: t.border.radius,
            border_width: t.border.width,
            border_color: t.border.color.into(),
        }
    }
}

impl text_input::StyleSheet for Theme {
    type Style = String;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let t = self.textinput.get(style).unwrap();
        text_input::Appearance {
            background: t.state[0].background.into(),
            border_radius: t.state[0].border.radius,
            border_width: t.state[0].border.width,
            border_color: t.state[0].border.color.into(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        let t = self.textinput.get(style).unwrap();
        text_input::Appearance {
            background: t.state[1].background.into(),
            border_radius: t.state[1].border.radius,
            border_width: t.state[1].border.width,
            border_color: t.state[1].border.color.into(),
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let t = self.textinput.get(style).unwrap();
        text_input::Appearance {
            background: t.state[2].background.into(),
            border_radius: t.state[2].border.radius,
            border_width: t.state[2].border.width,
            border_color: t.state[2].border.color.into(),
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        let t = self.textinput.get(style).unwrap();
        t.colors[0].into()
    }

    fn value_color(&self, style: &Self::Style) -> iced::Color {
        let t = self.textinput.get(style).unwrap();
        t.colors[1].into()
    }

    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        let t = self.textinput.get(style).unwrap();
        t.colors[2].into()
    }
}
