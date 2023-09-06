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
            // Skip doing any pane creation
            // if there's only 1 pane being passed.
            // We can just run the command in the
            // main pane
            return None;
        }

        match self {
            Layout::EvenHorizontal => even_horizontal(num_panes, parent_pane.clone()),
            Layout::EvenVertical => even_vertical(num_panes, parent_pane.clone()),
            Layout::MainVertical => main_vertical(num_panes, parent_pane.clone()),
            Layout::MainVerticalFlipped => main_vertical_flipped(num_panes, parent_pane.clone()),
            Layout::Tiled => None,
            Layout::ThreeColumns => None,
            Layout::DoubleMainVertical => None,
            Layout::DoubleMainHorizontal => None,
        }
    }
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

    if panes.len() > 0 {
        Some(panes)
    } else {
        None
    }
}

fn main_splits(
    num_panes: NumPanes,
    parent_pane: Pane,
    direction: SplitDirection,
) -> Option<Vec<Pane>> {
    let main_pane = parent_pane.split(Some(direction), &Some(String::from("50")), None);

    match split_even(
        NumPanes(num_panes.0 - 1),
        main_pane.clone(),
        SplitDirection::Bottom,
    ) {
        Some(mut panes) => {
            let mut all_panes = vec![main_pane.clone()];
            all_panes.append(&mut panes);
            Some(all_panes)
        }
        None => Some(vec![main_pane]),
    }
}

fn even_horizontal(num_panes: NumPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    split_even(num_panes, parent_pane, SplitDirection::Right)
}

fn even_vertical(num_panes: NumPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    split_even(num_panes, parent_pane, SplitDirection::Bottom)
}

fn main_vertical(num_panes: NumPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    main_splits(num_panes, parent_pane, SplitDirection::Left)
}

fn main_vertical_flipped(num_panes: NumPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    main_splits(num_panes, parent_pane, SplitDirection::Right)
}
