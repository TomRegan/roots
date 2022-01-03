use std::io;
use std::fs;
use std::path::Path;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use epub::doc::EpubDoc;
use mobi::Mobi as MobiDoc;

fn convert(published_date: Option<String>) -> DateTime<Utc> {
    let parsed: Option<NaiveDate> = NaiveDate::parse_from_str(published_date.unwrap().as_str(), "%Y-%m-%d").ok();
    let date_component: NaiveDate = parsed.unwrap_or(NaiveDate::from_ymd(1970, 01, 01));
    let time_component: NaiveTime = NaiveTime::from_hms(0, 0, 0);
    let naive_date_time: NaiveDateTime = NaiveDateTime::new(date_component, time_component);
    let utc_date_time: DateTime<Utc> = DateTime::from_utc(naive_date_time, Utc);
    utc_date_time
}

pub struct EpubLoader {
    data: EpubDoc<fs::File>,
}

impl EpubLoader {
    pub fn new(path: &Path) -> Result<EpubLoader, io::Error> {
        Ok(EpubLoader { data: EpubDoc::new(path).unwrap() })
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
        let published_date: Option<String> = self.data.metadata.get("date").unwrap().first().cloned();
        let utc_date_time: DateTime<Utc> = convert(published_date);
        Some(utc_date_time)
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

pub struct MobiLoader {
    data: MobiDoc,
}

impl MobiLoader {
    pub fn new(path: &Path) -> Result<MobiLoader, io::Error> {
        Ok(MobiLoader { data: MobiDoc::from_path(path).unwrap() })
    }

    pub fn get_author(&self) -> Option<Vec<String>> {
        self.data.author().map(|str| vec![str])
    }

    pub fn get_title(&self) -> Option<String> {
        Some(self.data.title())
    }

    pub fn get_publisher(&self) -> Option<String> {
        self.data.publisher()
    }

    pub fn get_publish_date(&self) -> Option<DateTime<Utc>> {
        let published_date: Option<String> = self.data.publish_date();
        let utc_date_time: DateTime<Utc> = convert(published_date);
        Some(utc_date_time)
    }

    pub fn get_imprint(&self) -> Option<String> {
        None
    }

    pub fn get_description(&self) -> Option<String> {
        self.data.description()
    }

    pub fn get_subject(&self) -> Option<Vec<String>> {
        self.data.metadata.subjects()
    }

    pub fn get_asin(&self) -> Option<String> {
        None
    }

    pub fn get_isbn(&self) -> Option<String> {
        self.data.isbn()
    }
}
