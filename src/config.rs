use argh::FromArgs;

/// Solar irradiance simulation server
#[derive(FromArgs)]
pub struct ServerConfig {
    /// ip address client connects to
    #[argh(option, short = 'a')]
    pub address: String,

    /// port client connects to
    #[argh(option, short = 'p', default = "80")]
    pub port: u16,

    /// wheather to enable deprecated features or not (GET request for now)
    #[argh(switch)]
    pub deprecated: bool

    // TODO: add log configuration, default is stdout for now
}