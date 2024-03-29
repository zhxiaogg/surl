mod path_var;

use super::RunnableCmd;
use http::{Method, Uri};
use hyper::{Body, Client, Request, StatusCode};
use path_var::PathVarExtractor;
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
    path_var_extractor: Option<PathVarExtractor>,
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
        let uri = HttpServiceInfo::get_uri(url.as_str());
        let path_var_extractor = PathVarExtractor::new(uri.path());
        HttpServiceInfo {
            method,
            url,
            response,
            headers,
            path_var_extractor,
        }
    }

    pub fn extract_path_vars(&self, uri: &str) -> BTreeMap<String, String> {
        match &self.path_var_extractor {
            Some(extractor) => extractor.extract_vars(uri),
            None => BTreeMap::new(),
        }
    }

    fn unify_uri(&self, uri: &str) -> String {
        match &self.path_var_extractor {
            Some(extractor) => extractor.unify(uri),
            None => uri.to_owned(),
        }
    }

    pub fn id(&self) -> HttpServiceId {
        HttpServiceId {
            method: self.method.to_owned(),
            uri: self.unify_uri(self.uri().path()),
        }
    }

    pub fn id_for_req(&self, req: &Request<Body>) -> HttpServiceId {
        HttpServiceId {
            method: req.method().as_str().to_owned(),
            uri: self.unify_uri(req.uri().path()),
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
        HttpServiceInfo::get_uri(self.url.as_str())
    }

    fn get_uri(url: &str) -> Uri {
        if url.contains("://") {
            url.parse::<Uri>().unwrap()
        } else {
            let url = format!("http://{}", url);
            let s: &str = url.as_ref();
            s.parse::<Uri>().unwrap()
        }
    }

    pub fn port(&self) -> u16 {
        let uri = self.uri();
        uri.port_u16().unwrap_or(80)
    }
}
