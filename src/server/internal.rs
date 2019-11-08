use super::container::ServiceContainer;
use crate::cmds::http::HttpServiceInfo;
use crate::utils::http::*;
use futures::lock::Mutex;
use hyper::{Body, Method, Request, Response, StatusCode};
use serde_json::Result as SerdeResult;
use std::sync::Arc;
use tokio::sync::oneshot::Sender;

pub struct InternalService {
    shutdown_handle: ShutdownHandle,
    service_container: Arc<Mutex<ServiceContainer>>,
}

impl InternalService {
    pub fn new(
        shutdown_handle: ShutdownHandle,
        service_container: Arc<Mutex<ServiceContainer>>,
    ) -> InternalService {
        InternalService {
            shutdown_handle,
            service_container,
        }
    }
    pub async fn process(&mut self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        match (
            req.method(),
            req.uri()
                .path()
                .replace("/s/u/r/l", "")
                .to_lowercase()
                .as_ref(),
        ) {
            (&Method::POST, "/stop") => match self.shutdown_handle.shutdown() {
                Ok(_) => Ok(Response::new(Body::from("done"))),
                Err(_) => {
                    eprintln!("shutdown failed.");
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("failed"))
                        .unwrap())
                }
            },
            (&Method::POST, "/http") => {
                let (parts, mut body) = req.into_parts();

                match body_to_str(&mut body).await {
                    Some(json) => {
                        let maybe_service_info: SerdeResult<HttpServiceInfo> =
                            serde_json::from_str(&json);
                        match maybe_service_info {
                            Ok(service_info) => {
                                let mut service_container = self.service_container.lock().await;
                                service_container.add_service(service_info);
                                ok()
                            }
                            Err(e) => {
                                eprintln!("deserialize json failed: {}", e);
                                bad_request()
                            }
                        }
                    }
                    _ => bad_request(),
                }
            }
            _ => not_found(),
        }
    }
}

pub struct ShutdownHandle {
    kill_sender: Option<Sender<()>>,
}

impl ShutdownHandle {
    pub fn shutdown(&mut self) -> Result<(), ()> {
        match self.kill_sender.take() {
            Some(s) => s.send(()),
            None => Err(()),
        }
    }

    pub fn new(kill_sender: Sender<()>) -> ShutdownHandle {
        ShutdownHandle {
            kill_sender: Some(kill_sender),
        }
    }
}
