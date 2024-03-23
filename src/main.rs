use std::{env, fs, path::Path};

use clap::Parser;
use serde::{Deserialize, Serialize};
use weztermocil::{
    layout::{Layout, TotalPanes},
    wezterm::pane::Pane,
};

#[derive(Serialize, Deserialize, Debug)]
struct PaneConfigOptions {
    commands: Option<Vec<String>>,
    focus: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
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
    focus: Option<String>, // unsupported currently
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

    if let Some(windows) = yaml_config.windows {
        for window in windows {
            let layout = layout_string_to_enum(window.layout.unwrap_or(String::from("tiled")));
            let panes = window.panes.unwrap_or(PaneConfig::Commands(vec![]));
            let main_pane = Pane::new(&window.root);

            if let Some(tab_name) = window.name {
                main_pane
                    .set_tab_title(tab_name)
                    .expect("Window name should've been set. Something bad happened here.");
            }

            let mut commands = vec![];
            match panes {
                PaneConfig::Commands(_commands) => {
                    for c in _commands {
                        commands.push(vec![c]);
                    }
                }
                PaneConfig::Hash(config) => {
                    for c in config {
                        if let Some(cmds) = c.commands {
                            commands.push(cmds);
                        }
                    }
                }
            };

            let total_panes = TotalPanes(commands.len());

            let all_panes = layout.create(total_panes, main_pane).unwrap_or(vec![]);

            for (i, pane) in all_panes.iter().enumerate() {
                let command_group = commands.get(i).expect("Pane option should exist!");
                for cmd in command_group {
                    pane.run_command(cmd.clone());
                }
            }
        }
    }
}
