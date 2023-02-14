//! Serial version of the theme.

use super::Theme as NormalTheme;
use crate::{serial::*, Color};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Theme {
    /// Name of this theme.
    /// This can be used to index a set of themes inside a `Hashmap`.
    pub name: String,

    /// Brief description of this theme.
    /// Used mainly as a helper in the serialized files.
    pub description: String,

    /// General Application Theme
    pub application: Application,

    /// Maps name keys to border themes.
    pub border: HashMap<String, Border>,

    // Maps name keys to button themes.
    pub button: HashMap<String, Button>,

    /// Maps name keys to colors.
    pub color: HashMap<String, Color>,

    /// Maps name keys to containers.
    pub container: HashMap<String, Container>,

    /// Maps name keys to pane grids.
    pub panegrid: HashMap<String, PaneGrid>,

    /// Maps name keys to picklists.
    pub picklist: HashMap<String, Picklist>,

    /// Maps name keys to progress bar.
    pub progressbar: HashMap<String, ProgressBar>,

    /// Maps name keys to scrollable.
    pub scrollable: HashMap<String, Scrollable>,

    /// Maps name keys to text input.
    pub textinput: HashMap<String, TextInput>,

    /// Maps name keys to tooltip.
    pub tooltip: HashMap<String, Tooltip>,
}

fn get_color_name(colors: &mut HashMap<String, Color>, c: &Color) -> String {
    let value = colors.iter().find(|color| color.1 == c).map(|r| r.0.to_string());
    if value.is_none() {
        let default_name = format!("color_{}", colors.len());
        colors.insert(default_name.clone(), c.clone());
        return default_name;
    };
    value.unwrap()
}

fn get_border_name(
    colors: &mut HashMap<String, Color>,
    borders: &mut HashMap<String, Border>,
    b: &crate::Border,
) -> String {
    let color_name = get_color_name(colors, &b.color);
    let value = borders
        .iter()
        .find(|border| {
            border.1.radius == b.radius
                && border.1.color == color_name.clone()
                && border.1.width == b.width
        })
        .map(|r| r.0.to_string());
    if value.is_none() {
        let default_name = format!("border_{}", borders.len());
        borders.insert(
            default_name.clone(),
            Border {
                color: color_name.clone(),
                radius: b.radius,
                width: b.width,
            },
        );
        return default_name;
    };
    value.unwrap()
}

