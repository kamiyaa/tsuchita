use serde_derive::Deserialize;

use tui::layout::Constraint;

use crate::config::Flattenable;
use crate::util::display::{default_column_ratio, DisplayOption};

use super::SortRawOption;

const fn default_true() -> bool {
    true
}

fn default_timezone() -> String {
    "local".to_string()
}

fn default_date_format() -> String {
    "%Y-%m-%d %I:%M %p".to_string()
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawDisplayOption {
    #[serde(default)]
    column_ratio: Option<[usize; 3]>,

    #[serde(default = "default_true")]
    show_borders: bool,

    #[serde(default = "default_timezone")]
    timezone: String,
    #[serde(default = "default_date_format")]
    date_format: String,

    #[serde(default, rename = "sort")]
    sort_options: SortRawOption,
}

impl Flattenable<DisplayOption> for RawDisplayOption {
    fn flatten(self) -> DisplayOption {
        let column_ratio = match self.column_ratio {
            Some(s) => (s[0], s[1], s[2]),
            _ => default_column_ratio(),
        };

        let total = (column_ratio.0 + column_ratio.1 + column_ratio.2) as u32;

        let default_layout = [
            Constraint::Ratio(column_ratio.0 as u32, total),
            Constraint::Ratio(column_ratio.1 as u32, total),
            Constraint::Ratio(column_ratio.2 as u32, total),
        ];
        let no_preview_layout = [
            Constraint::Ratio(column_ratio.0 as u32, total),
            Constraint::Ratio(column_ratio.1 as u32 + column_ratio.2 as u32, total),
            Constraint::Ratio(0, total),
        ];

        DisplayOption {
            column_ratio,
            _show_borders: self.show_borders,
            _sort_options: self.sort_options.into(),
            _timezone: self.timezone,
            _date_format: self.date_format,
            default_layout,
            no_preview_layout,
        }
    }
}

impl std::default::Default for RawDisplayOption {
    fn default() -> Self {
        Self {
            column_ratio: None,
            show_borders: true,
            timezone: default_timezone(),
            date_format: default_date_format(),
            sort_options: SortRawOption::default(),
        }
    }
}
