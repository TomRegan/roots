#![allow(dead_code)]
// Handling Metadata, Dublin Core spec:
// https://dublincore.org/specifications/dublin-core/dcmi-terms/

// Google scrApePI
// https://www.google.com/search?q=a+tale+of+two+cities&tbas=0&tbm=bks
// http://www.google.com/books/feeds/volumes/5EIPAAAAQAAJ

// Google Books API
// https://www.googleapis.com/books/v1/volumes?q=a+tale+of+two+cities
// https://www.googleapis.com/books/v1/volumes/5EIPAAAAQAAJ

use chrono::{DateTime, Utc};
use application::book::Book;
use internet::metadata::Volume;

impl From<&Volume> for Book {
    fn from(volume: &Volume) -> Self {
        let info = volume.volume_info.clone();
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

pub mod metadata {
    use std::vec::Vec;
    use serde::Deserialize;
    use url::Url;

    use application::book::Book;

    #[derive(Debug, Deserialize, Clone)]
    pub struct VolumeIdentifier {
        pub identifier: String,
        #[serde(rename = "type")]
        pub kind: String,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct VolumeInfo {
        pub title: Option<String>,
        pub authors: Option<Vec<String>>,
        pub publisher: Option<String>,
        #[serde(rename = "publishedDate")]
        pub published_date: Option<String>,
        pub description: Option<String>,
        #[serde(rename = "industryIdentifiers")]
        pub industry_identifiers: Option<Vec<VolumeIdentifier>>,
        pub language: Option<String>,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct Volume {
        #[serde(rename = "volumeInfo")]
        pub volume_info: VolumeInfo,

    }

    #[derive(Debug, Deserialize)]
    pub struct VolumeResponse {
        kind: String,
        #[serde(rename = "totalItems")]
        total_items: u64,
        pub items: Vec<Volume>,
    }

    pub fn request(book: &Book) -> Result<VolumeResponse, Box<dyn std::error::Error>> {
        println!("Making request");
        let book_title = match book.title.as_ref() {
            Some(title) => title,
            None => panic!("Book is not identifiable! {:?}", book)
        };
        let url = match Url::parse(format!(
            "https://www.googleapis.com/books/v1/volumes?q={title}",
            title = book_title).as_str()) {
            Ok(url) => url,
            Err(error) => panic!("Problem parsing title {:?}", error)
        };
        println!("Requesting {}", url);
        let resp = match reqwest::blocking::get(url)?
            .json::<VolumeResponse>() {
            Ok(resp) => resp,
            Err(error) => panic!("Problem making request {:?}", error)
        };
        Ok(resp)
    }
}

// heuristic:
// match isbn
// match as many authors and title
// match publication year and title
// match title
// match authors
