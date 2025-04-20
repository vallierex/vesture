use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::env;
use secrecy::{ExposeSecret, SecretString};

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn connect_options(&self) -> PgConnectOptions {
        let ssl_mode: PgSslMode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
            .database(&self.database_name)
    }
}

pub fn get_configuration() -> Result<Settings, Box<dyn std::error::Error>> {
    dotenv::from_path("./postgres.env").expect("Failed to read .env file");

    let config = Settings {
        database: DatabaseSettings {
            username: env::var("DB_USERNAME")?,
            password: SecretString::new(Box::from(env::var("DB_PASSWORD")?)),
            host: env::var("DB_HOST")?,
            port: env::var("DB_PORT")?.parse()?,
            database_name: env::var("DB_NAME")?,
            require_ssl: env::var("DB_SSL")?.parse()?,
        },
    };

    Ok(config)
}
