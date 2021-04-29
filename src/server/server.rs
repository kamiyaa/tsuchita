use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use crate::ADDRESS;
use crate::dbus_monitor;

#[get("/messages/{source}/")]
async fn get_messages(web::Path((source)): web::Path<(String)>) -> impl Responder {
    let messages = dbus_monitor::get_messages(source.as_str());
    println!("Messages: {:?}", messages);
    HttpResponse::Ok().json(messages)
}

pub async fn serve() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_messages)).bind(ADDRESS)?.run().await
}
