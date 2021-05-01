use serde_derive::Deserialize;

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
