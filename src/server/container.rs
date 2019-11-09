use crate::cmds::http::{HttpServiceId, HttpServiceInfo};
use crate::utils::http::*;
use hyper::{Body, Request, Response, StatusCode};
use std::collections::HashMap;

pub struct ServiceContainer {
    service_infos: HashMap<HttpServiceId, HttpServiceInfo>,
}

impl ServiceContainer {
    pub fn new() -> ServiceContainer {
        ServiceContainer {
            service_infos: HashMap::new(),
        }
    }

    pub fn add_service(&mut self, service_info: HttpServiceInfo) -> () {
        self.service_infos.insert(service_info.id(), service_info);
    }

    pub async fn process(&mut self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let id = HttpServiceId::from(&req);
        match self.service_infos.get(&id) {
            Some(service) => self.create_response(service),
            _ => not_found(),
        }
    }

    fn create_response(&self, service: &HttpServiceInfo) -> Result<Response<Body>, hyper::Error> {
        let mut b = Response::builder();
        for (k, v) in service.headers.iter() {
            b.header(k.trim(), v.as_ref().map_or("", String::as_ref).trim());
        }
        Ok(b.status(StatusCode::OK).body(self.body(service)).unwrap())
    }

    fn body(&self, service: &HttpServiceInfo) -> Body {
        let response = service
            .response
            .as_ref()
            .map_or("", String::as_str)
            .to_owned();
        Body::from(response)
    }
}
