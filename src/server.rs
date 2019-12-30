use std::io;

use actix_files as fs;
use actix_web::{
    client::Client, middleware, web, App, Either, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};
use url::Url;

use crate::cli_args::CliArgs;

async fn index_file(args: web::Data<CliArgs>) -> Result<fs::NamedFile> {
    fs::NamedFile::open(&args.index_file).map_err(|e| {
        actix_web::error::InternalError::new(
            format!("Error loading index file {}, {}\r\n", args.index_file, e),
            actix_web::http::StatusCode::NOT_FOUND,
        )
        .into()
    })
}

async fn default_handler(args: web::Data<CliArgs>) -> Either<HttpResponse, Result<fs::NamedFile>> {
    if args.no_spa {
        Either::A(
            HttpResponse::NotFound()
                .body("requested resource not found\r\n")
                .into(),
        )
    } else {
        Either::B(index_file(args).await)
    }
}

async fn forward(
    req: HttpRequest,
    body: web::Bytes,
    url: web::Data<Url>,
) -> Result<HttpResponse, Error> {
    let mut new_url = url.as_ref().clone();
    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

    log::debug!("Forwarding request {} to {}", req.uri().path(), new_url);

    let client = Client::new();
    // TODO: This forwarded implementation is incomplete as it only handles the inofficial
    // X-Forwarded-For header but not the official Forwarded one.
    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();
    let forwarded_req = if let Some(addr) = req.head().peer_addr {
        forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
    } else {
        forwarded_req
    };

    let mut res = forwarded_req.send_body(body).await.map_err(Error::from)?;

    let mut client_resp = HttpResponse::build(res.status());
    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    Ok(client_resp.body(res.body().await?))
}

pub(crate) async fn start(args: CliArgs) -> io::Result<()> {
    let port = args.port;
    let workers = args.workers;

    let res = HttpServer::new(move || {
        let mut app = App::new()
            .data(args.clone())
            .wrap(middleware::Logger::default());
        // add the proxy paths to the router
        for (path, url) in args.clone().proxies {
            app = app
                .data(url.clone())
                .service(web::scope(&path).default_service(web::to(forward)));
        }
        // add index and static file server routes
        app.service(web::resource("/").to(index_file))
            .service(fs::Files::new("/", &args.directory))
            .default_service(web::to(default_handler))
    })
    .workers(workers)
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await;
    res
}
