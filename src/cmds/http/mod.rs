use super::RunnableCmd;
use http::Uri;
use hyper::{Body, Client, Request, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
use tokio::runtime::Runtime;

#[derive(Serialize, Deserialize, Debug)]
pub struct RunnableHttpCmd {
    http_service_info: HttpServiceInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpServiceInfo {
    pub method: HttpMethod,
    pub url: String,
    pub response: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HttpMethod {
    GET,
    POST,
    UPDATE,
    DELETE,
    UNKNOWN,
}

impl RunnableHttpCmd {
    pub fn new(http_service_info: HttpServiceInfo) -> RunnableHttpCmd {
        RunnableHttpCmd { http_service_info }
    }
}

impl RunnableCmd for RunnableHttpCmd {
    fn run(&mut self) -> Result<(), String> {
        // println!("{:?} {} {:?}", self.method, self.url, self.response);

        let rt = Runtime::new().map_err(|e| {
            eprintln!("create tokio runtime failure: {}", e);
            "create tokio runtime failed.".to_owned()
        })?;

        let json = serde_json::to_string(&self.http_service_info)
            .map_err(|e| format!("serialize failed:{}", e))?;

        let port = self.http_service_info.port();
        let url = format!("http://localhost:{}/s/u/r/l/http", port);
        let url: &str = url.as_ref();
        let resp = rt
            .block_on(async {
                let req = Request::post(url).body(Body::from(json)).unwrap();
                Client::new().request(req).await
            })
            .map_err(|e| {
                eprintln!("request failure: {}", e);
                "request failed.".to_owned()
            })?;
        rt.shutdown_now();

        match resp.status() {
            StatusCode::OK => Ok(()),
            c => Err(format!("create cmd failed, status code: {}", c)),
        }
    }
}

impl HttpServiceInfo {
    pub fn new(method: HttpMethod, url: String, response: Option<String>) -> HttpServiceInfo {
        HttpServiceInfo {
            method,
            url,
            response,
        }
    }

    pub fn port(&self) -> u16 {
        let uri = if self.url.contains("://") {
            self.url.parse::<Uri>().unwrap()
        } else {
            let url = format!("http://{}", self.url);
            let s: &str = url.as_ref();
            s.parse::<Uri>().unwrap()
        };
        uri.port_u16().unwrap_or(80)
    }
}

impl HttpMethod {
    pub fn from(name: &str) -> HttpMethod {
        use HttpMethod::*;
        match name.to_uppercase().as_ref() {
            "GET" => GET,
            "POST" => POST,
            "UPDATE" => UPDATE,
            "DELETE" => DELETE,
            _ => UNKNOWN,
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
