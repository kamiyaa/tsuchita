use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use crate::config::AppConfig;
use crate::dbus_monitor;

#[get("/sources/")]
async fn get_sources() -> impl Responder {
    eprintln!("GET /sources");
    let sources = dbus_monitor::get_sources();
    HttpResponse::Ok().json(sources)
}

#[get("/source/{source}/messages/")]
async fn get_messages(web::Path(source): web::Path<String>) -> impl Responder {
    eprintln!("GET /source/{}/messages/", source);
    let messages = dbus_monitor::get_messages(source.as_str());
    HttpResponse::Ok().json(messages)
}

pub async fn serve(config: &AppConfig) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_sources).service(get_messages))
        .bind(config.server_ref().url.as_str())?
        .run()
        .await
}
