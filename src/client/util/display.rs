use std::fs;

use tui::layout::Constraint;

use crate::util::sort;

pub const fn default_column_ratio() -> (usize, usize, usize) {
    (1, 3, 4)
}

#[derive(Clone, Debug)]
pub struct DisplayOption {
    pub column_ratio: (usize, usize, usize),
    pub _show_borders: bool,
    pub _sort_options: sort::SortOption,
    pub default_layout: [Constraint; 3],
    pub no_preview_layout: [Constraint; 3],
}

impl DisplayOption {
    pub fn show_borders(&self) -> bool {
        self._show_borders
    }

    pub fn sort_options_ref(&self) -> &sort::SortOption {
        &self._sort_options
    }

    pub fn sort_options_mut(&mut self) -> &mut sort::SortOption {
        &mut self._sort_options
    }
}

impl std::default::Default for DisplayOption {
    fn default() -> Self {
        let column_ratio = default_column_ratio();

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

        Self {
            column_ratio,
            _show_borders: true,
            _sort_options: sort::SortOption::default(),
            default_layout,
            no_preview_layout,
        }
    }
}
