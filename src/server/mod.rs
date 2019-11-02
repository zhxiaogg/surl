use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Response, Server as HyperServer};
use tokio;
use tokio::runtime::Runtime;
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

    pub fn start_foreground(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (killSender, mut killReceiver) = tokio::sync::oneshot::channel::<()>();
        let (killedSender, mut killedReceiver) = tokio::sync::oneshot::channel::<()>();

        let addr = ([127, 0, 0, 1], self.port).into();

        let make_service = make_service_fn(|_| {
            async {
                Ok::<_, Error>(service_fn(|_req| {
                    async { Ok::<_, Error>(Response::new(Body::from("Hello World"))) }
                }))
            }
        });

        let server = HyperServer::bind(&addr).serve(make_service);

        let server = server.with_graceful_shutdown(async {
            killReceiver.await.ok();
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
