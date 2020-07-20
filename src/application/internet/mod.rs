// Handling Metadata, Dublin Core spec:
// https://dublincore.org/specifications/dublin-core/dcmi-terms/

// Google scrApePI
// https://www.google.com/search?q=a+tale+of+two+cities&tbas=0&tbm=bks
// http://www.google.com/books/feeds/volumes/5EIPAAAAQAAJ

// Google Books API
// https://www.googleapis.com/books/v1/volumes?q=quilting

pub mod metadata {

    use serde::Deserialize;
    use std::vec::Vec;

    #[derive(Debug, Deserialize)]
    pub struct VolumeIdentifier {
        identifier: String,
        #[serde(rename = "type")]
        kind: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct VolumeInfo {
        title: Option<String>,
        authors: Option<Vec<String>>,
        #[serde(rename = "publishedDate")]
        published_date: Option<String>,
        description: Option<String>,
        #[serde(rename = "industryIdentifiers")]
        industry_identifiers: Option<Vec<VolumeIdentifier>>,
        language: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Volume {
        #[serde(rename = "volumeInfo")]
        volume_info: VolumeInfo,
        
    }

    #[derive(Debug, Deserialize)]
    pub struct VolumeResponse {
        kind: String,
        #[serde(rename = "totalItems")]
        total_items: u64,
        items: Vec<Volume>,
    }

    pub fn request() -> Result<VolumeResponse, Box<dyn std::error::Error>> {
        println!("Making request");
        let resp: VolumeResponse =
            reqwest::blocking::get("https://www.googleapis.com/books/v1/volumes?q=Essential+Slick")?
                .json::<VolumeResponse>()?;
        Ok(resp)
    }

    // heuristic:
    // match isbn
    // match as many authors and title
    // match publication year and title
    // match title
    // match authors
}
