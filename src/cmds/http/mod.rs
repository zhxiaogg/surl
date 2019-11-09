use super::RunnableCmd;
use http::{Method, Uri};
use hyper::{Body, Client, Request, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;
use std::iter::Iterator;
use std::str::FromStr;
use tokio::runtime::Runtime;

#[derive(Serialize, Deserialize, Debug)]
pub struct RunnableHttpCmd {
    http_service_info: HttpServiceInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpServiceInfo {
    pub method: String,
    pub url: String,
    pub response: Option<String>,
    pub headers: BTreeMap<String, Option<String>>,
}

#[derive(PartialEq, Eq, Hash)]
pub struct HttpServiceId {
    method: String,
    uri: String,
}

impl RunnableHttpCmd {
    pub fn new(http_service_info: HttpServiceInfo) -> RunnableHttpCmd {
        RunnableHttpCmd { http_service_info }
    }
}

impl HttpServiceId {
    pub fn from(req: &Request<Body>) -> HttpServiceId {
        let uri = req.uri().path().to_owned();
        let method = req.method().as_str().to_owned();
        HttpServiceId { method, uri }
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
    pub fn new(
        method_str: &str,
        url: String,
        response: Option<String>,
        header: Option<&str>,
    ) -> HttpServiceInfo {
        let method = Method::from_str(method_str).unwrap().as_ref().to_owned();
        let headers: BTreeMap<String, Option<String>> = HttpServiceInfo::create_header(header);
        HttpServiceInfo {
            method,
            url,
            response,
            headers,
        }
    }

    pub fn id(&self) -> HttpServiceId {
        HttpServiceId {
            method: self.method.to_owned(),
            uri: self.uri().path().to_owned(),
        }
    }

    fn create_header(header: Option<&str>) -> BTreeMap<String, Option<String>> {
        match header {
            Some(h) => {
                let mut m = BTreeMap::new();
                h.split(";")
                    .map(|s| {
                        let kv = s.splitn(2, ":").collect::<Vec<&str>>();
                        let k = kv.get(0).unwrap().trim().to_string();
                        let v = kv.get(1).map(|s| s.to_string());
                        (k, v)
                    })
                    .filter(|(k, _)| k.len() > 0)
                    .for_each(|(k, v)| {
                        m.insert(k, v);
                    });
                m
            }
            None => BTreeMap::new(),
        }
    }

    pub fn uri(&self) -> Uri {
        if self.url.contains("://") {
            self.url.parse::<Uri>().unwrap()
        } else {
            let url = format!("http://{}", self.url);
            let s: &str = url.as_ref();
            s.parse::<Uri>().unwrap()
        }
    }

    pub fn port(&self) -> u16 {
        let uri = self.uri();
        uri.port_u16().unwrap_or(80)
    }
}
