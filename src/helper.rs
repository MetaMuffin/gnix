use hyper::{
    header::{self, HeaderValue},
    http, Body, Client, Request, Response, StatusCode,
};
use sanitize_html::{rules::predefined::DEFAULT, sanitize_str};

pub type HttpClient = Client<hyper::client::HttpConnector>;

pub fn host_addr(uri: &http::Uri) -> Option<String> {
    uri.authority().and_then(|auth| Some(auth.to_string()))
}

pub async fn fail_request(req: Request<Body>, message: &str) -> Response<Body> {
    let headers = req.headers();
    let a = HeaderValue::from_static("(no host)");
    let host = headers.get(header::HOST).unwrap_or(&a).to_str().unwrap();
    let host = host.split_once(":").unwrap_or((host, "")).0; // trim the port, if it even exists

    let mut res = Response::new(
        format!(
            "<html>
                <head>
                </head>
                <body>
                    <p>Here is a lovely error for you! <3</p>
                    <code>
                        Error: {}<br>
                        while trying to request '{}' on host '{}'
                    </code>
                    <p>with love - <i>gnix<i></p>
                </body>
            </html>",
            sanitize_str(&DEFAULT, message).unwrap(),
            sanitize_str(&DEFAULT, req.uri().path()).unwrap(),
            sanitize_str(&DEFAULT, host).unwrap()
        )
        .into(),
    );
    *res.status_mut() = StatusCode::BAD_GATEWAY;
    res.headers_mut()
        .insert("Content-Type", HeaderValue::from_static("text/html"));
    res
}
