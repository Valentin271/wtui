#[derive(Default, PartialEq)]
pub enum Focus {
    #[default]
    Main,
    Search,
    Yank,
}
