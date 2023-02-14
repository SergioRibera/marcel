//! `Theme` represents a &serializable collection of a theme.

pub(crate) mod serial;

use crate::*;

use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
pub struct Theme<'a> {
    /// Name of this theme.
    /// This can be used to index a set of themes inside a `Hashmap`.
    pub name: &'a str,

    /// Brief description of this theme.
    /// Used mainly as a helper in the &serialized files.
    pub description: &'a str,

    /// General Application Theme
    pub application: Application,

    /// Maps name keys to border themes.
    pub border: HashMap<&'a str, Border>,

    // Maps name keys to button themes.
    pub button: HashMap<&'a str, Button>,

    /// Maps name keys to colors.
    pub color: HashMap<&'a str, Color>,

    /// Maps name keys to container themes.
    pub container: HashMap<&'a str, Container>,

    /// Maps name keys to panegrid themes.
    pub panegrid: HashMap<&'a str, PaneGrid>,

    /// Maps name keys to picklist themes.
    pub picklist: HashMap<&'a str, Picklist>,

    /// Maps name keys to progress bar themes.
    pub progressbar: HashMap<&'a str, ProgressBar>,

    /// Maps name keys to scrollable themes.
    pub scrollable: HashMap<&'a str, Scrollable>,

    /// Maps name keys to text input themes.
    pub textinput: HashMap<&'a str, TextInput>,

    /// Maps name keys to tooltip themes.
    pub tooltip: HashMap<&'a str, Tooltip>,
}

