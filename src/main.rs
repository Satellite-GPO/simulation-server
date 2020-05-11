use std::net;

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
    let ip: net::IpAddr = match config.address.parse() {
        Ok(x) => x,
        Err(msg) => {
            println!("{:?}", msg);
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput, "Couldnt parse IP address!"
            ));
        }
    };

    let sock_addr = net::SocketAddr::new(ip, config.port);

    HttpServer::new(|| {
        App::new()
            .route("/sim/arloste", web::get().to(handlers::do_arloste))
    })
    .bind(sock_addr)?
    .run()
    .await
}
