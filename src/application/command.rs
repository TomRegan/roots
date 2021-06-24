#[derive(Debug)]
pub enum Command {
    Config {
        path: bool,
        default: bool,
    },
    Fields,
    Find {
        show_scores: bool,
    },
    Import {
        path: String,
    },
    Info {
        path: String,
        fetch: bool,
    },
    List {
        author: bool,
        isbn: bool,
        table: bool,
    },
    Update,
}
