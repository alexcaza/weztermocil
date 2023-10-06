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
    pub fn create(&self, total_panes: TotalPanes, starting_pane: Pane) -> Option<Vec<Pane>> {
        if total_panes == TotalPanes(1) {
            // Skip doing any pane creation
            // if there's only 1 pane being passed.
            // We can just run the command in the
            // main pane
            return None;
        }

        match self {
            Layout::EvenHorizontal => even_horizontal(total_panes, starting_pane.clone()),
            Layout::EvenVertical => even_vertical(total_panes, starting_pane.clone()),
            Layout::MainVertical => main_vertical(total_panes, starting_pane.clone()),
            Layout::MainVerticalFlipped => {
                main_vertical_flipped(total_panes, starting_pane.clone())
            }
            Layout::Tiled => tiled(total_panes, starting_pane.clone()),
            Layout::ThreeColumns => three_columns(total_panes, starting_pane.clone()),
            Layout::DoubleMainVertical => double_main_vertical(total_panes, starting_pane.clone()),
            Layout::DoubleMainHorizontal => {
                double_main_horizontal(total_panes, starting_pane.clone())
            }
        }
    }
}

fn split_even(
    total_panes: TotalPanes,
    starting_pane: Pane,
    direction: SplitDirection,
) -> Option<Vec<Pane>> {
    let mut panes: Vec<Pane> = vec![starting_pane.clone()];

    let remaining_panes_count = if total_panes.0 - panes.len() == 0 {
        // There should _always_ be at least one pane
        panes.len()
    } else {
        total_panes.0 - panes.len()
    };

    // If there's one other pane to create, split parent once at 50% and return
    if remaining_panes_count == 1 {
        let pane = starting_pane.split(Some(direction), &Some(String::from("50")), None, false);
        panes.push(pane);
        return Some(panes);
    }

    for p in 0..remaining_panes_count {
        let pane_perc = ((1.0 / (total_panes.0 - p) as f32) * 100.0)
            .round()
            .to_string();
        let pane = starting_pane.split(Some(direction), &Some(pane_perc), None, false);
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
    starting_pane: Pane,
    direction: SplitDirection,
) -> Option<Vec<Pane>> {
    let main_pane = starting_pane.split(Some(direction), &Some(String::from("50")), None, false);

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

fn even_horizontal(total_panes: TotalPanes, starting_pane: Pane) -> Option<Vec<Pane>> {
    split_even(total_panes, starting_pane, SplitDirection::Right)
}

fn even_vertical(total_panes: TotalPanes, starting_pane: Pane) -> Option<Vec<Pane>> {
    split_even(total_panes, starting_pane, SplitDirection::Bottom)
}

fn main_vertical(total_panes: TotalPanes, starting_pane: Pane) -> Option<Vec<Pane>> {
    main_splits(total_panes, starting_pane, SplitDirection::Left)
}

fn main_vertical_flipped(total_panes: TotalPanes, starting_pane: Pane) -> Option<Vec<Pane>> {
    main_splits(total_panes, starting_pane, SplitDirection::Right)
}

fn tiled(total_panes: TotalPanes, starting_pane: Pane) -> Option<Vec<Pane>> {
    let total_panes_even = total_panes.0 % 2 == 0;
    let mut all_panes = vec![];
    let left_pane = starting_pane;
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
    all_panes.append(&mut left_panes);
    all_panes.append(&mut right_panes);
    Some(all_panes)
}

fn three_columns(total_panes: TotalPanes, starting_pane: Pane) -> Option<Vec<Pane>> {
    let mut cols =
        split_even(TotalPanes(3), starting_pane.clone(), SplitDirection::Right).unwrap_or(vec![]);

    // HACK: Wezterm's split rules are a little finnicky.
    // When generating the columns, the last column gets put
    // in the center spot in the tab instead of at the end, which is what
    // would be expected from 3 horizontal splits.
    // To combat this, we manually move the last tab back one in the vector.

    // The columns should exist. It's safe to panic otherwise.
    let last_col = cols.pop().unwrap();
    let middle_col = cols.pop().unwrap();
    cols.push(last_col);
    cols.push(middle_col);

    let num_cols = cols.len();
    // Column panes already created, so remove them from the total count.
    let total_panes_to_gen = total_panes.0 - num_cols;
    let panes_per_col = (total_panes_to_gen as f32 / 3.0).ceil();
    let mut panes_left = total_panes_to_gen;
    let mut panes = vec![];

    for pane in cols.iter() {
        // When we've run out of panes, stop iterating.
        // This accounts for scenarios where we get less
        // rows than total columns, or an odd number of rows.
        if panes_left == 0 {
            break;
        }

        let v_panes = split_even(
            TotalPanes(panes_per_col as usize + 1),
            pane.clone(),
            SplitDirection::Bottom,
        );

        panes_left -= panes_per_col as usize;

        if let Some(mut created_panes) = v_panes {
            panes.append(&mut created_panes);
        }
    }

    Some(panes)
}

fn double_main_vertical(total_panes: TotalPanes, starting_pane: Pane) -> Option<Vec<Pane>> {
    let cols =
        split_even(TotalPanes(3), starting_pane.clone(), SplitDirection::Right).unwrap_or(vec![]);
    // TODO: Not use clone; this might be causing the double id issue
    let mut panes = cols.clone();

    // HACK: Wezterm's split rules are a little finnicky.
    // When generating the columns, the last column gets put
    // in the center spot in the tab instead of at the end, which is what
    // would be expected from 3 horizontal splits.
    // To combat this, we manually move the last tab back one in the vector.

    // The columns should exist. It's safe to panic otherwise.
    let visually_last_col = cols.get(1).unwrap();

    let num_cols = cols.len();
    // Column panes already created, so remove them from the total count.
    let total_panes_to_gen = total_panes.0 - num_cols;

    if total_panes_to_gen == 0 {
        return Some(panes);
    }

    let v_panes = split_even(
        TotalPanes(total_panes_to_gen + 1),
        visually_last_col.clone(),
        SplitDirection::Bottom,
    );

    if let Some(mut created_panes) = v_panes {
        panes.append(&mut created_panes);
    }

    Some(panes)
}

fn double_main_horizontal(total_panes: TotalPanes, starting_pane: Pane) -> Option<Vec<Pane>> {
    let row_count = 2;
    let rows = split_even(
        TotalPanes(row_count),
        starting_pane.clone(),
        SplitDirection::Bottom,
    )
    .unwrap_or(vec![]);

    // The columns should exist. It's safe to panic otherwise.
    let visually_first_row = rows.get(0).unwrap();
    let visually_last_row = rows.get(1).unwrap();

    let mut panes = vec![];

    let bottom_panes = split_even(
        TotalPanes(2),
        visually_last_row.clone(),
        SplitDirection::Right,
    );

    if let Some(mut created_panes) = bottom_panes {
        panes.append(&mut created_panes);
    };

    // Remove row panes from count since they've already been generated.
    let total_panes_to_gen = total_panes.0 - row_count;

    if total_panes_to_gen == 0 {
        return Some(panes);
    }

    let top_panes = split_even(
        TotalPanes(total_panes_to_gen),
        visually_first_row.clone(),
        SplitDirection::Right,
    );

    if let Some(mut created_panes) = top_panes {
        panes.append(&mut created_panes);
    }

    Some(panes)
}
