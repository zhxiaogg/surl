use http::Uri;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum Cmd {
    ServerCmd {
        name: ServerCmdName,
        config: String,
        port: u16,
    },
    HttpCmd {
        method: HttpMethod,
        url: String,
        response: Option<String>,
    },
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpServiceInfo {
    pub method: HttpMethod,
    pub url: String,
    pub response: Option<String>,
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

#[derive(Serialize, Deserialize, Debug)]
pub enum HttpMethod {
    GET,
    POST,
    UPDATE,
    DELETE,
    UNKNOWN,
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

#[derive(Debug)]
pub enum ServerCmdName {
    Start,
    Stop,
    Unknown,
}

impl ServerCmdName {
    pub fn from(name: &str) -> ServerCmdName {
        match name {
            "start" => ServerCmdName::Start,
            "stop" => ServerCmdName::Stop,
            _ => ServerCmdName::Unknown,
        }
    }
}
