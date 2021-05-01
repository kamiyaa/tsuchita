use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget};

pub struct TuiFooter {}

impl TuiFooter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for TuiFooter {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
