
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GnixRoute {
    pub path: Option<String>,
    pub host: String,
    pub backend_port: u16,
    pub backend_host: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GnixConfig {
    pub listen_http: Option<u16>,
    pub listen_https: Option<u16>,
    pub routes: Vec<GnixRoute>,
}

impl ::std::default::Default for GnixConfig {
    fn default() -> Self {
        Self {
            listen_http: Some(8080),
            listen_https: None,
            routes: vec![
                GnixRoute {
                    host: "host-a.local".to_string(),
                    path: Some("/test".to_string()),
                    backend_port: 8081,
                    backend_host: None,
                },
                GnixRoute {
                    host: "host-a.local".to_string(),
                    path: None,
                    backend_port: 80,
                    backend_host: Some("example.com".to_string()),
                },
                GnixRoute {
                    host: "host-b.local".to_string(),
                    path: None,
                    backend_port: 8081,
                    backend_host: None,
                },
            ],
        }
    }
}
