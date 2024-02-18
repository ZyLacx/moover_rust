use json::JsonValue;

use std::collections::HashSet;

use anyhow::Error;

use crate::util::security::dotenv_var;

use super::tenor_types::{ContentFilter, ArRange, MediaFilter};

pub struct Tenor {
    /// Specify the country of origin for the request. To do so, provide its two-letter ISO 3166-1 country code.
    country: Option<String>,
    /// Specify the default language to interpret the search string using ISO 639-1 language code + optional two-letter ISO 3166-1 country code.
    /// You can use the country code that you provide in locale to differentiate between dialects of the given language.
    locale: Option<String>,
    /// Specify the content safety filter level
    /// The default value is off
    contentfilter: Option<ContentFilter>,
    /// Set of GIF formats to filter
    /// By default returns all formats
    media_filter: Option<HashSet<MediaFilter>>,
    /// Filter to only include GIFS with the aspect ratios that fit within range
    /// Default value is all
    /// wide: 0.42 <= aspect ratio <= 2.36
    /// standard: 0.56 <= aspect ratio <= 1.78
    ar_range: Option<ArRange>,
    /// Whether to randomly order the response
    /// Default value is false
    random: Option<bool>,
    /// Fetch up to the specified number of results
    /// Default value (for Tenor) is 20 and maximum is 50
    /// if smaller or greater values is provided, the value is clamped to the min/max
    limit: Option<u8>,
    /// Retrieve results that start at the position
    /// Use a non-zero, non-empty value from next, returned by the API response, to get the next set of results
    pos: Option<String>
}

impl Tenor {
    pub fn new() -> Self {
        Tenor {
            country: None,
            locale: None,
            contentfilter: None,
            media_filter: None,
            ar_range: None,
            random: None,
            limit: None,
            pos: None
        }
    }

    /// Replaces current country with the passed one
    pub fn country(mut self, country: String) -> Self {
        self.country = Some(country);
        self
    }

    /// Replaces current locale with the passed one 
    pub fn locale(mut self, locale: String) -> Self {
        self.locale = Some(locale);
        self
    }

    /// Replaces current media_filter with the passed one
    pub fn media_filter(mut self, filter: HashSet<MediaFilter>) -> Self {
        self.media_filter = Some(filter);
        self
    }

    /// Replaces current contentfilter with the passed one
    pub fn contentfilter(mut self, filter: ContentFilter) -> Self {
        self.contentfilter = Some(filter);
        self
    }

    /// Replaces current media_filter with the passed one
    pub fn add_media_filter(mut self, filter: MediaFilter) -> Self {
        if self.media_filter.is_none() {
            let mut set = HashSet::new();
            set.insert(filter);
            self.media_filter = Some(set);
        }
        else {
            self.media_filter.as_mut().unwrap().insert(filter);
        }
        self
    }

    pub fn ar_range(mut self, range: ArRange) -> Self {
        self.ar_range = Some(range);
        self
    }

    pub fn random(mut self, random: bool) -> Self {
        self.random = Some(random);
        self
    }

    pub fn limit(mut self, mut limit: u8) -> Self {
        if limit < 20 {
            limit = 20;
        }
        else if limit > 50 {
            limit = 50;
        }

        self.limit = Some(limit);
        self
    }

    pub fn pos(mut self, pos: String) -> Self {
        self.pos = Some(pos);
        self
    }

    pub async fn search(self, query: &str) -> Result<JsonValue, Error> {
        use anyhow::Context;

        let q: String = form_urlencoded::byte_serialize(query.as_bytes()).collect();

        // TODO encode query for urls (replace special characters and stuff)
        let base_url = "https://tenor.googleapis.com/v2/search?";
        let api_key = dotenv_var("TENORV2").context("TENORV2 key not found in the .env")?;

        let mut request = format!("{base_url}q={q}&key={api_key}");
        
        if self.country.is_some() {
            request.push_str(&format!("&country={}", self.country.unwrap()));
        }
        
        if self.locale.is_some() {
            request.push_str(&format!("&locale={}", self.locale.unwrap()));
        }
        
        if self.contentfilter.is_some() {
            request.push_str(&format!("&contentfilter={}", self.contentfilter.unwrap()))
        }

        if self.media_filter.is_some() && self.media_filter.as_ref().unwrap().len() > 0 {
             request.push_str(
                format!("&media_filter={}",
                    self.media_filter.unwrap()
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                        .as_str()
                ).as_str()
            );
        }
        
        if self.ar_range.is_some() {
            request.push_str(&format!("&ar_range={}", self.ar_range.unwrap()));
        }
        
        if self.random.is_some() {
            request.push_str(&format!("&random={}", self.random.unwrap()));
        }
        
        if self.limit.is_some() {
            request.push_str(&format!("&limit={}", self.limit.unwrap()));
        }

        let response = reqwest::get(request).await?.text().await?;

        let response_json = json::parse(&response)?;

        Ok(response_json)
    }
}