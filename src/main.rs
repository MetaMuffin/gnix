use crate::helper::fail_request;
use crate::tunnel::proxy_tunnel;
use config::GnixConfig;
use helper::HttpClient;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Client, Method, Request, Response, Server};
use lazy_static::lazy_static;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::RwLock;

pub mod config;
pub mod helper;
pub mod tunnel;

// lazy_static! {
//     pub static ref CONFIG: RwLock<Option<GnixConfig>> = RwLock::new(None);
// }

#[tokio::main]
async fn main() {
    // let config = confy::load::<GnixConfig>("gnix").expect("Invalid config");
    // {
    //     *CONFIG.write().unwrap() = Some(config);
    // }
    start_http().await;
}

async fn start_http() {
    let client = Client::builder()
        .http1_title_case_headers(true)
        .http1_preserve_header_case(true)
        .build_http();

    let make_service = make_service_fn(move |_| {
        let client = client.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), req))) }
    });
    // let config = CONFIG.read().unwrap();
    // let config = config.as_ref().unwrap().clone();
    // let addr = SocketAddr::from((
    //     [0, 0, 0, 0],
    //     config
    //         .listen_http
    //         .expect("For now, please specify a http port, even if you dont need it"),
    // ));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let server = Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(make_service);

    println!("server listening.");

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn proxy(client: HttpClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let headers = req.headers();
    let host = match headers.get(header::HOST) {
        Some(h) => h.to_str().unwrap(),
        None => return Ok(fail_request(req, "no host header").await),
    };
    // let config = CONFIG.read().unwrap();
    // let config = config.as_ref().unwrap();

    return Ok(fail_request(req, "request failed on purpose").await);

    let host = host.split_once(":").unwrap_or((host, "")).0; // trim the port, if it even exists
    let path = req.uri().path();

    println!("[{}] {} {}", host, req.method(), path);

    if Method::CONNECT == req.method() {
        proxy_tunnel(client, req).await
    } else {
        let mut proxy_req = Request::from(req);
        // *proxy_req.uri_mut().host() = proxy_
        client.request(proxy_req).await
    }
}
