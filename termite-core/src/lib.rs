mod config;
use config::Config;



pub fn get_api_url() -> String {
    let c = Config::from_env();
    c.api_url.clone()
}


pub fn get_db_url() -> String {
    let c = Config::from_env();
    c.database_url.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        
    }
}