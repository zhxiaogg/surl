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
    Unknown,
}

impl ServerCmdName {
    pub fn from(name: &str) -> ServerCmdName {
        match name {
            "start" => ServerCmdName::Start,
            _ => ServerCmdName::Unknown,
        }
    }
}
