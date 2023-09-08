use std::fmt;

use crate::wezterm::pane::{Pane, SplitDirection};

#[derive(PartialEq, Clone, Copy)]
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
            Layout::Tiled => tiled(num_panes, parent_pane.clone()),
            Layout::ThreeColumns => three_columns(num_panes, parent_pane.clone()),
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
    num_panes: NumPanes,
    parent_pane: Pane,
    direction: SplitDirection,
) -> Option<Vec<Pane>> {
    let main_pane = parent_pane.split(Some(direction), &Some(String::from("50")), None, false);

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

fn tiled(num_panes: NumPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    let num_panes_even = num_panes.0 % 2 == 0;
    let mut all_panes = vec![];
    let left_pane = parent_pane;
    let right_pane = left_pane.split(
        Some(SplitDirection::Right),
        &Some(String::from("50")),
        None,
        false,
    );

    if num_panes.0 == 2 {
        all_panes.push(left_pane);
        all_panes.push(right_pane);
        return Some(all_panes);
    }

    let per_side;
    if num_panes_even {
        per_side = NumPanes(num_panes.0 / 2);
    } else {
        per_side = NumPanes((num_panes.0 - 1) / 2);

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

fn three_columns(num_panes: NumPanes, parent_pane: Pane) -> Option<Vec<Pane>> {
    // We have one pane already, so we only need 2 more columns.
    // TODO: Not correctly splitting. Doesn't take parent_pane size into account
    let cols = even_horizontal(NumPanes(2), parent_pane.clone()).unwrap_or(vec![]);
    let num_cols = cols.len();
    // Column panes already created, so remove them from the total count.
    let total_panes_to_gen = num_panes.0 - num_cols;
    let odd_num_panes = total_panes_to_gen % num_cols != 0;
    let panes_per_col = (total_panes_to_gen as f32 / 3.0).ceil();
    let mut panes = vec![];

    for (i, p) in cols.iter().enumerate() {
        if i == num_cols && odd_num_panes {
            break;
        }

        // TODO: Not correctly splitting. Can't take into account parent pane size.
        let v_panes = even_vertical(NumPanes(panes_per_col as usize), p.clone());

        if let Some(mut created_panes) = v_panes {
            panes.append(&mut created_panes);
        }
    }

    println!("Panes: {:?}", panes);
    Some(panes)
}
