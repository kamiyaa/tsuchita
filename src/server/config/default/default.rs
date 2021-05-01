use serde_derive::Deserialize;

use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};

#[derive(Clone, Debug, Deserialize)]
pub struct ClientConfig {
    #[serde(default)]
    server_url: String,
}

impl std::default::Default for ClientConfig {
    fn default() -> Self {
        Self {
            server_url: "localhost:9991".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    #[serde(default)]
    pub url: String,
}

impl std::default::Default for ServerConfig {
    fn default() -> Self {
        Self {
            url: "localhost:9991".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawAppConfig {
    #[serde(default, rename = "client")]
    _client: ClientConfig,
    #[serde(default, rename = "server")]
    _server: ServerConfig,
}

impl Flattenable<AppConfig> for RawAppConfig {
    fn flatten(self) -> AppConfig {
        AppConfig {
            _client: self._client,
            _server: self._server,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    _client: ClientConfig,
    _server: ServerConfig,
}

impl AppConfig {
    pub fn new(server: ServerConfig, client: ClientConfig) -> Self {
        Self {
            _server: server,
            _client: client,
        }
    }

    pub fn client_ref(&self) -> &ClientConfig {
        &self._client
    }

    pub fn server_ref(&self) -> &ServerConfig {
        &self._server
    }
}

impl ConfigStructure for AppConfig {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<RawAppConfig, AppConfig>(file_name).unwrap_or_else(Self::default)
    }
}

impl std::default::Default for AppConfig {
    fn default() -> Self {
        Self {
            _server: ServerConfig::default(),
            _client: ClientConfig::default(),
        }
    }
}
