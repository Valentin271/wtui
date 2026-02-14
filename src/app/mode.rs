#[derive(Default, PartialEq)]
pub enum Mode {
    #[default]
    Main,
    Search,
    Yank,
}
