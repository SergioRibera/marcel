use super::Theme;

impl core::fmt::Display for Theme {
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
