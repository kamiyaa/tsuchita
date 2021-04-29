mod dbus_monitor;
mod message;
mod server;

use std::thread;

pub const ADDRESS: &str = "127.0.0.1:9991";

#[actix_web::main]
async fn main() {
    println!("Listening to dbus...");
    let _dbus_thread = thread::spawn(|| {
        let res = dbus_monitor::listen();
        match res {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    });
    println!("Running HTTP server on {}", ADDRESS);
    server::serve().await;
}
