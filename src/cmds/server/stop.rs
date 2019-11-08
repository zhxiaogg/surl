use crate::cmds::RunnableCmd;
use hyper::{Body, Client, Request, StatusCode};
use tokio::runtime::Runtime;

pub struct StopCmd {
    config: String,
    port: u16,
}

impl StopCmd {
    pub fn new(config: String, port: u16) -> StopCmd {
        StopCmd { config, port }
    }
}

impl RunnableCmd for StopCmd {
    fn run(&mut self) -> Result<(), String> {
        let rt = Runtime::new().map_err(|e| {
            eprintln!("create tokio runtime failure: {}", e);
            "create tokio runtime failed.".to_owned()
        })?;

        let url = format!("http://localhost:{}/s/u/r/l/stop", self.port);
        let url: &str = url.as_ref();
        let resp = rt
            .block_on(async {
                let req = Request::post(url).body(Body::empty()).unwrap();
                Client::new().request(req).await
            })
            .map_err(|e| {
                eprintln!("request failure: {}", e);
                "request failed.".to_owned()
            })?;
        rt.shutdown_now();

        match resp.status() {
            StatusCode::OK => Ok(()),
            c => Err(format!("stop failed, status code: {}", c)),
        }
    }
}
