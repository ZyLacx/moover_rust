use dotenv::dotenv;
use std::env;

pub fn dotenv_var(key: &str) -> Option<String> {
    dotenv().ok();

    match env::var(key) {
        Ok(val) => return Some(val),
        Err(_) => None
    }
}