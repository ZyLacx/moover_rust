use super::tenor_types::MediaFilter;

use json::JsonValue;

pub fn get_gif_url(filter: MediaFilter, tenor_response: JsonValue) -> anyhow::Result<Vec<String>> {
    use anyhow::Context;

    let mut urls: Vec<String> = Vec::new();
    let results = tenor_response["results"].members();
    
    for result in results {
        let url = result["media_formats"][filter.to_string()]["url"].as_str().context("Value not found in Json")?;
        urls.push(url.to_string());
    }

    Ok(urls)    
}