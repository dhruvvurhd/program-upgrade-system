use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub rpc_url: String,
    pub program_id: String,
    pub payer_keypair_path: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Config {
            database_url: std::env::var("DATABASE_URL")?,
            rpc_url: std::env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8899".to_string()),
            program_id: std::env::var("PROGRAM_ID")?,
            payer_keypair_path: std::env::var("PAYER_KEYPAIR_PATH")?,
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse()?,
        })
    }
}
