use serde_derive::Deserialize;

use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::util::display::DisplayOption;
use crate::util::sort;

use super::client::{ClientConfig, RawClientConfig};
use super::server::ServerConfig;

const fn default_true() -> bool {
    true
}
const fn default_scroll_offset() -> usize {
    6
}
const fn default_max_preview_size() -> u64 {
    2 * 1024 * 1024 // 2 MB
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawAppConfig {
    #[serde(default)]
    pub client: RawClientConfig,
    #[serde(default)]
    pub server: ServerConfig,
}

impl Flattenable<AppConfig> for RawAppConfig {
    fn flatten(self) -> AppConfig {
        AppConfig {
            _client: self.client.flatten(),
            _server: self.server,
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

    pub fn client_mut(&mut self) -> &mut ClientConfig {
        &mut self._client
    }

    pub fn server_ref(&self) -> &ServerConfig {
        &self._server
    }

    pub fn server_mut(&mut self) -> &mut ServerConfig {
        &mut self._server
    }

    pub fn display_options_ref(&self) -> &DisplayOption {
        &self.client_ref().display_options
    }
    pub fn display_options_mut(&mut self) -> &mut DisplayOption {
        &mut self.client_mut().display_options
    }

    pub fn sort_options_ref(&self) -> &sort::SortOption {
        self.display_options_ref().sort_options_ref()
    }
    pub fn sort_options_mut(&mut self) -> &mut sort::SortOption {
        self.display_options_mut().sort_options_mut()
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
