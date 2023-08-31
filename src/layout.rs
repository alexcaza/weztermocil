pub enum Layout {
    Even,
    Odd,
}

impl Layout {
    pub fn create(&self, num_panes: i32) {
        match self {
            Layout::Even => even_layout(num_panes),
            Layout::Odd => odd_layout(num_panes),
        }
    }
}

fn even_layout(num_panes: i32) {
    // TODO do stuff
    println!("even_layout::num_panes: {}", num_panes);
}

fn odd_layout(num_panes: i32) {
    // todo do stuff
    println!("odd_layout::num_panes: {}", num_panes);
}
