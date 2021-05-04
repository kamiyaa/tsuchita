use termion::event::Event;

use crate::commands::{CommandKeybind, KeyCommand};
use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::ui::views::TuiView;
use crate::ui::TuiBackend;
use crate::util::event::AppEvent;
use crate::util::input;

pub fn run(
    backend: &mut TuiBackend,
    context: &mut AppContext,
    keymap_t: AppKeyMapping,
) -> std::io::Result<()> {
    while !context.exit {
        backend.render(TuiView::new(&context));

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        match event {
            AppEvent::Termion(Event::Mouse(event)) => {
                context.flush_event();
            }
            AppEvent::Termion(key) => {
                match keymap_t.as_ref().get(&key) {
                    None => {
                        // handle error
                    }
                    Some(CommandKeybind::SimpleKeybind(command)) => {
                        if let Err(e) = command.execute(context, backend) {
                            // handle error
                        }
                    }
                    Some(CommandKeybind::CompositeKeybind(m)) => {
                        /*
                                                let cmd = {
                                                    let mut menu = TuiCommandMenu::new();
                                                    menu.get_input(backend, context, &m)
                                                };
                                                if let Some(command) = cmd {
                                                    if let Err(e) = command.execute(context, backend) {
                                                        // handle error
                                                    }
                                                }
                        */
                    }
                }
                context.flush_event();
            }
            event => input::process_noninteractive(event, context),
        }
    }
    Ok(())
}
