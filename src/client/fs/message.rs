use serde::{Deserialize, Serialize};

use std::time::SystemTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TsuchitaMessage {
    #[serde(rename = "source")]
    _source: String,
    #[serde(rename = "title")]
    _title: String,
    #[serde(rename = "content")]
    _content: String,
    #[serde(rename = "timestamp")]
    _timestamp: SystemTime,
}

impl TsuchitaMessage {
    pub fn new(source: String, title: String, content: String, timestamp: SystemTime) -> Self {
        Self {
            _source: source,
            _title: title,
            _content: content,
            _timestamp: timestamp,
        }
    }

    pub fn source(&self) -> &str {
        self._source.as_str()
    }
    pub fn title(&self) -> &str {
        self._title.as_str()
    }
    pub fn content(&self) -> &str {
        self._content.as_str()
    }
    pub fn timestamp(&self) -> SystemTime {
        self._timestamp
    }
}
