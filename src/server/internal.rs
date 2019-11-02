use hyper::{Body, Error, Method, Request, Response, Server as HyperServer, StatusCode};
use tokio::sync::oneshot::Sender;

pub struct InternalService {
    shutdown_handle: ShutdownHandle,
}

impl InternalService {
    pub fn new(shutdown_handle: ShutdownHandle) -> InternalService {
        InternalService { shutdown_handle }
    }
    pub async fn process(&mut self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        match (
            req.method(),
            req.uri().path().replace("/s/u/r/l", "").to_lowercase().as_ref(),
        ) {
            (&Method::POST, "/stop") => 
            match self.shutdown_handle.shutdown() {
                Ok(_) => Ok(Response::new(Body::from("done"))),
                Err(_) => {
                    eprintln!("shutdown failed.");
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("failed")).unwrap())
                }
            },
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap())
        }
    }
}

pub struct ShutdownHandle {
    kill_sender:Option<Sender<()>>,
}

impl ShutdownHandle {
    pub fn shutdown(&mut self) -> Result<(),()> {
        match self.kill_sender.take() {
            Some(s) => s.send(()),
            None => Err(()),
        }
    }

    pub fn new(kill_sender:Sender<()>) -> ShutdownHandle {
        ShutdownHandle{kill_sender:Some(kill_sender)}
    }
}
