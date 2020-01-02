#[derive(Debug)]
pub enum Command {
    Config {
        path: bool,
        default: bool,
    },
    Fields,
    Import {
        path: String,
    },
    List {
        author: bool,
        isbn: bool,
        table: bool,
    },
    Update,
}
