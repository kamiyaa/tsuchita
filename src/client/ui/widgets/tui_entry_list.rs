use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::fs::{EntryType, TsuchitaEntry, TsuchitaList};
use crate::util::style;

const ELLIPSIS: &str = "…";

pub struct TuiEntryList<'a> {
    list: &'a TsuchitaList,
}

impl<'a> TuiEntryList<'a> {
    pub fn new(list: &'a TsuchitaList) -> Self {
        Self { list }
    }
}

impl<'a> Widget for TuiEntryList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 1 || area.height < 1 {
            return;
        }
        if area.width < 4 {
            return;
        }
        let x = area.left();
        let y = area.top();

        if self.list.is_empty() {
            let style = Style::default().bg(Color::Red).fg(Color::White);
            buf.set_stringn(x, y, "empty", area.width as usize, style);
            return;
        }

        let curr_index = self.list.index;
        let skip_dist = curr_index / area.height as usize * area.height as usize;

        let drawing_width = area.width as usize;

        self.list
            .iter()
            .skip(skip_dist)
            .enumerate()
            .take(area.height as usize)
            .for_each(|(i, entry)| {
                let style = style::entry_style(entry);
                print_entry(buf, entry, style, (x + 1, y + i as u16), drawing_width - 1);
            });

        // draw selected entry in a different style
        let screen_index = curr_index % area.height as usize;

        let entry = self.list.curr_entry_ref().unwrap();
        let style = style::entry_style(entry).add_modifier(Modifier::REVERSED);

        let space_fill = " ".repeat(drawing_width);
        buf.set_string(x, y + screen_index as u16, space_fill.as_str(), style);

        print_entry(
            buf,
            entry,
            style,
            (x + 1, y + screen_index as u16),
            drawing_width - 1,
        );
    }
}

fn print_entry(
    buf: &mut Buffer,
    entry: &TsuchitaEntry,
    style: Style,
    (x, y): (u16, u16),
    drawing_width: usize,
) {
    let name_width = entry.name().width();
    buf.set_stringn(x, y, entry.name(), drawing_width, style);
}
