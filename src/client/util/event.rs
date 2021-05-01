use std::io;
use std::sync::mpsc;
use std::thread;

use signal_hook::consts::signal;
use signal_hook::iterator::exfiltrator::SignalOnly;
use signal_hook::iterator::SignalsInfo;

use termion::event::Event;
use termion::input::TermRead;

#[derive(Debug)]
pub enum TsuchitaEvent {
    Termion(Event),
    Signal(i32),
}

#[derive(Debug, Clone, Copy)]
pub struct Config {}

impl Default for Config {
    fn default() -> Config {
        Config {}
    }
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    pub event_tx: mpsc::Sender<TsuchitaEvent>,
    event_rx: mpsc::Receiver<TsuchitaEvent>,
    pub input_tx: mpsc::SyncSender<()>,
}

impl Events {
    pub fn new() -> Self {
        Events::with_config()
    }

    pub fn with_config() -> Self {
        let (input_tx, input_rx) = mpsc::sync_channel(1);
        let (event_tx, event_rx) = mpsc::channel();

        // signal thread
        let event_tx2 = event_tx.clone();
        let _ = thread::spawn(move || {
            let sigs = vec![signal::SIGWINCH];
            let mut signals = SignalsInfo::<SignalOnly>::new(&sigs).unwrap();
            for signal in &mut signals {
                if let Err(e) = event_tx2.send(TsuchitaEvent::Signal(signal)) {
                    eprintln!("Signal thread send err: {:#?}", e);
                    return;
                }
            }
        });

        // input thread
        let event_tx2 = event_tx.clone();
        let _ = thread::spawn(move || {
            let stdin = io::stdin();
            let mut events = stdin.events();
            match events.next() {
                Some(event) => match event {
                    Ok(event) => {
                        if let Err(e) = event_tx2.send(TsuchitaEvent::Termion(event)) {
                            eprintln!("Input thread send err: {:#?}", e);
                            return;
                        }
                    }
                    Err(_) => return,
                },
                None => return,
            }

            while input_rx.recv().is_ok() {
                if let Some(Ok(event)) = events.next() {
                    if let Err(e) = event_tx2.send(TsuchitaEvent::Termion(event)) {
                        eprintln!("Input thread send err: {:#?}", e);
                        return;
                    }
                }
            }
        });

        Events {
            event_tx,
            event_rx,
            input_tx,
        }
    }

    pub fn next(&self) -> Result<TsuchitaEvent, mpsc::RecvError> {
        let event = self.event_rx.recv()?;
        Ok(event)
    }

    pub fn flush(&self) {
        let _ = self.input_tx.send(());
    }
}

impl std::default::Default for Events {
    fn default() -> Self {
        Events::new()
    }
}