use std::{
    error::Error,
    process::{Command, Stdio},
    str::from_utf8,
};

use crate::{format::strip_trailing_newline, WEZTERM_CLI};

use super::pane::SplitDirection;

pub struct CLI {}

impl CLI {
    // TODO: Add support for Windows
    fn new() -> Command {
        Command::new(WEZTERM_CLI)
    }

    pub fn split_pane(
        pane_id: &str,
        direction: &SplitDirection,
        percentage: Option<&str>,
        top_level: bool,
    ) -> Result<String, Box<dyn Error>> {
        let mut cmd = CLI::new();
        let mut commands = vec!["cli", "split-pane", "--pane-id", pane_id];

        let dir = match direction {
            SplitDirection::Right => "--right",
            SplitDirection::Left => "--left",
            SplitDirection::Top => "--top",
            SplitDirection::Bottom => "--bottom",
        };
        commands.push(dir);

        if let Some(p) = percentage {
            commands.push("--percent");
            commands.push(p);
        }

        if top_level {
            commands.push("--top-level");
        }

        // println!("cmds: {:?}", commands);
        let output = cmd.args(commands).output().expect("Failed to create pane");
        // TODO: Handle non-zero case from call to `.output()` properly
        // currently it's being swallowed.
        let pane_id = from_utf8(&output.stdout).expect("Failed to convert from utf8");
        // println!("output: {:?}", output);

        Ok(String::from(strip_trailing_newline(pane_id)))
    }

    pub fn spawn(cwd: Option<&str>) -> Result<String, Box<dyn Error>> {
        let mut cmd = CLI::new();
        let mut commands = vec!["cli", "spawn"];

        match cwd {
            Some(dir) => {
                commands.push("--cwd");
                commands.push(dir);
            }
            None => (),
        };

        let output = cmd.args(commands).output()?;
        let tab_id = from_utf8(&output.stdout)?;

        Ok(String::from(strip_trailing_newline(tab_id)))
    }

    pub fn set_tab_title(pane_id: &str, title: &str) -> Result<(), Box<dyn Error>> {
        let mut cmd = CLI::new();
        let commands = vec!["cli", "set-tab-title", title, "--pane-id", pane_id];

        cmd.args(commands).output()?;

        Ok(())
    }

    pub fn focus(pane_id: &str) -> Result<(), Box<dyn Error>> {
        CLI::new()
            .args(["cli", "activate-pane", "--pane-id", pane_id])
            .output()?;

        Ok(())
    }

    pub fn run_command(pane_id: &str, command: &str) -> () {
        let cmd = Command::new("echo")
            .arg(command)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        CLI::new()
            .args(["cli", "send-text", "--pane-id", pane_id, "--no-paste"])
            .stdin(Stdio::from(cmd.stdout.unwrap()))
            .spawn()
            .expect("Failed to send command");
    }
}
