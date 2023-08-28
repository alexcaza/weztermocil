pub enum SplitDirection {}

pub struct Pane {
    number: i32,
}

impl Pane {
    pub fn new() -> Pane {
        Pane {
            // TODO: Implement calling command
            number: 0,
        }
    }

    // Would it be nice to know if this was a _new_ pane that's
    // being returned?
    pub fn split_pane(&self, direction: &SplitDirection) -> Pane {
        // TODO: Implement creating new pane
        // from command to wezterm cli

        Pane { number: 0 }
    }
}
