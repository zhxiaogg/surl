pub trait Server {}

pub struct HttpServer {}

impl HttpServer {
    pub fn new() -> HttpServer {
        HttpServer {}
    }
}

impl Server for HttpServer {}
