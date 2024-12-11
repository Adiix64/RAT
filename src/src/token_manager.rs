use std::env;

pub struct TokenManager;

impl TokenManager {
    pub fn get_token() -> String {
        env::var("TELOXIDE_TOKEN").unwrap_or_else(|_| {
            eprintln!("Environment variable TELOXIDE_TOKEN not found. Using default token.");
            "default_token".to_string()
        })
    }
}
