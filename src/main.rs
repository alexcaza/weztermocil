use std::{env, fs, path::Path};

use clap::Parser;
use serde::{Deserialize, Serialize};
use weztermocil::{
    layout::{Layout, TotalPanes},
    wezterm::pane::Pane,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PaneConfigOptions {
    commands: Option<Vec<String>>,
    #[serde(default)]
    focus: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum PaneConfig {
    Commands(Vec<String>),
    Hash(Vec<PaneConfigOptions>),
}

#[derive(Serialize, Deserialize, Debug)]
struct WindowConfig {
    name: Option<String>, // unsupported currently
    root: Option<String>,
    layout: Option<String>,
    panes: Option<PaneConfig>,
    command: Option<String>,
    commands: Option<Vec<String>>,
    #[serde(default)]
    focus: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct YAMLConfig {
    name: Option<String>,
    windows: Option<Vec<WindowConfig>>,
    pre: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    global_layout: Option<String>,
    #[arg(long)]
    layout: Option<String>,
    #[arg(long)]
    here: Option<bool>,
    #[arg(long)]
    edit: Option<bool>,
    #[arg(long)]
    show: Option<bool>,
}

fn layout_string_to_enum(name: String) -> Layout {
    match name.as_str() {
        "tiled" => Layout::Tiled,
        "even-horizontal" => Layout::EvenHorizontal,
        "main-vertical" => Layout::MainVertical,
        "main-vertical-flipped" => Layout::MainVerticalFlipped,
        "even-vertical" => Layout::EvenVertical,
        "3_columns" => Layout::ThreeColumns,
        "double-main-horizontal" => Layout::DoubleMainHorizontal,
        "double-main-vertical" => Layout::DoubleMainVertical,
        &_ => Layout::Tiled,
    }
}

fn main() {
    // Job of this file:
    // 1. Read the yaml file given
    // 2. Kick off the logic based on the config in the file
    // 3. Create the panes and split them appropriately
    //
    // To get current pane id, check WEZTERM_PANE env var
    // it's set by wezterm as the program env var
    let args = Args::parse();
    // TODO:: Clean this up. There's definitely a better way to handle the layout
    // checking
    let mut layout_yml = String::from("");

    if let Some(global_layout) = args.global_layout {
        println!("{}", global_layout);
        let home = env::var("HOME").unwrap();
        let _path = format!("{}/.weztermocil/{}", home, global_layout);
        let path = Path::new(&_path);
        match fs::read_to_string(path) {
            // TODO: Better error handling/messaging
            Err(err) => println!("Failed with {}", err),
            Ok(res) => {
                layout_yml = res;
            }
        };
    }

    if let Some(layout) = args.layout {
        println!("{}", layout);
        layout_yml = fs::read_to_string(layout).expect("File should exist!");
    }

    if layout_yml.is_empty() {
        layout_yml =
            fs::read_to_string("./weztermocil.yml").expect("No local weztermocil.yml file to use.");
    }

    let yaml_config: YAMLConfig = serde_yaml::from_str(&layout_yml).unwrap();

    // println!("yaml_config: {:?}", yaml_config);

    let mut focus_tuple = vec![0, 0];
    let mut all_panes = vec![];
    let mut focus_list = vec![];

    if let Some(windows) = yaml_config.windows {
        for (window_index, window) in windows.iter().enumerate() {
            focus_list.push(vec![]);

            if window.focus {
                focus_tuple = vec![window_index, 0];
            }

            let layout =
                layout_string_to_enum(window.layout.clone().unwrap_or(String::from("tiled")));
            let panes = window.panes.clone().unwrap_or(PaneConfig::Commands(vec![]));
            let main_pane = Pane::new(&window.root);

            if let Some(tab_name) = window.name.clone() {
                main_pane
                    .set_tab_title(tab_name)
                    .expect("Window name should've been set. Something bad happened here.");
            }

            let mut commands = vec![];
            match panes {
                PaneConfig::Commands(_commands) => {
                    for c in _commands {
                        commands.push(vec![c]);
                        // If it's just a list of commands, you can't focus a pane
                        focus_list[window_index].push(false);
                    }
                }
                PaneConfig::Hash(config) => {
                    for c in config {
                        if let Some(cmds) = c.commands {
                            commands.push(cmds);
                        }

                        focus_list[window_index].push(c.focus);
                    }
                }
            };

            let total_panes = TotalPanes(commands.len());

            all_panes.push(layout.create(total_panes, main_pane).unwrap_or(vec![]));

            // TODO: Handle pane/window focus here
            // Need to retain cmd/cmd_group + pane id for focus
            for (i, pane) in all_panes[window_index].iter().enumerate() {
                let command_group = commands.get(i).expect("Pane option should exist!");
                let should_focus = focus_list[window_index].get(i).expect("Focus should exist");

                if *should_focus == true {
                    focus_tuple = vec![window_index, i];
                }

                for cmd in command_group {
                    pane.run_command(cmd.clone());
                }
            }
        }
    }

    let focus_pane = all_panes
        .get(focus_tuple[0])
        .expect("Window to focus should exist!")
        .get(focus_tuple[1])
        .expect("Pane to focus should exist!");

    match focus_pane.focus() {
        Ok(res) => res,
        Err(error) => println!("{:?}", error),
    }
}
