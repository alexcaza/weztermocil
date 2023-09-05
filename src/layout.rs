use std::fmt;

#[derive(PartialEq)]
pub struct NumPanes(i32);

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
    pub fn create(&self, num_panes: NumPanes) {
        if num_panes == NumPanes(1) {
            single_pane()
        }

        match self {
            Layout::EvenHorizontal => even_layout(num_panes),
            Layout::EvenVertical => even_layout(num_panes),
            Layout::MainVertical => even_layout(num_panes),
            Layout::MainVerticalFlipped => even_layout(num_panes),
            Layout::Tiled => even_layout(num_panes),
            Layout::ThreeColumns => even_layout(num_panes),
            Layout::DoubleMainVertical => even_layout(num_panes),
            Layout::DoubleMainHorizontal => even_layout(num_panes),
        }
    }
}

fn single_pane() {
    // TODO: Create pane
}

fn even_layout(num_panes: NumPanes) {
    println!("even_layout::num_panes: {}", num_panes);
}
