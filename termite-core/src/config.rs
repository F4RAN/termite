use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Config{
    pub api_url: String,
    pub database_url: String
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        envy::from_env::<Config>().expect("Missing or invalid env vars (e.g. API_URL)")
    }
}