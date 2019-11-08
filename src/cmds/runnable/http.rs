use super::RunnableCmd;
use crate::cmds::cmd::*;
use hyper::{Body, Client, Request, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::runtime::Runtime;

#[derive(Serialize, Deserialize, Debug)]
pub struct RunnableHttpCmd {
    http_service_info: HttpServiceInfo,
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