impl From<NormalTheme> for Theme {
    fn from(theme: NormalTheme) -> Self {
        // Deserialize all the colors.
        let mut color = theme
            .color
            .iter()
            .map(|(n, c)| (n.to_string(), c.clone()))
            .collect::<HashMap<String, Color>>();
        let mut border = theme
            .border
            .iter()
            .map(|(n, b)| {
                (
                    n.to_string(),
                    Border {
                        color: get_color_name(&mut color, &b.color),
                        radius: b.radius,
                        width: b.width,
                    },
                )
            })
            .collect::<HashMap<String, Border>>();

        let button = theme
            .button
            .iter()
            .map(|(name, btn)| {
                (
                    name.to_string(),
                    Button {
                        active: ButtonComponent::Defined(ButtonState {
                            background: get_color_name(&mut color, &btn.state[0].background),
                            text: get_color_name(&mut color, &btn.state[0].text),
                            border: get_border_name(&mut color, &mut border, &btn.state[0].border),
                        }),
                        hovered: ButtonComponent::Defined(ButtonState {
                            background: get_color_name(&mut color, &btn.state[1].background),
                            text: get_color_name(&mut color, &btn.state[1].text),
                            border: get_border_name(&mut color, &mut border, &btn.state[1].border),
                        }),
                        pressed: ButtonComponent::Defined(ButtonState {
                            background: get_color_name(&mut color, &btn.state[2].background),
                            text: get_color_name(&mut color, &btn.state[2].text),
                            border: get_border_name(&mut color, &mut border, &btn.state[2].border),
                        }),
                        disabled: ButtonComponent::Defined(ButtonState {
                            background: get_color_name(&mut color, &btn.state[3].background),
                            text: get_color_name(&mut color, &btn.state[3].text),
                            border: get_border_name(&mut color, &mut border, &btn.state[3].border),
                        }),
                    },
                )
            })
            .collect::<HashMap<String, Button>>();

        let container = theme
            .container
            .iter()
            .map(|(name, c)| {
                (
                    name.to_string(),
                    Container {
                        color: get_color_name(&mut color, &c.color),
                        border: get_border_name(&mut color, &mut border, &c.border),
                    },
                )
            })
            .collect::<HashMap<String, Container>>();

        let panegrid = theme
            .panegrid
            .iter()
            .map(|(name, p)| {
                (
                    name.to_string(),
                    PaneGrid {
                        picked: PaneGridComponent::Defined(PaneGridState {
                            color: get_color_name(&mut color, &p.state[0].color),
                            width: p.state[0].width,
                        }),
                        hovered: PaneGridComponent::Defined(PaneGridState {
                            color: get_color_name(&mut color, &p.state[1].color),
                            width: p.state[1].width,
                        }),
                    },
                )
            })
            .collect::<HashMap<String, PaneGrid>>();

        let picklist = theme
            .picklist
            .iter()
            .map(|(name, p)| {
                (
                    name.to_string(),
                    Picklist {
                        active: PicklistStateComponent::Defined(PicklistState {
                            background: get_color_name(&mut color, &p.state[0].background),
                            text: get_color_name(&mut color, &p.state[0].text),
                            placeholder: get_color_name(&mut color, &p.state[0].placeholder),
                            border: get_border_name(&mut color, &mut border, &p.state[0].border),
                            handle: get_color_name(&mut color, &p.state[0].handle),
                        }),
                        hovered: PicklistStateComponent::Defined(PicklistState {
                            background: get_color_name(&mut color, &p.state[1].background),
                            text: get_color_name(&mut color, &p.state[1].text),
                            placeholder: get_color_name(&mut color, &p.state[1].placeholder),
                            border: get_border_name(&mut color, &mut border, &p.state[1].border),
                            handle: get_color_name(&mut color, &p.state[1].handle),
                        }),
                        menu: PicklistMenuComponent::Defined(PicklistMenu {
                            background: get_color_name(&mut color, &p.menu.background[0]),
                            text: get_color_name(&mut color, &p.menu.text[0]),
                            border: get_border_name(&mut color, &mut border, &p.menu.border),
                            sbackground: get_color_name(&mut color, &p.menu.background[1]),
                            stext: get_color_name(&mut color, &p.menu.text[1]),
                        }),
                    },
                )
            })
            .collect::<HashMap<String, Picklist>>();

        let progressbar = theme
            .progressbar
            .iter()
            .map(|(name, p)| {
                (
                    name.to_string(),
                    ProgressBar {
                        background: get_color_name(&mut color, &p.background),
                        bar: get_color_name(&mut color, &p.bar),
                        radius: p.radius,
                    },
                )
            })
            .collect::<HashMap<String, ProgressBar>>();

        let scrollable = theme
            .scrollable
            .iter()
            .map(|(name, s)| {
                (
                    name.to_string(),
                    Scrollable {
                        active: ScrollableComponent::Defined(ScrollableState {
                            color: get_color_name(&mut color, &s.state[0].color),
                            border: get_border_name(&mut color, &mut border, &s.state[0].border),
                            scolor: get_color_name(&mut color, &s.state[0].scolor),
                            sborder: get_border_name(&mut color, &mut border, &s.state[0].sborder),
                        }),
                        hovered: ScrollableComponent::Defined(ScrollableState {
                            color: get_color_name(&mut color, &s.state[1].color),
                            border: get_border_name(&mut color, &mut border, &s.state[1].border),
                            scolor: get_color_name(&mut color, &s.state[1].scolor),
                            sborder: get_border_name(&mut color, &mut border, &s.state[1].sborder),
                        }),
                        dragging: ScrollableComponent::Defined(ScrollableState {
                            color: get_color_name(&mut color, &s.state[2].color),
                            border: get_border_name(&mut color, &mut border, &s.state[2].border),
                            scolor: get_color_name(&mut color, &s.state[2].scolor),
                            sborder: get_border_name(&mut color, &mut border, &s.state[2].sborder),
                        }),
                    },
                )
            })
            .collect::<HashMap<String, Scrollable>>();

        let textinput = theme
            .textinput
            .iter()
            .map(|(name, t)| {
                (
                    name.to_string(),
                    TextInput {
                        active: TextInputComponent::Defined(TextInputState {
                            background: get_color_name(&mut color, &t.state[0].background),
                            border: get_border_name(&mut color, &mut border, &t.state[0].border),
                        }),
                        hovered: TextInputComponent::Defined(TextInputState {
                            background: get_color_name(&mut color, &t.state[1].background),
                            border: get_border_name(&mut color, &mut border, &t.state[1].border),
                        }),
                        focused: TextInputComponent::Defined(TextInputState {
                            background: get_color_name(&mut color, &t.state[2].background),
                            border: get_border_name(&mut color, &mut border, &t.state[2].border),
                        }),
                        placeholder: get_color_name(&mut color, &t.colors[0]),
                        value: get_color_name(&mut color, &t.colors[1]),
                        selection: get_color_name(&mut color, &t.colors[2]),
                    },
                )
            })
            .collect::<HashMap<String, TextInput>>();

        let tooltip = theme
            .tooltip
            .iter()
            .map(|(name, t)| {
                (
                    name.to_string(),
                    Tooltip {
                        background: get_color_name(&mut color, &t.background),
                        text: get_color_name(&mut color, &t.text),
                        border: get_border_name(&mut color, &mut border, &t.border),
                    },
                )
            })
            .collect::<HashMap<String, Tooltip>>();

        Self {
            name: theme.name.to_string(),
            description: theme.description.to_string(),
            color: color.clone(),
            border: border.clone(),
            application: Application {
                background_color: get_color_name(&mut color, &theme.application.background_color),
                text_color: get_color_name(&mut color, &theme.application.text_color),
            },
            button,
            container,
            panegrid,
            picklist,
            progressbar,
            scrollable,
            textinput,
            tooltip,
        }
    }
}
