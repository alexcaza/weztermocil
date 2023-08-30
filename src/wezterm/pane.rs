use super::{cli::CLI, tab::Tab};

pub enum SplitDirection {
    Right,
    Left,
    Bottom,
    Top,
}

#[derive(Debug)]
pub struct Pane {
    pub number: String,
    pub tab: String,
}

impl Pane {
    pub fn new(tab: Tab, direction: Option<SplitDirection>) -> Pane {
        let id = match CLI::split_pane(tab.id.clone(), &direction) {
            Ok(id) => id,
            Err(e) => panic!("Failed to split pane: {}", e),
        };

        Pane {
            number: id,
            tab: tab.id,
        }
    }
}
