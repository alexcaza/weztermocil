use std::fmt;

use crate::wezterm::pane::{Pane, SplitDirection};

#[derive(PartialEq, Clone, Copy)]
pub struct TotalPanes(pub usize);

impl fmt::Display for TotalPanes {
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
    pub fn create(&self, total_panes: TotalPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
        if total_panes == TotalPanes(1) {
            // Skip doing any pane creation
            // if there's only 1 pane being passed.
            // We can just run the command in the
            // main pane
            return None;
        }

        match self {
            Layout::EvenHorizontal => even_horizontal(total_panes, parent_pane.clone()),
            Layout::EvenVertical => even_vertical(total_panes, parent_pane.clone()),
            Layout::MainVertical => main_vertical(total_panes, parent_pane.clone()),
            Layout::MainVerticalFlipped => main_vertical_flipped(total_panes, parent_pane.clone()),
            Layout::Tiled => tiled(total_panes, parent_pane.clone()),
            Layout::ThreeColumns => three_columns(total_panes, parent_pane.clone()),
            Layout::DoubleMainVertical => None,
            Layout::DoubleMainHorizontal => None,
        }
    }
}

fn split_even(
    total_panes: TotalPanes,
    parent_pane: Pane,
    direction: SplitDirection,
) -> Option<Vec<Pane>> {
    let mut panes: Vec<Pane> = vec![parent_pane.clone()];

    let remaining_panes_count = if total_panes.0 - panes.len() == 0 {
        // There should _always_ be at least one pane
        panes.len()
    } else {
        total_panes.0 - panes.len()
    };

    // If there's one other pane to create, split parent once at 50% and return
    if remaining_panes_count == 1 {
        let pane = parent_pane.split(Some(direction), &Some(String::from("50")), None, false);
        panes.push(pane);
        return Some(panes);
    }

    for p in 0..remaining_panes_count {
        let pane_perc = ((1.0 / (total_panes.0 - p) as f32) * 100.0)
            .round()
            .to_string();
        let pane = parent_pane.split(Some(direction), &Some(pane_perc), None, false);
        panes.push(pane);
    }

    if panes.len() > 0 {
        Some(panes)
    } else {
        None
    }
}

fn main_splits(
    total_panes: TotalPanes,
    parent_pane: Pane,
    direction: SplitDirection,
) -> Option<Vec<Pane>> {
    let main_pane = parent_pane.split(Some(direction), &Some(String::from("50")), None, false);

    match split_even(
        TotalPanes(total_panes.0 - 1),
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

fn even_horizontal(total_panes: TotalPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    split_even(total_panes, parent_pane, SplitDirection::Right)
}

fn even_vertical(total_panes: TotalPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    split_even(total_panes, parent_pane, SplitDirection::Bottom)
}

fn main_vertical(total_panes: TotalPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    main_splits(total_panes, parent_pane, SplitDirection::Left)
}

fn main_vertical_flipped(total_panes: TotalPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    main_splits(total_panes, parent_pane, SplitDirection::Right)
}

fn tiled(total_panes: TotalPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    let total_panes_even = total_panes.0 % 2 == 0;
    let mut all_panes = vec![];
    let left_pane = parent_pane;
    let right_pane = left_pane.split(
        Some(SplitDirection::Right),
        &Some(String::from("50")),
        None,
        false,
    );

    if total_panes.0 == 2 {
        all_panes.push(left_pane);
        all_panes.push(right_pane);
        return Some(all_panes);
    }

    let per_side;
    if total_panes_even {
        per_side = TotalPanes(total_panes.0 / 2);
    } else {
        per_side = TotalPanes((total_panes.0 - 1) / 2);

        // If panes are odd, create a bottom pane at the top level
        let bottom_pane = left_pane.split(Some(SplitDirection::Bottom), &None, None, true);
        all_panes.push(bottom_pane);
    }

    let mut left_panes = even_vertical(per_side, left_pane.clone()).unwrap_or(vec![]);
    let mut right_panes = even_vertical(per_side, right_pane.clone()).unwrap_or(vec![]);
    all_panes.push(left_pane);
    all_panes.push(right_pane);
    all_panes.append(&mut left_panes);
    all_panes.append(&mut right_panes);
    Some(all_panes)
}

fn three_columns(total_panes: TotalPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    let cols =
        split_even(TotalPanes(3), parent_pane.clone(), SplitDirection::Right).unwrap_or(vec![]);
    let num_cols = cols.len();
    // Column panes already created, so remove them from the total count.
    let total_panes_to_gen = total_panes.0 - num_cols;
    let odd_total_panes = total_panes_to_gen % num_cols != 0;
    let panes_per_col = (total_panes_to_gen as f32 / 3.0).ceil();
    let mut panes = vec![];

    for (i, p) in cols.iter().enumerate() {
        if i == num_cols - 1 && odd_total_panes {
            break;
        }

        let v_panes = split_even(
            TotalPanes(panes_per_col as usize),
            p.clone(),
            SplitDirection::Bottom,
        );

        if let Some(mut created_panes) = v_panes {
            panes.append(&mut created_panes);
        }
    }

    Some(panes)
}
