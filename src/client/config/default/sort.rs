use serde_derive::Deserialize;

use crate::util::sort;

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct SortRawOption {
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default = "default_true")]
    pub reverse: bool,
    #[serde(default)]
    pub sort_method: Option<String>,
}

impl SortRawOption {
    pub fn into(self) -> sort::SortOption {
        let sort_method = match self.sort_method.as_ref() {
            Some(s) => sort::SortType::parse(s).unwrap_or(sort::SortType::Natural),
            None => sort::SortType::Natural,
        };
        sort::SortOption {
            case_sensitive: self.case_sensitive,
            reverse: self.reverse,
            sort_method,
        }
    }
}

impl std::default::Default for SortRawOption {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            reverse: true,
            sort_method: None,
        }
    }
}
