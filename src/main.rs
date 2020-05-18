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
    handlers::do_arloste,
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
            .route("/sim/arloste", web::get().to(async move |r| {

                let mut log_stream = io::stdout(); // TODO: invent how to bring here mutable log_stream and leave closure just Fn and not FnMut or FnOnce

                do_arloste(r).await.unwrap_or_else(|msg| {
                    log_stream.write(msg.as_bytes()).unwrap();
                    log_stream.flush().unwrap();
                    HttpResponse::Ok().json(format!("¯\\_(ツ)_/¯"))
                })

            }))
    })
    .bind(socket)?
    .run()
    .await
}
