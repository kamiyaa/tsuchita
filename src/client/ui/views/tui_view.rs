use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Widget};

use crate::context::AppContext;
use crate::fs::EntryType;
use crate::tree::DbusTreeTrait;
use crate::ui::widgets::{TuiEntryList, TuiInboxMessage, TuiTopBar};

pub struct TuiView<'a> {
    context: &'a AppContext,
}

impl<'a> TuiView<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Self { context }
    }
}

impl<'a> Widget for TuiView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constraints = [
            Constraint::Ratio(1, 8),
            Constraint::Ratio(3, 8),
            Constraint::Ratio(4, 8),
        ];

        let layout_rect = if true {
            let area = Rect {
                y: area.top() + 1,
                height: area.height - 2,
                ..area
            };
            let block = Block::default().borders(Borders::ALL);
            let inner = block.inner(area);
            block.render(area, buf);

            let layout_rect = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints.as_ref())
                .split(inner);

            let block = Block::default().borders(Borders::RIGHT);
            let inner1 = block.inner(layout_rect[0]);
            block.render(layout_rect[0], buf);

            let block = Block::default().borders(Borders::LEFT);
            let inner3 = block.inner(layout_rect[2]);
            block.render(layout_rect[2], buf);

            vec![inner1, layout_rect[1], inner3]
        } else {
            let mut layout_rect = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(1)
                .constraints(constraints.as_ref())
                .split(area);

            layout_rect[0] = Rect {
                width: layout_rect[0].width - 1,
                ..layout_rect[0]
            };
            layout_rect[1] = Rect {
                width: layout_rect[1].width - 1,
                ..layout_rect[1]
            };
            layout_rect
        };

        let topbar_width = area.width;
        let rect = Rect {
            x: 0,
            y: 0,
            width: topbar_width,
            height: 1,
        };
        TuiTopBar::new(self.context, self.context.path_ref()).render(rect, buf);

        let tree = self.context.tree_ref();
        if self.context.path_ref() == "/" {
            if let Some(list) = tree.get_sources() {
                TuiEntryList::new(list).render(layout_rect[1], buf);
                if let Some(entry) = list.curr_entry_ref() {
                    if let Some(list) = tree.get(entry.name()) {
                        TuiEntryList::new(list).render(layout_rect[2], buf);
                    }
                }
            }
        } else {
            if let Some(list) = tree.get_sources() {
                TuiEntryList::new(list).render(layout_rect[0], buf);
            }
            if let Some(list) = self.context.curr_list_ref() {
                TuiEntryList::new(list).render(layout_rect[1], buf);
                if let Some(entry) = list.curr_entry_ref() {
                    if let EntryType::Message(message) = entry.get_type() {
                        let display_options = self.context.config_ref().display_options_ref();
                        TuiInboxMessage::new(message, display_options).render(layout_rect[2], buf);
                    }
                }
            }
        }
    }
}
