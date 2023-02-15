//! `Theme` represents a &serializable collection of a theme.

mod disp;
mod style;
pub mod serial;

use crate::*;

use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
pub struct Theme {
    /// Name of this theme.
    /// This can be used to index a set of themes inside a `Hashmap`.
    pub name: String,

    /// Brief description of this theme.
    /// Used mainly as a helper in the &serialized files.
    pub description: String,

    /// General Application Theme
    pub application: Application,

    /// Maps name keys to border themes.
    pub border: HashMap<String, Border>,

    // Maps name keys to button themes.
    pub button: HashMap<String, Button>,

    /// Maps name keys to colors.
    pub color: HashMap<String, Color>,

    /// Maps name keys to container themes.
    pub container: HashMap<String, Container>,

    /// Maps name keys to panegrid themes.
    pub panegrid: HashMap<String, PaneGrid>,

    /// Maps name keys to picklist themes.
    pub picklist: HashMap<String, Picklist>,

    /// Maps name keys to progress bar themes.
    pub progressbar: HashMap<String, ProgressBar>,

    /// Maps name keys to scrollable themes.
    pub scrollable: HashMap<String, Scrollable>,

    /// Maps name keys to text input themes.
    pub textinput: HashMap<String, TextInput>,

    /// Maps name keys to tooltip themes.
    pub tooltip: HashMap<String, Tooltip>,
}

impl iced::application::StyleSheet for Theme {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> iced::application::Appearance {
        self.application.appearance(style)
    }
}

impl Into<iced::Theme> for Theme {
    fn into(self) -> iced::Theme {
        iced::Theme::custom(iced::theme::Palette {
            background: self.application.background_color.into(),
            text: self.application.text_color.into(),
            ..iced::theme::Palette::LIGHT
        })
    }
}

impl Theme {
    /// Creates an empty theme.
    pub fn new() -> Self {
        Theme {
            name: String::new(),
            description: String::new(),

            application: Application::default(),

            border: HashMap::new(),
            button: HashMap::new(),

            color: HashMap::new(),
            container: HashMap::new(),

            panegrid: HashMap::new(),
            picklist: HashMap::new(),
            progressbar: HashMap::new(),

            scrollable: HashMap::new(),

            textinput: HashMap::new(),
            tooltip: HashMap::new(),
        }
    }

    /// Attempts to create a theme from its &serialized version.
    pub fn parse(theme: &serial::Theme) -> Result<Self, ()> {
        let mut new_theme = Self {
            name: theme.name.clone(),
            description: theme.description.clone(),
            ..Default::default()
        };

        // De&serialize all the colors.
        new_theme.color = theme
            .color
            .iter()
            .map(|(n, c)| (n.clone(), c.clone()))
            .collect();

        // De&serialize application
        new_theme.application = Application::create(&theme.application, &new_theme)?;

        // De&serialize the borders, as they only depend on colors.
        for (name, serial) in &theme.border {
            let v = Border::create(serial, &new_theme)?;
            new_theme.border.insert(name.clone(), v);
        }

        // De&serialize the progress bars, as they only depend on colors.
        for (name, serial) in &theme.progressbar {
            let v = ProgressBar::create(serial, &new_theme)?;
            new_theme.progressbar.insert(name.clone(), v);
        }

        // De&serialize the containers, as they only depend on colors and borders.
        for (name, serial) in &theme.container {
            let c = Container::create(serial, &new_theme)?;
            new_theme.container.insert(name.clone(), c);
        }

        // De&serialize the tooltips, as they only depend on colors and borders.
        for (name, serial) in &theme.tooltip {
            let c = Tooltip::create(serial, &new_theme)?;
            new_theme.tooltip.insert(name.clone(), c);
        }

        // De&serialize the composable.
        // Allow a maximum depth of 10.
        // TODO : Instead of maximum depth check for no changes in the set of buttons for a lock.
        for _ in 0..10 {
            // De&serialize the buttons.
            for (name, serial) in &theme.button {
                let b = Button::create(serial, &new_theme)?;
                new_theme.button.insert(name.clone(), b);
            }

            // De&serialize the picklists.
            for (name, serial) in &theme.panegrid {
                let p = PaneGrid::create(serial, &new_theme)?;
                new_theme.panegrid.insert(name.clone(), p);
            }

            // De&serialize the picklists.
            for (name, serial) in &theme.picklist {
                let p = Picklist::create(serial, &new_theme)?;
                new_theme.picklist.insert(name.clone(), p);
            }

            // De&serialize the scrollables.
            for (name, serial) in &theme.scrollable {
                let s = Scrollable::create(serial, &new_theme)?;
                new_theme.scrollable.insert(name.clone(), s);
            }

            // De&serialize the text inputs.
            for (name, serial) in &theme.textinput {
                let t = TextInput::create(serial, &new_theme)?;
                new_theme.textinput.insert(name.clone(), t);
            }
        }

        Ok(new_theme)
    }

    pub fn get_button(&self, k: &String) -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(self.button.get(k).unwrap().clone()))
    }

    pub fn get_container(&self, k: &String) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(self.container.get(k).unwrap().clone()))
    }

    pub fn get_panegrid(&self, k: &String) -> iced::theme::PaneGrid {
        iced::theme::PaneGrid::Custom(Box::new(self.panegrid.get(k).unwrap().clone()))
    }

    pub fn get_progressbar(&self, k: &String) -> iced::theme::ProgressBar {
        iced::theme::ProgressBar::Custom(Box::new(self.progressbar.get(k).unwrap().clone()))
    }

    pub fn get_scrollable(&self, k: &String) -> iced::theme::Scrollable {
        iced::theme::Scrollable::Custom(Box::new(self.scrollable.get(k).unwrap().clone()))
    }

    pub fn get_textinput(&self, k: &String) -> iced::theme::TextInput{
        iced::theme::TextInput::Custom(Box::new(self.textinput.get(k).unwrap().clone()))
    }
}
