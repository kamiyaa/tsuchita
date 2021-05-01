use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Paragraph, Widget, Wrap};
use unicode_width::UnicodeWidthStr;

use crate::fs::TsuchitaMessage;
use crate::util::display::DisplayOption;
use crate::util::format;

const ELLIPSIS: &str = "…";

pub struct TuiInboxMessage<'a> {
    message: &'a TsuchitaMessage,
    display_options: &'a DisplayOption,
}

impl<'a> TuiInboxMessage<'a> {
    pub fn new(message: &'a TsuchitaMessage, display_options: &'a DisplayOption) -> Self {
        Self {
            message,
            display_options,
        }
    }
}

impl<'a> Widget for TuiInboxMessage<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 1 || area.height < 1 {
            return;
        }
        if area.width < 4 {
            return;
        }
        let x = area.left();
        let y = area.top();

        let drawing_width = area.width as usize;

        let style = Style::default();

        let title = format!("Title: {}", self.message.title());
        Paragraph::new(title)
            .wrap(Wrap { trim: true })
            .render(area, buf);

        let timestamp =
            format::time_to_local(self.message.timestamp(), self.display_options.date_format());
        buf.set_string(x, y + 3, format!("Timestamp: {}", timestamp), style);

        buf.set_string(x, y + 5, "Content:", style);
        let paragraph_area = Rect {
            y: y + 6,
            height: area.height - 6,
            ..area
        };
        Paragraph::new(self.message.content())
            .wrap(Wrap { trim: true })
            .render(paragraph_area, buf);
    }
}
