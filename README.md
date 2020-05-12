# Solar irradiance simulation server

The server calculates solar irradiance under given conditions.

You can find query reference at the repository's wiki.

## Installation and use

You can install the server with
* [x] cargo
* [ ] homebrew
* [ ] docker container

Yeah, now only the Rust way to build is supported so you have to [install and configure Rust toolchain](https://www.rust-lang.org/tools/install).

1. Clone [the repo](https://github.com/Satellite-GPO/simulation-server) and `cd` into
    ```shell
    $ git clone https://github.com/Satellite-GPO/simulation-server.git && cd simulation-server
    ```

1. Build it
    ```shell
    $ cargo build --release
    ```

1. The resulting binary is `target/release/simulation-server`. You can run it by name
    ```shell
    $ target/release/simulation-server -a 127.0.0.1 -p 8008
    ```
    or with cargo
    ```shell
    $ cargo run --release -- -a 127.0.0.1 -p 8008
    ```

* `-a` or `--address` sets the IP address for the server.

* `-p` or `--port` sets TCP port the server listens. The default port is 80.

    Note that first 1024 ports are unavailable for non-privileged users so make sure you are logged in as `root` or use `sudo` to run the server on port 80.

The application is a standalone software so you won't need separate HTTP server to run it.
