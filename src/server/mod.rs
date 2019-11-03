use futures::lock::Mutex;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Request, Response, Server as HyperServer};
use std::sync::Arc;
use tokio;
use tokio::runtime::Runtime;

mod container;
mod internal;
use container::ServiceContainer;
use internal::{InternalService, ShutdownHandle};
use std::time::Duration;

pub struct HttpServer {
    address: String,
    port: u16,
}

impl HttpServer {
    pub fn new(address: &str, port: u16) -> HttpServer {
        HttpServer {
            address: address.to_owned(),
            port: port,
        }
    }

    pub async fn handler(
        req: Request<Body>,
        internal_service: Arc<Mutex<InternalService>>,
        service_container: Arc<Mutex<ServiceContainer>>,
    ) -> Result<Response<Body>, hyper::Error> {
        match (req.method(), req.uri().path()) {
            (_, s) if s.starts_with("/s/u/r/l") => {
                let mut internal_service = internal_service.lock().await;
                internal_service.process(req).await
            }
            _ => {
                let mut service_container = service_container.lock().await;
                service_container.process(req).await
            }
        }
    }

    pub fn start_foreground(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (kill_sender, mut kill_receiver) = tokio::sync::oneshot::channel::<()>();

        let addr = ([127, 0, 0, 1], self.port).into();

        let shutdown_handle = ShutdownHandle::new(kill_sender);
        let service_container = Arc::new(Mutex::new(ServiceContainer::new()));
        let internal_service = Arc::new(Mutex::new(InternalService::new(
            shutdown_handle,
            service_container.clone(),
        )));

        let make_service = make_service_fn(move |_| {
            let internal_service = internal_service.clone();
            let service_container = service_container.clone();
            async move {
                Ok::<_, Error>(service_fn(move |req| {
                    let internal_service = internal_service.clone();
                    let service_container = service_container.clone();
                    async move { HttpServer::handler(req, internal_service, service_container).await }
                }))
            }
        });

        let server = HyperServer::bind(&addr).serve(make_service);

        let server = server.with_graceful_shutdown(async {
            kill_receiver.await.ok();
            // sleep 1 second, so that the shutdown request can be responsed timely
            tokio::timer::delay_for(Duration::from_millis(1000)).await
        });

        // Create the runtime
        let rt = Runtime::new()?;

        rt.block_on(async {
            if let Err(e) = server.await {
                eprintln!("server error: {}", e);
            }
        });

        rt.shutdown_now();

        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub fn address(&self) -> String {
        self.address.to_owned()
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
