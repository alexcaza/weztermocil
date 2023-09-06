use std::fmt;

use crate::wezterm::pane::{Pane, SplitDirection};

#[derive(PartialEq)]
pub struct NumPanes(pub usize);

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
            Layout::EvenVertical => even_vertical(num_panes, parent_pane.clone()),
            Layout::MainVertical => None,
            Layout::MainVerticalFlipped => None,
            Layout::Tiled => None,
            Layout::ThreeColumns => None,
            Layout::DoubleMainVertical => None,
            Layout::DoubleMainHorizontal => None,
        }
    }
}

fn build_percentages(num_panes: &NumPanes) -> Vec<usize> {
    let max: usize = 100;
    let per_pane: usize = max / num_panes.0;
    let mut percentages: Vec<usize> = vec![];

    // TODO: build percentages for each pane based on the previous
    // percentage. Inspo: https://wezfurlong.org/wezterm/config/lua/gui-events/gui-startup.html
    // It's all about _remaining available space_
    for p in 0..num_panes.0 - 1 {
        if p == 0 {
            let percentage = &max;
            percentages.push(percentage - per_pane);
        } else {
            let percentage = percentages.get(p - 1).unwrap_or(&max);
            percentages.push(percentage - per_pane);
        }
    }

    println!("percentages: {:?}", percentages);

    percentages.reverse();
    percentages
}

fn split_even(
    num_panes: NumPanes,
    parent_pane: Pane,
    direction: SplitDirection,
) -> Option<Vec<Pane>> {
    let mut panes: Vec<Pane> = vec![];

    for p in 0..num_panes.0 {
        let pane_perc = ((1.0 / (num_panes.0 - p) as f32) * 100.0)
            .round()
            .to_string();
        let pane = parent_pane.split(Some(direction), &Some(pane_perc), None);
        panes.push(pane);
    }

    Some(panes)
}

fn even_horizontal(num_panes: NumPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    split_even(num_panes, parent_pane, SplitDirection::Right)
}

fn even_vertical(num_panes: NumPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    split_even(num_panes, parent_pane, SplitDirection::Bottom)
}
