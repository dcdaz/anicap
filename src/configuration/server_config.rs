use serde::Deserialize;
use config::ConfigError;

const CONFIG_FILE_NAME: &str = "anicap.yml";

// This lazy_static allows to have some sort of global singleton
// to access from other modules to the same configuration without creating new instances
lazy_static! {
    pub static ref SERVER_CONFIG: ServerConfig = ServerConfig::new();
}


#[derive(Deserialize, Debug, Clone)]
pub struct ServerDataBaseConfig {
    pub db_url: String,
    pub pool_size: u32,
}

impl Default for ServerDataBaseConfig {
    fn default() -> Self {
        ServerDataBaseConfig {
            db_url: String::from("mysql://user:pass@ip:port/db_name"),
            pool_size: 6
        }
    }

}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerTokenConfig {
    pub jwt_secret: String,
    pub duration: u16
}

impl Default for ServerTokenConfig {
    fn default() -> Self {
        ServerTokenConfig {
            jwt_secret: String::from("anicap-super-secret-key"),
            duration: 60
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub ip_address: String,
    pub server_port: u16,
    pub log_type: String,
    pub database: ServerDataBaseConfig,
    pub token: ServerTokenConfig
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            ip_address: String::from("0.0.0.0"),
            server_port: 8085,
            log_type: String::from("actix_web=DEBUG"),
            database: ServerDataBaseConfig::default(),
            token: ServerTokenConfig::default()
        }
    }
}

impl ServerConfig {

    // New instance of ServerConfig
    // gets current filepath with YML config file in it
    // then tries to get configuration
    // if something happens will get Default config for all properties
    pub fn new() -> Self {
        let file_path = std::env::current_dir().unwrap();
        let full_config_path = format!(
            "{}/{}",
            file_path.as_os_str().to_str().unwrap(),
            CONFIG_FILE_NAME
        );

        match Self::get_configuration(full_config_path.as_str()) {
            Ok(config) => config,
            Err(_error) => {
                println!("\n- Default config will be used due to:\n\t{}\n", _error);
                Self::default()
            }
        }
    }

    fn get_configuration(file_path: &str) -> Result<Self, ConfigError> {
        use config::{Config, File};

        let config = Config::builder()
            .add_source(File::with_name(file_path))
            .build();
        match config {
            Ok(_) => config.unwrap().try_deserialize(),
            Err(_error) => {
                println!("\n- Default config will be used due to:\n\t{}\n", _error);
                Ok(Self::default())
            }
        }
    }
}

#[cfg(test)]
mod settings_test {

    use super::*;

    #[test]
    fn get_default_db_config() {
        let expected = ServerDataBaseConfig {
            db_url: String::from("./anicap.db"),
            pool_size: 6
        };

        let actual = ServerConfig::new();
        assert_eq!(actual.database.db_url, expected.db_url);
        assert_eq!(actual.database.pool_size, expected.pool_size);
    }

    #[test]
    fn get_default_token_config() {
        let expected = ServerTokenConfig {
            jwt_secret: String::from("anicap-super-secret-key"),
            duration: 60
        };

        let actual = ServerConfig::new();
        assert_eq!(actual.token.jwt_secret, expected.jwt_secret);
        assert_eq!(actual.token.duration, expected.duration);
    }

    #[test]
    fn get_default_server_config() {
        let expected = ServerConfig {
            ip_address: String::from("0.0.0.0"),
            server_port: 8085,
            log_type: String::from("actix_web=DEBUG"),
            database: ServerDataBaseConfig::default(),
            token: ServerTokenConfig::default()
        };

        let actual = ServerConfig::new();
        assert_eq!(actual.ip_address, expected.ip_address);
        assert_eq!(actual.server_port, expected.server_port);
        assert_eq!(actual.log_type, expected.log_type);
    }
}
