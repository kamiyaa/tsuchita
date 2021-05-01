use crate::message::TsuchitaMessage;

use dbus::arg::TypeMismatchError;
use dbus::blocking::Connection;
use dbus::channel::MatchingReceiver;
use dbus::message::{MatchRule, Message, MessageType};

use lazy_static::lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

pub type Notifications = HashMap<String, Vec<TsuchitaMessage>>;

lazy_static! {
    static ref NOTIFICATIONS: Mutex<Notifications> = Mutex::new(Notifications::default());
}

pub fn handle_message(message: &Message) {
    let res: Result<(String, u32, String, String, String), TypeMismatchError> = message.read5();

    match res {
        Ok((source, _, _, title, content)) => {
            let timestamp = SystemTime::now();
            let t_message = TsuchitaMessage::new(source, title, content, timestamp);
            let mut notification_center = NOTIFICATIONS.lock().unwrap();
            notification_center
                .entry(t_message.source().to_string())
                .or_insert(vec![])
                .push(t_message);
            println!("{:#?}", *notification_center);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

pub fn get_sources() -> Vec<String> {
    let notification_center = NOTIFICATIONS.lock().unwrap();
    let sources = notification_center.keys().map(|s| s.to_string()).collect();
    sources
}

pub fn get_messages(source: &str) -> Vec<TsuchitaMessage> {
    let notification_center = NOTIFICATIONS.lock().unwrap();
    match notification_center.get(source) {
        Some(v) => v.clone(),
        None => vec![],
    }
}

pub fn listen() -> Result<(), dbus::Error> {
    let conn = Connection::new_session()?;
    let duration = Duration::from_secs(86400);

    let mut rule = MatchRule::new_signal("org.freedesktop.Notifications", "Notify");
    rule.msg_type = Some(MessageType::MethodCall);

    let proxy = conn.with_proxy("org.freedesktop.DBus", "/org/freedesktop/DBus", duration);
    let _result: Result<(), dbus::Error> = proxy.method_call(
        "org.freedesktop.DBus.Monitoring",
        "BecomeMonitor",
        (vec![rule.match_str()], 0u32),
    );

    conn.start_receive(
        rule,
        Box::new(|msg, _| {
            handle_message(&msg);
            true
        }),
    );

    loop {
        conn.process(duration).unwrap();
    }

    Ok(())
}
