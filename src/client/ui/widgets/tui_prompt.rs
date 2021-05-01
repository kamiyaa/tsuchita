use termion::event::{Event, Key};
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Clear, Paragraph, Wrap};

use crate::context::AppContext;
use crate::ui::views::TuiView;
use crate::ui::TuiBackend;
use crate::util::event::TsuchitaEvent;

pub struct TuiPrompt<'a> {
    prompt: &'a str,
}

impl<'a> TuiPrompt<'a> {
    pub fn new(prompt: &'a str) -> Self {
        Self { prompt }
    }

    pub fn get_key(&mut self, backend: &mut TuiBackend, context: &mut AppContext) -> Key {
        let terminal = backend.terminal_mut();

        context.flush_event();
        loop {
            let _ = terminal.draw(|frame| {});
        }
    }
}
