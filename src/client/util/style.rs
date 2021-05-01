use tui::style::{Color, Modifier, Style};

use crate::fs::{EntryType, TsuchitaEntry};
use crate::THEME_T;

pub fn entry_style(entry: &TsuchitaEntry) -> Style {
    match entry.get_type() {
        EntryType::Source => Style::default()
            .fg(THEME_T.source.fg)
            .bg(THEME_T.source.bg)
            .add_modifier(THEME_T.source.modifier),
        EntryType::Message(_) => Style::default()
            .fg(THEME_T.message.fg)
            .bg(THEME_T.message.bg)
            .add_modifier(THEME_T.message.modifier),
    }
}
