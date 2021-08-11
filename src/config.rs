use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GnixRoute {
    pub path: Option<String>,
    pub host: String,
    pub backend_port: u16,
    pub backend_host: Option<String>,
    pub backend_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GnixConfig {
    pub listen_http: Option<u16>,
    pub listen_https: Option<u16>,
    pub route: Vec<GnixRoute>,
    pub fallback_route: GnixRoute,
}

impl ::std::default::Default for GnixConfig {
    fn default() -> Self {
        Self {
            listen_http: Some(match users::get_current_uid() {
                0 => 80,
                _ => 8080
            }),
            listen_https: None,
            fallback_route: GnixRoute {
                backend_port: 8080,
                backend_host: None,
                path: None,
                host: "".to_string(),
                backend_path: None,
            },
            route: vec![
                GnixRoute {
                    host: "host-a.local".to_string(),
                    path: None,
                    backend_port: 8081,
                    backend_host: None,
                    backend_path: None,
                },
            ],
        }
    }
}
