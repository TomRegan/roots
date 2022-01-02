use std::io;
use std::path::Path;
use chrono::{ DateTime, Utc };


pub struct Mobi {
}

impl Mobi {
    pub fn new(path: &Path) -> Result<Mobi, io::Error> {
        Ok(Mobi {} )
    }

    pub fn get_author(&self) -> Option<Vec<String>> {
        let vector = Vec::new();
        Some(vector)
    }

    pub fn get_title(&self) -> Option<String> {
        Some("".to_string())
    }

    pub fn get_publisher(&self) -> Option<String> {
        Some("".to_string())
    }

    pub fn get_publish_date(&self) -> Option<DateTime<Utc>> {
        Some(Utc::now())
    }

    pub fn get_imprint(&self) -> Option<String> {
        Some("".to_string())
    }

    pub fn get_description(&self) -> Option<String> {
        Some("".to_string())
    }

    pub fn get_subject(&self) -> Option<Vec<String>> {
        let vector = Vec::new();
        Some(vector)
    }

    pub fn get_asin(&self) -> Option<String> {
        Some("".to_string())
    }

    pub fn get_isbn(&self) -> Option<String> {
        Some("".to_string())
    }
}
