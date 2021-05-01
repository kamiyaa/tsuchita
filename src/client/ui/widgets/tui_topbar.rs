use std::path::Path;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget};

use crate::context::AppContext;

pub struct TuiTopBar<'a> {
    pub context: &'a AppContext,
    path: &'a str,
}

impl<'a> TuiTopBar<'a> {
    pub fn new(context: &'a AppContext, path: &'a str) -> Self {
        Self { context, path }
    }
}

impl<'a> Widget for TuiTopBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = Style::default();
        let x = area.left();
        let y = area.top();

        buf.set_string(x, y, self.path, style);
    }
}
