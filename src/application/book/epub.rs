use std::io;
use std::path::Path;

use chrono::{DateTime, Utc};
use epub::doc::EpubDoc;

pub struct Epub {
    data: EpubDoc,
}

impl Epub {
    pub fn new(path: &Path) -> Result<Epub, io::Error> {
        Ok(Epub { data: EpubDoc::new(path)? })
    }

    pub fn get_author(&self) -> Option<Vec<String>> {
        self.data.mdata("creator")
            .map(|string| string.split(',')
                .map(|str| str.trim())
                .map(|str| str.to_string())
                .collect::<Vec<_>>())
    }

    pub fn get_title(&self) -> Option<String> {
        self.data.mdata("title")
    }

    pub fn get_publisher(&self) -> Option<String> {
        self.data.mdata("publisher")
    }

    pub fn get_publish_date(&self) -> Option<DateTime<Utc>> {
        self.data.metadata.get("date")?.iter()
            .filter_map(|str| DateTime::parse_from_rfc3339(str.as_str()).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .next()
    }

    pub fn get_imprint(&self) -> Option<String> {
        None
    }

    pub fn get_description(&self) -> Option<String> {
        self.data.mdata("description")
    }

    pub fn get_subject(&self) -> Option<Vec<String>> {
        self.data.metadata.get("subject").cloned()
    }

    pub fn get_asin(&self) -> Option<String> {
        None
    }

    pub fn get_isbn(&self) -> Option<String> {
        None
    }
}