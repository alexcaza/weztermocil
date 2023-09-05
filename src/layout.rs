use std::fmt;

use crate::wezterm::pane::{Pane, SplitDirection};

#[derive(PartialEq)]
pub struct NumPanes(usize);

impl fmt::Display for NumPanes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub enum Layout {
    EvenHorizontal,
    EvenVertical,
    MainVertical,
    MainVerticalFlipped,
    Tiled,
    ThreeColumns,
    DoubleMainHorizontal,
    DoubleMainVertical,
}

impl Layout {
    pub fn create(&self, num_panes: NumPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
        if num_panes == NumPanes(1) {
            // Skip doing anything pane creation
            // if there's only 1 pane being passed.
            // We can just run the command in the
            // main pane
            return None;
        }

        match self {
            Layout::EvenHorizontal => even_horizontal(num_panes, parent_pane.clone()),
            Layout::EvenVertical => None,
            Layout::MainVertical => None,
            Layout::MainVerticalFlipped => None,
            Layout::Tiled => None,
            Layout::ThreeColumns => None,
            Layout::DoubleMainVertical => None,
            Layout::DoubleMainHorizontal => None,
        }
    }
}

fn even_horizontal(num_panes: NumPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    let first_pane = parent_pane.split(Some(SplitDirection::Right), None);
    let mut panes: Vec<Pane> = vec![first_pane];

    for p in 0..num_panes.0 {
        match panes.get(p - 1) {
            Some(parent_pane) => {
                let pane = parent_pane.split(Some(SplitDirection::Right), None);
                panes.push(pane)
            }
            None => break,
        }
    }

    Some(panes)
}
