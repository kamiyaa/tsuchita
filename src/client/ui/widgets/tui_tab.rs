use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Paragraph, Widget, Wrap};

use unicode_width::UnicodeWidthStr;

pub struct TuiTabBar<'a> {
    name: &'a str,
    curr: usize,
    len: usize,
}

impl<'a> TuiTabBar<'a> {
    pub fn new(name: &'a str, curr: usize, len: usize) -> Self {
        Self { name, curr, len }
    }
}

impl<'a> Widget for TuiTabBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let selected = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::REVERSED);

        let str1 = format!("{}/{}", self.curr + 1, self.len);
        let str2 = {
            let space_avail = if str1.width() >= area.width as usize {
                0
            } else {
                area.width as usize - str1.len()
            };
            if space_avail >= self.name.width() {
                self.name
            } else {
                "…"
            }
        };

        Paragraph::new(Span::styled(format!("{}: {}", str1, str2), selected))
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
