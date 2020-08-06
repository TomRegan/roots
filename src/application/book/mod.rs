use std::ffi::OsStr;
use std::path::Path;

use chrono::{DateTime, Utc};

use application::book::file::{BookFile, EpubFile, MobiFile};
use application::internet::metadata::Volume;

pub mod file;
mod mobi;
mod epub;

#[derive(Debug)]
pub struct Book {
    pub title: Option<String>,
    pub author: Option<Vec<String>>,
    pub publisher: Option<String>,
    pub publication_date: Option<DateTime<Utc>>,
    pub imprint: Option<String>,
    pub description: Option<String>,
    pub subject: Option<Vec<String>>,
    pub asin: Option<String>,
    pub isbn: Option<String>,
}

impl Book {
    pub fn new(p: &Path) -> Book {
        match p.extension().and_then(OsStr::to_str) {
            Some("mobi") => MobiFile::new(p).book_data(),
            Some("epub") => EpubFile::new(p).book_data(),
            _ => panic!("oops")
        }
    }

    pub fn from(v: &Volume) -> Book {
        let info = v.volume_info.clone();
        Book {
            title: info.title,
            author: info.authors,
            publisher: info.publisher,
            publication_date:
            info.published_date
                .and_then(|s| DateTime::parse_from_rfc3339(s.as_str()).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            imprint: None,
            description: info.description,
            subject: None,
            asin: None,
            isbn: None,
        }
    }
}

#[allow(dead_code)]
pub fn book_comparator(_l: &Book, _r: &Book) -> usize {
    unimplemented!()
}
