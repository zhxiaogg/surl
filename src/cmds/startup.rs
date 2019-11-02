use super::RunnableCmd;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server};

pub struct StartupCmd {
    config: String,
    port: u16,
}

impl StartupCmd {
    pub fn new(config: String, port: u16) -> StartupCmd {
        StartupCmd { config, port }
    }
}

const PHRASE: &str = "Hello, World!";

fn hello_world(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(PHRASE))
}

impl RunnableCmd for StartupCmd {
    fn run(&mut self) -> Result<(), String> {
        let addr = ([127, 0, 0, 1], self.port).into();

        let new_svc = || service_fn_ok(hello_world);

        let server = Server::bind(&addr)
            .serve(new_svc)
            .map_err(|e| eprintln!("server error: {}", e));

        hyper::rt::spawn(server);

        Ok(())
    }
}
