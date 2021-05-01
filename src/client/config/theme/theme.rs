use serde_derive::Deserialize;

use tui::style::Color;

use super::{AppStyle, RawAppStyle};
use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};

const fn default_color() -> Color {
    Color::Reset
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawAppTheme {
    #[serde(default)]
    pub selection: RawAppStyle,
    #[serde(default)]
    pub source: RawAppStyle,
    #[serde(default)]
    pub message: RawAppStyle,
}

impl std::default::Default for RawAppTheme {
    fn default() -> Self {
        Self {
            selection: RawAppStyle::default(),
            source: RawAppStyle::default(),
            message: RawAppStyle::default(),
        }
    }
}

impl Flattenable<AppTheme> for RawAppTheme {
    fn flatten(self) -> AppTheme {
        AppTheme {
            selection: self.selection.to_style_theme(),
            source: self.source.to_style_theme(),
            message: self.message.to_style_theme(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppTheme {
    pub selection: AppStyle,
    pub source: AppStyle,
    pub message: AppStyle,
}

impl ConfigStructure for AppTheme {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<RawAppTheme, AppTheme>(file_name).unwrap_or_else(Self::default)
    }
}

impl std::default::Default for AppTheme {
    fn default() -> Self {
        Self {
            selection: AppStyle::default(),
            source: AppStyle::default(),
            message: AppStyle::default(),
        }
    }
}
