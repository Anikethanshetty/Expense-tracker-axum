use std::env;


#[derive(Debug,Clone)]
pub struct Config {
    pub db_url : String,
    pub jwt_secret : String,
    pub jwt_maxage : i64,
    pub port : u32 
}

impl Config {
    pub fn init() -> Self {
        let database_url = env::var("DATABASE_URL").expect("Database Url does not exist");
        let jwt_secret = env::var("jwt_secret").expect("jwt secret does not exist");
        let jwt_maxage = env::var("jwt_maxage").expect("jwt expiration time does not exist");
        let port = env::var("port")
                                            .unwrap_or_else(|_|{"8080".to_string()})
                                            .parse::<u32>()
                                            .expect("port number must be a u32");

        Self { 
            db_url: database_url, 
            jwt_secret,
            jwt_maxage:jwt_maxage.parse::<i64>().expect("expiration must be a i16"), 
            port 
        }
    }
}