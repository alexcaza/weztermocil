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
        pane_id: String,
        direction: &Option<SplitDirection>,
        percentage: &Option<String>,
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

        if let Some(p) = percentage {
            commands.push("--percent");
            commands.push(p.as_str());
        }

        println!("Commands: {:?}", commands);

        let output = cmd.args(commands).output().expect("Failed to create pane");
        // TODO: Handle non-zero case from call to `.output()` properly
        // currently it's being swallowed.
        let pane_id = from_utf8(&output.stdout).expect("Failed to convert from utf8");

        println!("Output: {:?}", output);

        Ok(String::from(strip_trailing_newline(pane_id)))
    }

    pub fn spawn(cwd: Option<&str>) -> Result<String, Box<dyn Error>> {
        let mut cmd = CLI::new();
        let mut commands = vec!["cli", "spawn"];

        match cwd {
            Some(dir) => commands.extend(["--cwd", dir].iter()),
            None => (),
        };

        let output = cmd.args(commands).output()?;
        let tab_id = from_utf8(&output.stdout)?;

        Ok(String::from(strip_trailing_newline(tab_id)))
    }

    pub fn set_tab_title(pane_id: String, title: String) -> Result<(), Box<dyn Error>> {
        let mut cmd = CLI::new();
        let commands = vec![
            "cli",
            "set-tab-title",
            title.as_str(),
            "--pane-id",
            pane_id.as_str(),
        ];

        cmd.args(commands).output()?;

        Ok(())
    }

    pub fn run_command(pane_id: String, command: String) -> () {
        let cmd = Command::new("echo")
            .arg(command.as_str())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        CLI::new()
            .args([
                "cli",
                "send-text",
                "--pane-id",
                pane_id.as_str(),
                "--no-paste",
            ])
            .stdin(Stdio::from(cmd.stdout.unwrap()))
            .spawn()
            .expect("Failed to send command");
    }
}
