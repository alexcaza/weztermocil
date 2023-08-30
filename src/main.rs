use weztermocil::wezterm::{
    pane::{Pane, SplitDirection},
    tab::Tab,
};

fn main() {
    let tab = Tab::new();
    println!("{:?}", tab);

    let pane = Pane::new(tab, Some(SplitDirection::Top));
    println!("{:?}", pane);
}
