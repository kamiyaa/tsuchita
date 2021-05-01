use serde_derive::Deserialize;

use crate::config::Flattenable;
use crate::util::display::DisplayOption;

use super::display::RawDisplayOption;

#[derive(Clone, Debug, Deserialize)]
pub struct RawClientConfig {
    #[serde(default)]
    pub server_url: String,
    #[serde(default)]
    pub display_options: RawDisplayOption,
}

impl Flattenable<ClientConfig> for RawClientConfig {
    fn flatten(self) -> ClientConfig {
        ClientConfig {
            server_url: self.server_url,
            display_options: self.display_options.flatten(),
        }
    }
}

impl std::default::Default for RawClientConfig {
    fn default() -> Self {
        Self {
            server_url: "localhost:9991".to_string(),
            display_options: RawDisplayOption::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub server_url: String,
    pub display_options: DisplayOption,
}

impl ClientConfig {
    pub fn display_options_ref(&self) -> &DisplayOption {
        &self.display_options
    }
}

impl std::default::Default for ClientConfig {
    fn default() -> Self {
        Self {
            server_url: "localhost:9991".to_string(),
            display_options: DisplayOption::default(),
        }
    }
}
