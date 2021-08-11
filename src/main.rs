use crate::helper::fail_request;
use crate::tunnel::proxy_tunnel;
use config::GnixConfig;
use helper::HttpClient;
use hyper::http::uri::Authority;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Client, Method, Request, Response, Server, Uri};
use lazy_static::lazy_static;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

pub mod config;
pub mod helper;
pub mod tunnel;

lazy_static! {
    pub static ref CONFIG: Arc<RwLock<Option<GnixConfig>>> = Arc::new(RwLock::new(None));
}

#[tokio::main]
async fn main() {
    let config = confy::load::<GnixConfig>("gnix").expect("Invalid config");
    {
        *CONFIG.write().unwrap() = Some(config);
    }
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

    let port = {
        // let config = ;
        let config = CONFIG.read().unwrap();
        let config = config.as_ref().unwrap().clone();
        config
            .listen_http
            .expect("For now, please specify a http port, even if you dont need it")
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let server = Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(make_service);

    println!("server listening.");

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn proxy(client: HttpClient, mut req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let (host, path) = {
        let headers = req.headers();
        let host = match headers.get(header::HOST) {
            Some(h) => h.to_str().unwrap(),
            None => return Ok(fail_request(req, "no host header").await),
        };
        let host = host.split_once(":").unwrap_or((host, "")).0; // trim the port, if it even exists
        let path = req.uri().path();
        (host.to_string(), path.to_string())
    };

    {
        let config = CONFIG.read().unwrap();
        let config = config.as_ref().unwrap();
        let mut route = &config.fallback_route;

        for r in &config.route {
            if r.host != host {
                continue;
            }
            if let Some(p) = &r.path {
                if !path.starts_with(p) {
                    continue;
                }
            }
            route = r;
            break;
        }

        let new_host = route
            .backend_host
            .clone()
            .or_else(|| Some("127.0.0.1".to_string()))
            .unwrap();
        let new_port = route.backend_port;
        let new_authority = format!("{}:{}", new_host, new_port)
            .parse::<Authority>()
            .unwrap();

        println!("[{}] {} {}", host, req.method(), path);
        println!("-> {}:{} {}", new_host, new_port, req.uri().path());

        *req.uri_mut() = Uri::builder()
            .authority(new_authority)
            .scheme("http")
            .path_and_query("/index.html")
            .build()
            .unwrap();
    };

    // return Ok(fail_request(req, "request failed on purpose").await);

    if Method::CONNECT == req.method() {
        proxy_tunnel(client, req).await
    } else {
        client.request(req).await
    }
}
