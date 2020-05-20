use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct BookData {
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
