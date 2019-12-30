mod cli_args;
mod server;

use std::env;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info,actix_server=warn,actix_web=info")
    }

    env_logger::init();

    let args = cli_args::parse_args();

    actix_rt::System::new(stringify!("fserv")).block_on(async {
        match server::start(args).await {
            Ok(_) => log::info!("Shutting down"),
            Err(e) => log::error!("Server Error {:?}", e),
        };
    })
}
