use epub::doc::EpubDoc;
use std::path::Path;
use std::io;
use chrono::{DateTime, Utc, NaiveDate, NaiveTime, NaiveDateTime};

pub struct Epub {
    data: EpubDoc,
}

impl Epub {
    pub fn new(path: &Path) -> Result<Epub, io::Error> {
        let data = EpubDoc::new(path).unwrap();
        Ok(Epub { data: data })
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
        self.data.mdata("date")
            .map(|date| NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d"))
            .and_then(|date| date.ok())
            .map(|date| {
                let time = NaiveTime::from_hms(0, 0, 0);
                NaiveDateTime::new(date, time)
            })
            .map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc))
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