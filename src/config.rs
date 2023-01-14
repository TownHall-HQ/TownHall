use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    pub cors_allow_origin: String,
    pub database_path: String,
    pub jwt_secret: String,
    pub server_host: IpAddr,
    pub server_port: u16,
}

impl Config {
    pub fn new() -> Self {
        let cors_allow_origin = Config::env_var::<String>("CORS_ALLOW_ORIGIN");
        let database_path = Config::env_var::<String>("DATABASE_DIRECTORY");
        let jwt_secret = Config::env_var::<String>("JWT_SECRET");
        let server_host = Config::env_var_opt::<IpAddr>("HOST")
            .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        let server_port = Config::env_var::<u16>("PORT");

        Self {
            cors_allow_origin,
            database_path,
            jwt_secret,
            server_host,
            server_port,
        }
    }

    fn env_var<T: FromStr>(key: &str) -> T {
        let value =
            env::var(key).unwrap_or_else(|_| panic!("Missing environment variable: {}", key));

        if let Ok(parsed) = str::parse::<T>(&value) {
            return parsed;
        }

        panic!(
            "Failed to parse environment variable value from key: {}",
            key
        );
    }

    #[allow(dead_code)]
    fn env_var_opt<T: FromStr>(key: &str) -> Option<T> {
        let value = env::var(key).ok()?;

        str::parse::<T>(&value).ok()
    }
}
