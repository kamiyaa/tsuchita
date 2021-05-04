use signal_hook::consts::signal;

use crate::context::AppContext;
use crate::util::event::AppEvent;

pub fn process_noninteractive(event: AppEvent, context: &mut AppContext) {
    match event {
        AppEvent::Signal(signal::SIGWINCH) => {}
        _ => {}
    }
}
