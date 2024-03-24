use std::error::Error;

use super::cli::CLI;

#[derive(Clone, Copy)]
pub enum SplitDirection {
    Right,
    Left,
    Bottom,
    Top,
}

// TODO: Refactor to use more efficient
// data types
#[derive(Debug, Clone)]
pub struct Pane {
    pub id: String,
    pub parent_id: Option<String>,
}

impl Pane {
    pub fn new(cwd: Option<&str>) -> Pane {
        let id = match CLI::spawn(cwd) {
            Ok(id) => id,
            Err(e) => panic!("Failed to split pane: {}", e),
        };

        Pane {
            id,
            parent_id: None,
        }
    }

    pub fn split(
        &self,
        direction: Option<SplitDirection>,
        percentage: &Option<String>,
        parent: Option<&Pane>,
        top_level: bool,
    ) -> Pane {
        let pane_to_split = match parent {
            Some(pane) => pane.id.clone(),
            None => self.id.clone(),
        };

        let id = match CLI::split_pane(pane_to_split.clone(), &direction, &percentage, top_level) {
            Ok(id) => id,
            Err(e) => panic!("Failed to split pane: {}", e),
        };

        Pane {
            id,
            parent_id: Some(pane_to_split.clone()),
        }
    }

    pub fn set_tab_title(&self, title: &str) -> Result<(), Box<dyn Error>> {
        CLI::set_tab_title(&self.id, title)
    }

    pub fn focus(&self) -> Result<(), Box<dyn Error>> {
        CLI::focus(&self.id)
    }

    pub fn run_command(&self, command: &str) {
        CLI::run_command(&self.id, command)
    }
}