impl<'a> Theme<'a> {
    /// Creates an empty theme.
    pub fn new() -> Self {
        Theme {
            name: "",
            description: "",

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
    pub fn parse(&mut self, theme: &'a serial::Theme) -> Result<usize, ()> {
        // Get the name and description.
        self.name = theme.name.as_str();
        self.description = theme.description.as_str();

        // Number of failed elements.
        let mut failed = 0;

        // De&serialize all the colors.
        self.color = theme
            .color
            .iter()
            .map(|(n, c)| (n.as_str(), c.clone()))
            .collect();

        // De&serialize application
        self.application = Application::create(&theme.application, &self)?;

        // De&serialize the borders, as they only depend on colors.
        for (name, serial) in &theme.border {
            match Border::create(serial, &self) {
                Ok(b) => {
                    self.border.insert(name.as_str(), b);
                }
                Err(_) => failed += 1,
            }
        }

        // De&serialize the progress bars, as they only depend on colors.
        for (name, serial) in &theme.progressbar {
            match ProgressBar::create(serial, &self) {
                Ok(p) => {
                    self.progressbar.insert(name.as_str(), p);
                }
                Err(_) => failed += 1,
            }
        }

        // De&serialize the containers, as they only depend on colors and borders.
        for (name, serial) in &theme.container {
            match Container::create(serial, &self) {
                Ok(c) => {
                    self.container.insert(name.as_str(), c);
                }
                Err(_) => failed += 1,
            }
        }

        // De&serialize the tooltips, as they only depend on colors and borders.
        for (name, serial) in &theme.tooltip {
            match Tooltip::create(serial, &self) {
                Ok(c) => {
                    self.tooltip.insert(name.as_str(), c);
                }
                Err(_) => failed += 1,
            }
        }

        // De&serialize the composable.
        // Allow a maximum depth of 10.
        // TODO : Instead of maximum depth check for no changes in the set of buttons for a lock.
        for _ in 0..10 {
            // De&serialize the buttons.
            for (name, serial) in &theme.button {
                match Button::create(serial, &self) {
                    Ok(b) => {
                        self.button.insert(name.as_str(), b);
                    }
                    Err(_) => failed += 1,
                }
            }

            // De&serialize the picklists.
            for (name, serial) in &theme.panegrid {
                match PaneGrid::create(serial, &self) {
                    Ok(p) => {
                        self.panegrid.insert(name.as_str(), p);
                    }
                    Err(_) => failed += 1,
                }
            }

            // De&serialize the picklists.
            for (name, serial) in &theme.picklist {
                match Picklist::create(serial, &self) {
                    Ok(p) => {
                        self.picklist.insert(name.as_str(), p);
                    }
                    Err(_) => failed += 1,
                }
            }

            // De&serialize the scrollables.
            for (name, serial) in &theme.scrollable {
                match Scrollable::create(serial, &self) {
                    Ok(s) => {
                        self.scrollable.insert(name.as_str(), s);
                    }
                    Err(_) => failed += 1,
                }
            }

            // De&serialize the text inputs.
            for (name, serial) in &theme.textinput {
                match TextInput::create(serial, &self) {
                    Ok(t) => {
                        self.textinput.insert(name.as_str(), t);
                    }
                    Err(_) => failed += 1,
                }
            }
        }

        Ok(failed)
    }
}

impl<'a> core::fmt::Display for Theme<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // Create the buffer string.
        let mut string = String::new();

        // Name and description.
        string += &format!("Theme \"{}\"\n", self.name);
        string += &format!("  {}\n", self.description);

        // Display the colors.
        string += "|- Colors\n";

        for (name, color) in &self.color {
            string += &format!("| |- \"{}\": {}\n", name, color);
        }

        // Display the borders.
        string += "|- Borders\n";

        for (name, border) in &self.border {
            string += &format!("| |- \"{}\"\n", name);
            string += &format!("| | |- Color: {}\n", border.color);
            string += &format!("| | |- Radius: {:.3}\n", border.radius);
            string += &format!("| | |- Width:  {:.3}\n", border.width);
        }

        // Display the buttons.
        string += "|- Buttons\n";

        for (name, button) in &self.button {
            const STATE: [&'static str; 4] = ["Active  ", "Hovered ", "Pressed ", "Disabled"];

            string += &format!("| |- \"{}\"\n", name);

            for state in 0..4 {
                string += &format!("| | |- {}\n", STATE[state]);
                string += &format!("| | | |- Background: {}\n", button.state[state].background);
                string += &format!("| | | |- Text color: {}\n", button.state[state].text);
                string += "| | | |- Border:\n";
                string += &format!("| | |   |- Color: {}\n", button.state[state].border.color);
                string += &format!(
                    "| | |   |- Radius: {:.3}\n",
                    button.state[state].border.radius
                );
                string += &format!(
                    "| | |   |- Width:  {:.3}\n",
                    button.state[state].border.width
                );
            }
        }

        // Display the borders.
        string += "|- Containers\n";

        for (name, container) in &self.container {
            string += &format!("| |- \"{}\"\n", name);
            string += &format!("| | |- Color: {}\n", container.color);
            string += "| | |- Border\n";
            string += &format!("| |   |- Color: {}\n", container.border.color);
            string += &format!("| |   |- Radius: {:.3}\n", container.border.radius);
            string += &format!("| |   |- Width:  {:.3}\n", container.border.width);
        }

        // Display the pick list.
        string += "|- Pane Grids\n";

        for (name, panegrid) in &self.panegrid {
            const STATE: [&'static str; 2] = ["Picked  ", "Hovered "];

            string += &format!("| |- \"{}\"\n", name);

            for state in 0..2 {
                string += &format!("| | |- {}\n", STATE[state]);
                string += &format!("| | | |- Line color: {}\n", panegrid.state[state].color);
                string += &format!("| | | |- Line width: {}\n", panegrid.state[state].width);
            }
        }

        // Display the pick list.
        string += "|- Picklists (Dropdowns)\n";

        for (name, picklist) in &self.picklist {
            const STATE: [&'static str; 2] = ["Active  ", "Hovered "];

            string += &format!("| |- \"{}\"\n", name);

            for state in 0..2 {
                string += &format!("| | |- {}\n", STATE[state]);
                string += &format!(
                    "| | | |- Background:        {}\n",
                    picklist.state[state].background
                );
                string += &format!(
                    "| | | |- Text color:        {}\n",
                    picklist.state[state].text
                );
                string += &format!(
                    "| | | |- Placeholder color: {}\n",
                    picklist.state[state].placeholder
                );
                string += &format!(
                    "| | | |- Handle color     : {}\n",
                    picklist.state[state].handle
                );
                string += "| | | |- Border:\n";
                string += &format!("| | |   |- Color: {}\n", picklist.state[state].border.color);
                string += &format!(
                    "| | |   |- Radius: {:.3}\n",
                    picklist.state[state].border.radius
                );
                string += &format!(
                    "| | |   |- Width:  {:.3}\n",
                    picklist.state[state].border.width
                );
            }

            string += "| | |- Menu\n";

            string += &format!(
                "| | | |- Background:        {}\n",
                picklist.menu.background[0]
            );
            string += &format!("| | | |- Text color:        {}\n", picklist.menu.text[0]);
            string += &format!(
                "| | | |- Selected background:        {}\n",
                picklist.menu.background[1]
            );
            string += &format!(
                "| | | |- Selected text color:        {}\n",
                picklist.menu.text[1]
            );

            string += "| | | |- Border:\n";
            string += &format!("| | |   |- Color: {}\n", picklist.menu.border.color);
            string += &format!("| | |   |- Radius: {:.3}\n", picklist.menu.border.radius);
            string += &format!("| | |   |- Width:  {:.3}\n", picklist.menu.border.width);
        }

        // Display the progress bar.
        string += "|- Progress bars\n";

        for (name, progressbar) in &self.progressbar {
            string += &format!("| |- \"{}\"\n", name);
            string += &format!("| | |- Background: {}\n", progressbar.background);
            string += &format!("| | |- Bar:        {}\n", progressbar.bar);
            string += &format!("| | |- Radius: {:.3}\n", progressbar.radius);
        }

        // Display the scrollbars.
        string += "|- Scrollbars\n";

        for (name, scrollable) in &self.scrollable {
            const STATE: [&'static str; 3] = ["Active  ", "Hovered ", "Dragging"];

            string += &format!("| |- \"{}\"\n", name);

            for state in 0..3 {
                string += &format!("| | |- {}\n", STATE[state]);
                string += &format!(
                    "| | | |- Scrollbar color: {}\n",
                    scrollable.state[state].color
                );
                string += "| | | |- Scrollbar border:\n";
                string += &format!(
                    "| | |   |- Color: {}\n",
                    scrollable.state[state].border.color
                );
                string += &format!(
                    "| | |   |- Radius: {:.3}\n",
                    scrollable.state[state].border.radius
                );
                string += &format!(
                    "| | |   |- Width:  {:.3}\n",
                    scrollable.state[state].border.width
                );

                string += &format!(
                    "| | | |- Scroller color: {}\n",
                    scrollable.state[state].scolor
                );
                string += "| | | |- Scroller border:\n";
                string += &format!(
                    "| | |   |- Color: {}\n",
                    scrollable.state[state].sborder.color
                );
                string += &format!(
                    "| | |   |- Radius: {:.3}\n",
                    scrollable.state[state].sborder.radius
                );
                string += &format!(
                    "| | |   |- Width:  {:.3}\n",
                    scrollable.state[state].sborder.width
                );
            }
        }

        // Display the text input.
        string += "|- Scrollbars\n";

        for (name, textinput) in &self.textinput {
            const STATE: [&'static str; 3] = ["Active  ", "Hovered ", "Focused"];

            string += &format!("| |- \"{}\"\n", name);

            for state in 0..3 {
                string += &format!("| | |- {}\n", STATE[state]);
                string += &format!(
                    "| | | |- Background: {}\n",
                    textinput.state[state].background
                );
                string += "| | | |- Border:\n";
                string += &format!(
                    "| | |   |- Color: {}\n",
                    textinput.state[state].border.color
                );
                string += &format!(
                    "| | |   |- Radius: {:.3}\n",
                    textinput.state[state].border.radius
                );
                string += &format!(
                    "| | |   |- Width:  {:.3}\n",
                    textinput.state[state].border.width
                );
            }

            string += &format!("| | |- Placeholder color: {}\n", textinput.colors[0]);
            string += &format!("| | |- Value color:       {}\n", textinput.colors[1]);
            string += &format!("| | |- Selection color:   {}\n", textinput.colors[2]);
        }

        // Display the tooltip.
        string += "|- Tooltips\n";

        for (name, tooltip) in &self.tooltip {
            string += &format!("| |- \"{}\"\n", name);
            string += &format!("| | |- Background: {}\n", tooltip.background);
            string += &format!("| | |- Text color: {}\n", tooltip.text);
            string += "| | |- Border:\n";
            string += &format!("| |   |- Color: {}\n", tooltip.border.color);
            string += &format!("| |   |- Radius: {:.3}\n", tooltip.border.radius);
            string += &format!("| |   |- Width:  {:.3}\n", tooltip.border.width);
        }

        f.write_str(&string)
    }
}
