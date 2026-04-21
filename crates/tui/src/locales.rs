pub enum HelpSections {
    GettingStarted,
    Commands,
    Navigation,
}

#[derive(Debug, Default)]
pub struct HelpMenu {
    sections: Vec<HelpSection>,
}

impl HelpMenu {
    pub fn add(mut self, section: HelpSections) -> Self {
        match section {
            HelpSections::GettingStarted => self.sections.push(HelpSection {
                title: " Getting Started ",
                text: GETTING_STARTED,
            }),
            HelpSections::Commands => self.sections.push(HelpSection {
                title: " Commands ",
                text: COMMANDS,
            }),
            HelpSections::Navigation => self.sections.push(HelpSection {
                title: " Navigation ",
                text: NAVIGATION,
            }),
        }
        self
    }

    pub fn build(self) -> Vec<HelpSection> {
        self.sections
    }
}

#[derive(Debug, Default, Clone)]
pub struct HelpSection {
    pub title: &'static str,
    pub text: &'static str,
}

impl HelpSection {}

const GETTING_STARTED: &str = r#"
    - ':'   -- enter command mode

    - '/'   -- search

    - 'esc' -- clear all 

    - ':man' -- show commands
    "#;

const COMMANDS: &str = r#"
    - ':help: | :h'              -- show help

    - ':quit | :q'               -- quit

    - ':write | :w <path>'       -- save to file

    - ':search | :s <pattern>'   -- search log message for <pattern>

    - ':filter | :f <log level>' -- filter for <log level> (info/log/warn/error)

    "#;

const NAVIGATION: &str = r#"
    - 'j'        -- scroll down

    - 'k'        -- scroll up

    - 'gg'       -- scroll to top

    - 'G'        -- scroll to bottom

    - 'ctrl+u'   -- scroll up half a page

    - 'ctrl+d'   -- scroll down half a page

    "#;
