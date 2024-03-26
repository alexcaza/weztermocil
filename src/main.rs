use std::{
    env, fs,
    ops::Deref,
    process::{self, Command},
};

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
    name: Option<String>,
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

const DIRS: &[&str] = &[".weztermocil", ".teamocil", ".itermocil"];

type PaneIndex = usize;
type WindowIndex = usize;
struct FocusTuple(WindowIndex, PaneIndex);
struct WindowPanes(Vec<Vec<Pane>>);

fn layout_string_to_enum(name: &str) -> Layout {
    match name {
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

fn get_global_config_path() -> Result<String, String> {
    let mut path = String::from("");
    for dir in DIRS {
        let _dir = format!("~/{}", dir);
        let expanded = tilde(&_dir);
        let paths = fs::read_dir(expanded.deref());
        match paths {
            Ok(_) => {
                path = expanded.to_string();
            }
            Err(_) => continue,
        };
    }

    if path == "" {
        return Err(String::from("Couldn't find a .weztermocil, .teamocil or .itermocil folder in the home directory (~)\nPlease make sure one of them exists before continuing"));
    }

    Ok(path)
}

fn get_local_config_path() -> Result<String, String> {
    let mut path = String::from("");
    for dir in DIRS {
        let paths = fs::read_dir(dir);
        match paths {
            Ok(_) => {
                path = dir.to_string();
            }
            Err(_) => continue,
        };
    }

    if path == "" {
        return Err(String::from("Couldn't find a .weztermocil, .teamocil or .itermocil folder in the current directory\nPlease make sure one of them exists before continuing"));
    }

    Ok(path)
}

fn list_layouts() {
    let path = get_global_config_path();
    match path {
        Ok(p) => {
            // We've already validated that the path exists, so we can unwrap here.
            let entries = fs::read_dir(p).unwrap();
            for entry in entries {
                println!("{}", entry.unwrap().file_name().into_string().unwrap());
            }
        }
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}

fn get_path_for_layout_file(layout_name: &str) -> Result<String, String> {
    let current_dir = std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let current_dir_fp = format!("{}/{}", current_dir, layout_name);
    let in_current_dir = fs::File::open(&current_dir_fp);

    if let Ok(_) = in_current_dir {
        return Ok(current_dir_fp);
    }

    let local_layout_dir_path = get_local_config_path();
    match local_layout_dir_path {
        Ok(p) => {
            let local_layout_dir_fp = format!("{}/{}/{}", current_dir, p, layout_name);
            let in_local_layout_dir = fs::File::open(&local_layout_dir_fp);

            if let Ok(_) = in_local_layout_dir {
                return Ok(local_layout_dir_fp);
            }
        }
        Err(_) => {}
    }

    let global_path = get_global_config_path();
    match global_path {
        Ok(p) => {
            let global_layout_fp = format!("{}/{}", p, layout_name);
            let in_global_layout = fs::File::open(&global_layout_fp);

            if let Ok(_) = in_global_layout {
                return Ok(global_layout_fp);
            }
        }
        Err(_) => {}
    }

    return Err(String::from("Couldn't find layout"));
}

fn show_layout_contents(path: String) {
    let contents = fs::read_to_string(path).unwrap();
    println!("{}", contents);
}

fn edit_layout(path: String) -> () {
    let editor = env::var("EDITOR").unwrap();
    Command::new(editor)
        .arg(path.as_str())
        .status()
        .expect("Editor should exist");
}

fn use_layout(path: &str) -> YAMLConfig {
    match fs::read_to_string(path) {
        Ok(file) => serde_yaml::from_str(&file).unwrap(),
        Err(_) => {
            println!("{} not found!", path);
            process::exit(1);
        }
    }
}

fn main() {
    let args = Args::parse();

    if args.list {
        list_layouts();
        return;
    }

    if let Some(path) = args.show {
        let path = get_path_for_layout_file(&path);
        match path {
            Ok(p) => {
                println!("Found layout at path: {}", p);
                show_layout_contents(p);
            }
            Err(error) => {
                println!("{}", error);
                process::exit(1);
            }
        }
        return;
    }

    if let Some(path) = args.edit {
        let layout = qualify_layout_file(&path);
        let path = get_path_for_layout_file(&layout);
        match path {
            Ok(p) => {
                println!("Found layout at path: {}", p);
                println!("Opening with '{}'...", env::var("EDITOR").unwrap());
                edit_layout(p);
            }
            Err(error) => {
                println!("{}", error);
                process::exit(1);
            }
        }
        return;
    }

    let mut layout_path: String = String::from("");

    if let Some(global_layout) = args.global_layout {
        let layout = qualify_layout_file(&global_layout);
        let path = get_path_for_layout_file(&layout);
        match path {
            Ok(p) => {
                layout_path = p;
            }
            Err(error) => {
                println!("{}", error);
                process::exit(1);
            }
        }
    }

    if let Some(path) = args.layout {
        let layout = qualify_layout_file(&path);
        let f = match fs::canonicalize(&layout) {
            Ok(path) => path.into_os_string().into_string(),
            Err(_) => {
                println!("Couldn't find file at path: {}. Does it exist?", layout);
                process::exit(1);
            }
        };

        let p = match f {
            Ok(path) => path,
            Err(_) => {
                println!("Couldn't find file at path: {}. Does it exist?", layout);
                process::exit(1);
            }
        };

        layout_path = p;
    }

    if layout_path.is_empty() {
        layout_path = String::from("./weztermocil.yml");
    }

    let yaml_config: YAMLConfig = use_layout(&layout_path);

    let (focus_tuple, window_panes) = build_panes(yaml_config);

    let focus_pane = window_panes
        .0
        .get(focus_tuple.0)
        .expect("Window to focus should exist!\nIs the layout file empty?")
        .get(focus_tuple.1)
        .expect("Pane to focus should exist!\nIs the layout file malformed?");

    match focus_pane.focus() {
        Ok(res) => res,
        Err(error) => println!("{:?}", error),
    }
}

fn qualify_layout_file(path: &str) -> String {
    if path.contains(".yml") == false {
        format!("{}.yml", path)
    } else {
        String::from(path)
    }
}

fn build_panes(yaml_config: YAMLConfig) -> (FocusTuple, WindowPanes) {
    let mut focus_tuple = FocusTuple(0, 0);
    let mut all_panes = vec![];
    let mut focus_list = vec![];

    if let Some(windows) = yaml_config.windows {
        for (window_index, window) in windows.iter().enumerate() {
            focus_list.push(vec![]);

            if window.focus {
                focus_tuple = FocusTuple(window_index, 0);
            }

            let layout =
                layout_string_to_enum(&window.layout.clone().unwrap_or(String::from("tiled")));
            let panes = window.panes.clone().unwrap_or(PaneConfig::Commands(vec![]));
            let main_pane = match window.root.clone() {
                Some(cwd) => Pane::new(Some(&cwd)),
                None => Pane::new(None),
            };

            if let Some(tab_name) = window.name.clone() {
                main_pane
                    .set_tab_title(&tab_name)
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
                    focus_tuple = FocusTuple(window_index, i);
                }

                for cmd in command_group {
                    pane.run_command(cmd);
                }
            }
        }
    }
    (focus_tuple, WindowPanes(all_panes))
}
