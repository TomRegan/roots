use std::ffi::OsStr;
use std::path::Path;

use chrono::{DateTime, Utc};

use crate::application::book::file::{BookFile, EpubFile, MobiFile};

pub mod file;
mod loader;

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
}

#[allow(dead_code)]
pub fn book_comparator(_l: &Book, _r: &Book) -> usize {
    unimplemented!()
}
