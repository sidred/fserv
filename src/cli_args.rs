use std::error::Error;

use structopt::StructOpt;
use url::Url;

fn parse_key_vals<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

/// fserv serves a directory via http. Allows you to optionally configure some routes to be proxied to another endpoint
///
/// [Usage]
/// fserv -p <port> -i <index_file> -d <directory> -f <path>=<target> -f <path>=<target>
///
/// [Example]
///
/// fserv -p 9000 -i html_data/index.html  -d build -f api=http://localhost:9010 -f ws=http://localhost:9020
///
/// This starts the server on port 9000 and serves the files in build directory. Requests to /api and /ws will be proxied {n}
/// - localhost:9000/               will load the html_data/index.html file {n}
/// - localhost:9000/main.css       will load the build/main.css file if available {n}
/// - localhost:9000/logo.png       will load the build/logo.png file if available {n}
/// - localhost:9000/unknown.png    will load the html_data/index.html file if the file does not exist {n}
/// - localhost:9000/user/info      will load the html_file/index.html file {n}
/// - localhost:9000/api/list       will proxy the request to http://localhost:9010/api/list {n}
/// - localhost:9000/ws/updates     will proxy the request to http://localhost:9020/ws/updates {n}
#[derive(StructOpt, Clone, Debug)]
pub(crate) struct CliArgs {
    /// Sets the port
    #[structopt(short = "p", long = "port", default_value = "8000")]
    pub(crate) port: u16,

    /// Disables spa mode.
    /// When spa mode is enabled an index.html (configured by -i flag) file is returned when a resourece is not found
    /// When spa mode is disabled a 404 error is returned when a resource is not found
    #[structopt(short = "n", long = "no-spa")]
    pub(crate) no_spa: bool,

    /// Directory to serve. Uses current directory by default
    #[structopt(short = "d", long = "directory", default_value = ".")]
    pub(crate) directory: String,

    /// The file to server if when a resource is not found and spa mode is enabled.
    #[structopt(short = "i", long = "index-file", default_value = "index.html")]
    pub(crate) index_file: String,

    /// Optional path to forward as a key=value pair. Multiple entries can be added
    #[structopt(short = "f", long = "forward", parse(try_from_str = parse_key_vals), number_of_values = 1)]
    pub(crate) proxies: Vec<(String, Url)>,

    /// Sets the number of worker threads
    #[structopt(short = "w", long = "workers", default_value = "1")]
    pub(crate) workers: usize,
}

pub(crate) fn parse_args() -> CliArgs {
    let args = CliArgs::from_args();

    log::info!("{:?}", args);
    log::info!("SpaMode={}", !args.no_spa);
    log::info!("DirectoryPath={}", args.directory);
    log::info!("IndexFile={}", args.index_file);
    log::info!("WorkerCount={}", args.workers);
    for (k, v) in args.proxies.iter() {
        log::info!("Proxy: {} -> {} ", k, v);
    }
    log::info!("Starting server on port {}", args.port);
    args
}
