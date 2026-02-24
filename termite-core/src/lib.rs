mod config;
use config::Config;


pub(crate) fn get_api_url_from_config(config: &Config) -> String {
    config.api_url.clone()
}

pub fn get_api_url() -> String {
    get_api_url_from_config(&Config::from_env())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    #[test]
    fn api_url_check() {
        let config = Config {
            api_url: "https://api.example.com".to_string(),
        };
        let result = get_api_url_from_config(&config);
        assert_eq!(result, "https://api.example.com");
    }
}