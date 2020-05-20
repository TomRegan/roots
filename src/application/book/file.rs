use std::path::{Path, PathBuf};
use std::fs::canonicalize;
use super::Book;
use application::book::mobi::Mobi;

pub trait BookFile {
    fn as_book(&self) -> &Book;

    fn path(&self) -> &Path;

    fn book_data(self) -> Book;
}

pub struct EpubFile {
    path: PathBuf,
    book_data: Book,
}

impl EpubFile {
    pub fn new(path: &Path) -> EpubFile {
        EpubFile {
            path: canonicalize(path).unwrap_or(path.to_path_buf()),
            book_data: Book {
                title: None,
                author: None,
                publisher: None,
                publication_date: None,
                imprint: None,
                description: None,
                subject: None,
                asin: None,
                isbn: None,
            },
        }
    }
}

impl BookFile for EpubFile {
    fn as_book(&self) -> &Book {
        &self.book_data
    }

    fn path(&self) -> &Path {
        self.path.as_path()
    }

    fn book_data(self) -> Book {
        self.book_data
    }
}

#[derive(Debug)]
pub struct MobiFile {
    path: PathBuf,
    book_data: Book,
}

impl MobiFile {
    pub fn new(path: &Path) -> MobiFile {
        let book = Mobi::new(path).unwrap();
        MobiFile {
            path: canonicalize(path).unwrap_or(path.to_path_buf()),
            book_data: Book {
                title: book.get_title(),
                author: book.get_author(),
                publisher: book.get_publisher(),
                publication_date: book.get_publish_date(),
                imprint: book.get_imprint(),
                description: book.get_description(),
                subject: book.get_subject(),
                asin: book.get_asin(),
                isbn: book.get_isbn(),
            },
        }
    }
}

impl BookFile for MobiFile {
    fn as_book(&self) -> &Book {
        &self.book_data
    }

    fn path(&self) -> &Path {
        self.path.as_path()
    }

    fn book_data(self) -> Book {
        self.book_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epub_path_returns_path() {
        let book = EpubFile::new(Path::new("share/pg98.epub"));
        assert!(book.path().ends_with(Path::new("share/pg98.epub")));
    }

    #[test]
    fn mobi_path_returns_path() {
        let book = MobiFile::new(Path::new("share/pg98.mobi"));
        assert!(book.path().ends_with(Path::new("share/pg98.mobi")));
    }
}