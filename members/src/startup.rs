use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use std::net::TcpListener;
use crate::routes::{
    create, health_check
};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/", web::post().to(create)))
        .listen(listener)?
        .run();
    Ok(server)
}
