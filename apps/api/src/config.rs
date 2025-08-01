use std::env;
use rand::Rng;
use tracing::warn;

#[derive(Clone)]
pub struct Config {
    pub server_addr: String,
    pub ollama_url: String,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(val) => {
                if val.trim().is_empty() {
                    warn!("JWT_SECRET is empty, generating a random secret");
                    Self::generate_random_secret()
                } else if val.len() < 16 {
                    warn!("JWT_SECRET is too short ({} chars), minimum 16 required. Generating a random secret", val.len());
                    Self::generate_random_secret()
                } else {
                    val
                }
            },
            Err(_) => {
                warn!("No JWT_SECRET set in environment, generating a random secret");
                Self::generate_random_secret()
            }
        };
        Self {
            server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3001".to_string()),
            ollama_url: env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://ollama:11434/".to_string()),
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgresql://postgres:postgres@localhost:5432/sya_db".to_string()
            }),
            jwt_secret,
        }
    }

    fn generate_random_secret() -> String {
        let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::rng();
        (0..32)
            .map(|_| {
                let idx = rng.random_range(0..charset.len());
                charset[idx] as char
            })
            .collect()
    }
}
