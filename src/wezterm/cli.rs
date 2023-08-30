use std::{error::Error, process::Command, str::from_utf8};

use crate::{strip_trailing_newline, WEZTERM_CLI};

use super::pane::SplitDirection;

pub struct CLI {}

impl CLI {
    // TODO: Add support for Windows
    fn new() -> Command {
        Command::new(WEZTERM_CLI)
    }

    pub fn split_pane(
        pane_id: String,
        direction: &Option<SplitDirection>,
    ) -> Result<String, Box<dyn Error>> {
        let mut cmd = CLI::new();
        let mut commands = vec!["cli", "split-pane", "--pane-id", pane_id.as_str()];

        if let Some(pane_dir) = &direction {
            let dir = match pane_dir {
                SplitDirection::Right => "--right",
                SplitDirection::Left => "--left",
                SplitDirection::Top => "--top",
                SplitDirection::Bottom => "--bottom",
            };
            commands.push(dir);
        };

        let output = cmd.args(commands).output()?;
        let pane_id = from_utf8(&output.stdout)?;

        Ok(String::from(strip_trailing_newline(pane_id)))
    }

    // TODO: Implement passing in CWD
    pub fn create_tab(_cwd: Option<String>) -> Result<String, Box<dyn Error>> {
        let mut cmd = CLI::new();
        let commands = vec!["cli", "spawn"];

        let output = cmd.args(commands).output()?;
        let tab_id = from_utf8(&output.stdout)?;

        Ok(String::from(strip_trailing_newline(tab_id)))
    }
}
