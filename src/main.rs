#![feature(async_closure)]

use std::{
    net,
    io, io::Write,
};

use actix_web::{
    web,
    App, 
    HttpServer,
    HttpResponse,
};

use simulation_server::{
    handlers,
    config::ServerConfig,
};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let config: ServerConfig = argh::from_env();

    let ip: net::IpAddr = config.address.parse()
        .map_err(|_| io::Error::new(
            io::ErrorKind::InvalidInput, "Couldnt parse IP address!"
        ))?;

    let socket = net::SocketAddr::new(ip, config.port);

    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/static", ".").show_files_listing())
            .route("/sim/arloste", web::get().to(async move |r| {

                let mut log_stream = io::stdout(); // TODO: invent how to bring here mutable log_stream and leave closure just Fn and not FnMut or FnOnce

                handlers::do_arloste_get(r).await.unwrap_or_else(|msg| {
                    log_stream.write(msg.as_bytes()).unwrap();
                    log_stream.flush().unwrap();
                    HttpResponse::Ok().json(format!("¯\\_(ツ)_/¯")) // TODO: fallback response more explanatory by inserting an error code. Ah, it means I can't return just String from get anymore and have to specify errors
                })

            }))
            .route("/services/arloste", web::post().to(handlers::do_arloste_post))
            .route("/", web::get().to(handlers::initial_page))
            .route("/{filename:.*}", web::get().to(handlers::static_files))
    })
    .bind(socket)?
    .run()
    .await
}
