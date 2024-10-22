use std::fmt::Formatter;

#[derive(Debug)]
pub struct TwitterAuthError;

#[derive(Debug)]
pub struct TwitterScrapeError;

impl std::fmt::Display for TwitterAuthError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Error authenticating with Twitter")
    }
}

impl std::fmt::Display for TwitterScrapeError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Error Scraping twitter")
    }
}

impl std::error::Error for TwitterAuthError {}
impl std::error::Error for TwitterScrapeError {}