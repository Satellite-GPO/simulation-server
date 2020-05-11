use std::{
    net,
    io,
};

use actix_web::{
    web,
    App, 
    HttpServer,
};

use simulation_server::{
    handlers,
    config::ServerConfig,
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config: ServerConfig = argh::from_env();

    let ip: net::IpAddr = config.address.parse()
        .map_err(|_| io::Error::new(
            io::ErrorKind::InvalidInput, "Couldnt parse IP address!"
        ))?;

    let socket = net::SocketAddr::new(ip, config.port);

    HttpServer::new(|| {
        App::new()
            .route("/sim/arloste", web::get().to(handlers::do_arloste))
    })
    .bind(socket)?
    .run()
    .await
}
