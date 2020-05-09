use std::net;

use actix_web::{
    web,
    App, 
    HttpServer,
};

use argh::FromArgs;

use simulation_server::handlers;

/// Solar irradiance server
#[derive(FromArgs)]
struct AppConfig {
    /// ip address client connects to
    #[argh(option, short = 'a')]
    address: String,

    /// port client connects to
    #[argh(option, short = 'p', default = "80")]
    port: u16,

    // TODO: add log configuration, default is stdout for now
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config: AppConfig = argh::from_env();
    let ip = match config.address.parse::<net::IpAddr>() {
        Ok(x) => x,
        Err(msg) => {
            println!("{:?}", msg);
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput, "Couldn't parse ip address!"
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
