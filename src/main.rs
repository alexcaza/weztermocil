use std::fs;

use clap::Parser;
use serde::{Deserialize, Serialize};
use weztermocil::{
    layout::{Layout, TotalPanes},
    wezterm::pane::{Pane, SplitDirection},
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
    Hash(PaneConfigOptions),
    // commands: Vec<String>,
    // focus: bool,
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
        layout_yml = fs::read_to_string(format!("~/.weztermocil/{}", global_layout))
            .expect("File should exist!");
    }

    if let Some(layout) = args.layout {
        println!("{}", layout);
        layout_yml = fs::read_to_string(layout).expect("File should exist!");
    }

    if layout_yml.is_empty() {
        layout_yml = fs::read_to_string("./weztermocil.yml").expect("File should exist!");
    }

    let yaml_config: YAMLConfig = serde_yaml::from_str(&layout_yml).unwrap();

    println!("yaml_config: {:?}", yaml_config);

    if let Some(windows) = yaml_config.windows {
        for window in windows {
            // TODO: Extract to fn
            let layout = layout_string_to_enum(window.layout.unwrap_or(String::from("tiled")));
            println!("{:?}", window.panes);
            let panes = window.panes.unwrap_or(PaneConfig::Commands(vec![]));
            // TODO: handle --here case
            // Though, I don't know how it --here works with multiple windows...
            let main_pane = Pane::new(None);
            // let total_panes = TotalPanes(panes.len());
            let commands = match panes {
                PaneConfig::Commands(commands) => commands,
                // TODO: Fix Hash enum member.
                PaneConfig::Hash(panes) => panes.commands.unwrap_or(vec![]),
            };

            let total_panes = TotalPanes(commands.len());

            // TODO: Sometimes same pane is being returned from create function.
            // So we get two commands running in the same pane unintentionally.
            let all_panes = layout.create(total_panes, main_pane).unwrap_or(vec![]);

            for (i, pane) in all_panes.iter().enumerate() {
                let command = commands.get(i).expect("Pane option should exist!");
                pane.run_command(command.clone());
            }
        }
    }

    // Check if weztermocil.yml file exists in current directory.
    // Use that if it does and no layout or global_layout exists.

    // let cwd_test = Some("/Users/alexcaza/Documents/programming/personal/weztermocil");
    //
    // let pane = Pane::new(cwd_test);
    // println!("{:?}", pane);

    // let new_pane = pane.split(Some(SplitDirection::Top), None);
    // println!("{:?}", new_pane);
    //
    // let child_pane = new_pane.split(Some(SplitDirection::Bottom), None);
    // println!("{:?}", child_pane);
    //
    // let _ = child_pane.set_tab_title(String::from("Test"));
    //
    // child_pane.run_command(String::from("cargo help"));

    // Layout::EvenHorizontal.create(TotalPanes(3), pane);
    // Layout::EvenVertical.create(TotalPanes(7), pane);
    // Layout::MainVertical.create(TotalPanes(5), pane);
    // Layout::MainVerticalFlipped.create(TotalPanes(3), pane);
    // Layout::Tiled.create(TotalPanes(5), pane);
    // Layout::ThreeColumns.create(TotalPanes(3), pane);
    // Layout::DoubleMainVertical.create(TotalPanes(7), pane);
    // Layout::DoubleMainHorizontal.create(TotalPanes(8), pane);
}
