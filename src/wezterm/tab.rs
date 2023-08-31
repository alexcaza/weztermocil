use super::cli::CLI;

#[derive(Debug)]
pub struct Tab {
    pub id: String,
}

impl Tab {
    pub fn new(cwd: Option<&str>) -> Tab {
        let id = CLI::create_tab(cwd);
        match id {
            Ok(id) => Tab { id },
            Err(e) => panic!("Couldn't create new tab! {}", e),
        }
    }
}
