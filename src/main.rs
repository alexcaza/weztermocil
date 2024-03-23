use std::{env, fs, path::Path};

use clap::Parser;
use serde::{Deserialize, Serialize};
use shellexpand::tilde;
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
    #[arg(long, action)]
    here: Option<bool>,
    #[arg(long)]
    edit: Option<String>,
    #[arg(long)]
    show: Option<String>,
    #[arg(long, action)]
    list: bool,
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

fn expand_path(raw_path: String) -> String {
    // TODO: Expand the raw path to an absolute path from `~` or relative.
    return String::from("TEST");
}

fn list_layouts() {
    let dir = tilde("~/.weztermocil").to_string();
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        println!("{}", path.unwrap().file_name().into_string().unwrap());
    }
}

fn show_layout_contents(path: String) {
    // TODO: Implement showing layout
}

fn edit_layout(path: String) {
    // TODO: Implement opening file with `$EDITOR`;
}

fn use_layout(path: String) -> YAMLConfig {
    let layout_yml = fs::read_to_string(path).expect("File should exist!");
    serde_yaml::from_str(&layout_yml).unwrap()
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

    if args.list {
        list_layouts();
        return;
    }

    let mut layout_path: String = String::from("");

    if let Some(global_layout) = args.global_layout {
        let home = env::var("HOME").unwrap();
        let path = format!("{}/.weztermocil/{}", home, global_layout);
        layout_path = path;
    }

    if let Some(layout) = args.layout {
        let path = shellexpand::env(layout.as_str()).unwrap().to_string();
        layout_path = path.clone();
    }

    if layout_path.is_empty() {
        layout_path = String::from("./weztermocil.yml");
    }

    let yaml_config: YAMLConfig = use_layout(layout_path);

    let (focus_tuple, all_panes) = build_panes(yaml_config);

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

// TODO: Conver return types into newtypes (FocusTuple, WindowPaneTuple)
fn build_panes(yaml_config: YAMLConfig) -> (Vec<usize>, Vec<Vec<Pane>>) {
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
    (focus_tuple, all_panes)
}
