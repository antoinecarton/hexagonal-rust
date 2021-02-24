use crate::state::state_factory;
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{HttpServer, App, web};
use crate::web::routes::comics::get_comic;
use crate::web::routes::health_check::health_check;


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .data_factory(state_factory)
            .route("/health_check", web::get().to(health_check))
            .route("/comics/{upc}", web::get().to(get_comic))
    })
        .listen(listener)?
        .run();
    Ok(server)
}